use crate::core::{write_to_stream_cb, StreamWriterContext};
use crate::correlation_id::{CorrelationIdBuilder, ValueType};
use crate::datetime::{HighPrecisionDateTime, HighPrecisionDateTimeBuilder, TimePointBuilder};
use crate::name::NameBuilder;
use crate::{correlation_id, Error};
use crate::{correlation_id::CorrelationId, element::Element, event::Event, name::Name};
use blpapi_sys::*;
use std::collections::HashMap;
use std::ffi::{c_char, c_void, CStr};
use std::io::Write;
use std::marker::PhantomData;
use std::ptr;

/// Fragment Message Indicator
#[derive(Debug, Default)]
pub enum FragmentMessage {
    Start,
    Intermediate,
    End,
    #[default]
    None,
}

impl From<u32> for FragmentMessage {
    fn from(v: u32) -> Self {
        match v {
            BLPAPI_MESSAGE_FRAGMENT_END => FragmentMessage::End,
            BLPAPI_MESSAGE_FRAGMENT_INTERMEDIATE => FragmentMessage::Intermediate,
            BLPAPI_MESSAGE_FRAGMENT_NONE => FragmentMessage::None,
            BLPAPI_MESSAGE_FRAGMENT_START => FragmentMessage::Start,
            _ => FragmentMessage::default(),
        }
    }
}

/// Recap Message Indicator
#[derive(Debug, Default)]
pub enum RecapMessage {
    Solicited,
    Unsolicited,
    #[default]
    None,
}

impl From<u32> for RecapMessage {
    fn from(v: u32) -> Self {
        match v {
            BLPAPI_MESSAGE_RECAPTYPE_UNSOLICITED => RecapMessage::Unsolicited,
            BLPAPI_MESSAGE_RECAPTYPE_SOLICITED => RecapMessage::Solicited,
            _ => RecapMessage::None,
        }
    }
}

/// MktDataEventType
#[derive(Debug, Default)]
pub enum MktDataEventType {
    Summary,
    Trade,
    Quote,
    MarketDepth,
    #[default]
    Unkown,
}

/// MktDataEventSubtype
#[derive(Debug, Default)]
pub enum MktDataEventSubtype {
    NewDay,
    InitPaint,
    IntraDay,
    Interval,
    Dataloss,
    New,
    Cancel,
    Correction,
    Bid,
    Ask,
    Mid,
    Paired,
    Table,
    #[default]
    Unkown,
}

/// Core Message Type
#[derive(Debug, Default)]
pub enum MessageTypeCore {
    AuthorizationSuccess,
    AuthorizationFailure,
    AuthorizationRevoked,
    AuthorizationRequest,
    AuthorizationUpdate,
    ResponseError,
    FieldResponse,
    FieldResponseFieldDataField,
    FieldResponseFieldSearchError,
    FieldExceptions,
    FieldSearchError,
    CategorizedFieldReponse,
    CategorizedFieldSearchError,
    EntitlementChanged,
    SlowConsumerWarning,
    SlowConsumerWarningCleared,
    DataLosee,
    RequestFailure,
    RequestTemplateAvailable,
    RequestTemplatePending,
    RequestTemplateTerminated,
    ReferenceDataResponse,
    HistoricalDataResponse,
    MarketDataEvents,
    SubscriptionTerminated,
    SubscriptionStarted,
    SubscriptionStreamsActivated,
    SubscriptionStreamsDeactivated,
    SubscriptionFailure,
    SessionStarted,
    SessionTerminated,
    SessionStartupFailure,
    SessionConnectionDown,
    ServiceOpened,
    ServiceOpenFailure,
    SessionClusterInfo,
    SessionClusterUpdate,
    TokenGenerationSuccess,
    TokenGenerationFailure,
    #[default]
    SessionConnectionUp,
}

impl From<Name> for MessageTypeCore {
    fn from(v: Name) -> Self {
        match v.name.as_str() {
            "AuthorizationSuccess" => MessageTypeCore::AuthorizationSuccess,
            "AuthorizationFailure" => MessageTypeCore::AuthorizationFailure,
            "AuthorizationRevoked" => MessageTypeCore::AuthorizationRevoked,
            "AuthorizationRequest" => MessageTypeCore::AuthorizationRequest,
            "AuthorizationUpdate" => MessageTypeCore::AuthorizationUpdate,
            "ResponseError" => MessageTypeCore::ResponseError,
            "FieldResponse" => MessageTypeCore::FieldResponse,
            "FieldResponseFieldDataField" => MessageTypeCore::FieldResponseFieldDataField,
            "FieldResponseFieldSearchError" => MessageTypeCore::FieldResponseFieldSearchError,
            "FieldExceptions" => MessageTypeCore::FieldExceptions,
            "FieldSearchError" => MessageTypeCore::FieldSearchError,
            "CategorizedFieldReponse" => MessageTypeCore::CategorizedFieldReponse,
            "CategorizedFieldSearchError" => MessageTypeCore::CategorizedFieldSearchError,
            "EntitlementChanged" => MessageTypeCore::EntitlementChanged,
            "SlowConsumerWarning" => MessageTypeCore::SlowConsumerWarning,
            "SlowConsumerWarningCleared" => MessageTypeCore::SlowConsumerWarningCleared,
            "DataLosee" => MessageTypeCore::DataLosee,
            "RequestFailure" => MessageTypeCore::RequestFailure,
            "RequestTemplateAvailable" => MessageTypeCore::RequestTemplateAvailable,
            "RequestTemplatePending" => MessageTypeCore::RequestTemplatePending,
            "RequestTemplateTerminated" => MessageTypeCore::RequestTemplateTerminated,
            "ReferenceDataResponse" => MessageTypeCore::ReferenceDataResponse,
            "HistoricalDataResponse" => MessageTypeCore::HistoricalDataResponse,
            "MarketDataEvents" => MessageTypeCore::MarketDataEvents,
            "SubscriptionTerminated" => MessageTypeCore::SubscriptionTerminated,
            "SubscriptionStarted" => MessageTypeCore::SubscriptionStarted,
            "SubscriptionStreamsActivated" => MessageTypeCore::SubscriptionStreamsActivated,
            "SubscriptionStreamsDeactivated" => MessageTypeCore::SubscriptionStreamsDeactivated,
            "SubscriptionFailure" => MessageTypeCore::SubscriptionFailure,
            "SessionStarted" => MessageTypeCore::SessionStarted,
            "SessionTerminated" => MessageTypeCore::SessionTerminated,
            "SessionStartupFailure" => MessageTypeCore::SessionStartupFailure,
            "SessionConnectionDown" => MessageTypeCore::SessionConnectionDown,
            "ServiceOpened" => MessageTypeCore::ServiceOpened,
            "ServiceOpenFailure" => MessageTypeCore::ServiceOpenFailure,
            "SessionClusterInfo" => MessageTypeCore::SessionClusterInfo,
            "SessionClusterUpdate" => MessageTypeCore::SessionClusterUpdate,
            "TokenGenerationSuccess" => MessageTypeCore::TokenGenerationSuccess,
            "TokenGenerationFailure" => MessageTypeCore::TokenGenerationFailure,
            &_ => MessageTypeCore::default(),
        }
    }
}

/// Message Types
#[derive(Debug, Default)]
pub struct MessageType {
    pub message_type: MessageTypeCore,
    pub market_data_type: MktDataEventType,
    pub market_data_subtype: MktDataEventSubtype,
}

/// A message Builder
pub struct MessageBuilder<'a> {
    pub(crate) ptr: *mut blpapi_Message_t,
    pub(crate) elements: *mut blpapi_Element_t,
    pub request_id: Option<String>,
    pub message_type: Option<MessageType>,
    pub fragment: Option<FragmentMessage>,
    pub recap: Option<RecapMessage>,
    pub time_received: Option<HighPrecisionDateTime>,
    pub private_data: Option<String>,
    pub correlation_ids: Option<HashMap<usize, CorrelationId>>,
    pub num_of_correlation_ids: Option<usize>,
    _marker: PhantomData<&'a Event>,
}

/// Default trait of the MessageBuilder
impl<'a> Default for MessageBuilder<'a> {
    fn default() -> Self {
        let ptr: *mut blpapi_Message_t = ptr::null_mut();
        let elements: *mut blpapi_Element_t = ptr::null_mut();
        Self {
            ptr,
            elements,
            request_id: None,
            message_type: Some(MessageType::default()),
            fragment: Some(FragmentMessage::default()),
            recap: Some(RecapMessage::default()),
            time_received: None,
            private_data: None,
            correlation_ids: None,
            num_of_correlation_ids: None,
            _marker: PhantomData,
        }
    }
}

impl<'a> MessageBuilder<'a> {
    /// Creating a new Builder
    pub fn new() -> Self {
        Self::default()
    }
    /// Adding new message ptr to the builder
    pub fn ptr(mut self, ptr: *mut blpapi_Message_t) -> Self {
        self.ptr = ptr;
        self
    }

    /// Adding new elements ptr to the builder
    pub fn elements(mut self, ptr: *mut blpapi_Element_t) -> Self {
        self.elements = ptr;
        self
    }

    /// Adding Message Type
    fn msg_type(&mut self) -> Result<(), Error> {
        let msg_type_ptr = unsafe { blpapi_Message_messageType(self.ptr as *const _) };
        let msg_type_name = NameBuilder::default().by_ptr(msg_type_ptr).build();
        dbg!(&msg_type_name);
        let msg_type = MessageType {
            message_type: msg_type_name.into(),
            market_data_type: MktDataEventType::default(),
            market_data_subtype: MktDataEventSubtype::default(),
        };
        self.message_type = Some(msg_type);
        Ok(())
    }

    /// Adding the Fragment Type
    fn fragment_type(&mut self) -> Result<(), Error> {
        let fr_type_no = unsafe { blpapi_Message_fragmentType(self.ptr as *const _) } as u32;
        dbg!(&fr_type_no);
        self.fragment = Some(fr_type_no.into());
        Ok(())
    }

    /// Adding the Recap Type
    fn recap_type(&mut self) -> Result<(), Error> {
        let recap_type = unsafe { blpapi_Message_recapType(self.ptr as *const _) } as u32;
        dbg!(&recap_type);
        self.recap = Some(recap_type.into());
        Ok(())
    }

    /// Time Received
    fn time_received(&mut self) -> Result<(), Error> {
        let new_tp = TimePointBuilder::new().build();
        let point_ptr: *mut blpapi_TimePoint_t = Box::into_raw(Box::new(new_tp.point));
        let time_no = unsafe { blpapi_Message_timeReceived(self.ptr as *const _, point_ptr) };
        if time_no == 0 {
            let hp_dt = HighPrecisionDateTimeBuilder::new().build();
            let offset = 0;
            let mut time_point = TimePointBuilder::new().build();
            time_point.from_ptr(point_ptr)?;
            let hp_dt = hp_dt.get_from_time_point(time_point, offset);
            self.time_received = Some(hp_dt);
        }
        Ok(())
    }

    /// Private Data
    fn private_data(&mut self) -> Result<(), Error> {
        let mut size: usize = 256;
        let private_data = unsafe {
            let name = blpapi_Message_privateData(self.ptr as *const _, &mut size as *mut usize);
            if !name.is_null() {
                Some(CStr::from_ptr(name).to_string_lossy().into_owned())
            } else {
                None
            }
        };
        self.private_data = private_data;
        Ok(())
    }

    /// Request ids
    fn request_id(&mut self) -> Result<(), Error> {
        let mut request_ptr: *const c_char = ptr::null();
        let res = unsafe { blpapi_Message_getRequestId(self.ptr, &mut request_ptr) };
        if res != 0 {
            return Err(Error::struct_error(
                "RequestId",
                "request_id",
                "Error when trying to receive Request Id",
            ));
        };
        self.request_id = if !request_ptr.is_null() {
            let req_id = unsafe { CStr::from_ptr(request_ptr).to_string_lossy().into_owned() };
            Some(req_id)
        } else {
            None
        };
        Ok(())
    }

    /// Correlation Ids
    fn correlation_ids(&mut self) -> Result<(), Error> {
        // Get number of active correlation ids
        let cor_id_no = unsafe { blpapi_Message_numCorrelationIds(self.ptr) };
        let mut new_hash = HashMap::new();

        for index in 0..cor_id_no {
            let id = unsafe { blpapi_Message_correlationId(self.ptr, index as usize) };
            let correlation_id = CorrelationIdBuilder::new().from_pointer(id);
            dbg!(&correlation_id);
            new_hash.insert(index as usize, correlation_id);
        }
        self.correlation_ids = Some(new_hash);

        Ok(())
    }

    pub fn build(mut self) -> Message<'a> {
        let _msg_type = self.msg_type();
        let _frg_type = self.fragment_type();
        let _recap = self.recap_type();
        let _time_rec = self.time_received();
        let _private_data = self.private_data();
        let _request_id = self.request_id();
        let _cor_ids = self.correlation_ids();
        Message {
            ptr: self.ptr,
            _phantom: PhantomData,
            elements: self.elements,
            request_id: self.request_id.unwrap_or_default(),
            message_type: self.message_type.unwrap_or_default(),
            fragment_type: self.fragment.unwrap_or_default(),
            recap_type: self.recap.unwrap_or_default(),
            time_received: self.time_received.unwrap_or_default(),
            private_data: self.private_data.unwrap_or_default(),
            correlation_ids: self.correlation_ids.unwrap_or_default(),
        }
    }
}

/// A message
#[derive(Debug)]
pub struct Message<'a> {
    pub(crate) ptr: *mut blpapi_Message_t,
    pub(crate) _phantom: PhantomData<&'a Event>,
    pub(crate) elements: *mut blpapi_Element_t,
    pub request_id: String,
    pub message_type: MessageType,
    pub fragment_type: FragmentMessage,
    pub recap_type: RecapMessage,
    pub time_received: HighPrecisionDateTime,
    pub private_data: String,
    pub correlation_ids: HashMap<usize, CorrelationId>,
}

impl<'a> Message<'a> {
    /// Get topic name
    pub fn topic_name(&self) -> String {
        unsafe {
            let name = blpapi_Message_topicName(self.ptr);
            CStr::from_ptr(name).to_string_lossy().into_owned()
        }
    }

    /// Get type string
    pub fn type_string(&self) -> String {
        unsafe {
            let name = blpapi_Message_typeString(self.ptr);
            CStr::from_ptr(name).to_string_lossy().into_owned()
        }
    }

    /// Get message type
    pub fn message_type(&self) -> Name {
        unsafe {
            let ptr = blpapi_Message_messageType(self.ptr);
            NameBuilder::default().by_ptr(ptr).build()
        }
    }

    /// Get number of correlation ids
    pub fn num_correlation_ids(&self) -> usize {
        unsafe { blpapi_Message_numCorrelationIds(self.ptr) as usize }
    }

    /// Get correlation id
    pub fn correlation_id(&self, index: usize) -> Option<CorrelationId> {
        if index > self.num_correlation_ids() {
            None
        } else {
            unsafe {
                let ptr = blpapi_Message_correlationId(self.ptr, index);
                let cor_id = CorrelationIdBuilder::new().from_pointer(ptr);
                Some(cor_id)
            }
        }
    }

    /// Get corresponding element
    pub fn element(&self) -> Element {
        Element {
            ptr: self.elements,
            ..Default::default()
        }
    }

    /// Implementing the writer function to return the details about the SessionOptions
    pub fn print<T: Write>(&self, writer: &mut T, indent: i32, spaces: i32) -> Result<(), Error> {
        let mut context = StreamWriterContext { writer };
        unsafe {
            let res = blpapi_Message_print(
                self.ptr,
                Some(write_to_stream_cb),
                &mut context as *mut _ as *mut c_void,
                indent as std::ffi::c_int,
                spaces as std::ffi::c_int,
            );
            if res != 0 {
                return Err(Error::struct_error(
                    "Message",
                    "print",
                    "Error when trying to write to stream writer",
                ));
            };
        };
        Ok(())
    }
}

impl<'a> Drop for Message<'a> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let _ = blpapi_Message_release(self.ptr);
            }
        }
    }
}
