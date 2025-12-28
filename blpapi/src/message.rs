use crate::name::NameBuilder;
use crate::Error;
use crate::{correlation_id::CorrelationId, element::Element, event::Event, name::Name};
use blpapi_sys::*;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::{default, ptr};

/// Fragment Message Indicator
#[derive(Debug, Default)]
enum FragmentMessage {
    Start,
    Intermediate,
    End,
    #[default]
    None,
}

/// MktDataEventType
#[derive(Debug, Default)]
enum MktDataEventType {
    Summary,
    Trade,
    Quote,
    MarketDepth,
    #[default]
    Unkown,
}

/// MktDataEventSubtype
#[derive(Debug, Default)]
enum MktDataEventSubtype {
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
enum MessageTypeCore {
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

impl From<Name> for MessageTypeCore{
    fn from(v: Name) -> Self {
        match v.name{
    "AuthorizationSuccess"=>MessageTypeCore::AuthorizationSuccess,
    "AuthorizationFailure"=>MessageTypeCore::AuthorizationFailure,
    "AuthorizationRevoked"=>MessageTypeCore::AuthorizationRevoked,
    "AuthorizationRequest"=>MessageTypeCore::AuthorizationRequest,
    "AuthorizationUpdate"=>MessageTypeCore::AuthorizationUpdate,
    "ResponseError"=>MessageTypeCore::ResponseError,
    "FieldResponse"=>MessageTypeCore::FieldResponse,
    "FieldResponseFieldDataField"=>MessageTypeCore::FieldResponseFieldDataField,
    "FieldResponseFieldSearchError"=>MessageTypeCore::FieldResponseFieldSearchError,
    "FieldExceptions"=>MessageTypeCore::FieldExceptions,
    "FieldSearchError"=>MessageTypeCore::FieldSearchError,
    "CategorizedFieldReponse"=>MessageTypeCore::CategorizedFieldReponse,
    "CategorizedFieldSearchError"=>MessageTypeCore::CategorizedFieldSearchError, 
   "EntitlementChanged"=>MessageTypeCore::EntitlementChanged, 
   "SlowConsumerWarning"=>MessageTypeCore::SlowConsumerWarning, 
    "SlowConsumerWarningCleared"=>MessageTypeCore::SlowConsumerWarningCleared,
    "DataLosee" => MessageTypeCore::DataLosee,
    "RequestFailure" => MessageTypeCore::RequestFailure,
    "RequestTemplateAvailable" => MessageTypeCore::RequestTemplateAvailable,
    "RequestTemplatePending" => MessageTypeCore::RequestTemplatePending,
    "RequestTemplateTerminated" => MessageTypeCore::RequestTemplateTerminated,
    "ReferenceDataResponse" => MessageTypeCore::ReferenceDataResponse,
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
    "ServiceOpened" => MessageTypeCore::"ServiceOpened" => MessageTypeCore::ServiceOpened,
    "ServiceOpenFailure" => MessageTypeCore::ServiceOpenFailure,
    "SessionClusterInfo" => MessageTypeCore::SessionClusterInfo,
    "SessionClusterUpdate" => MessageTypeCore::SessionClusterUpdate,
    "TokenGenerationSuccess" => MessageTypeCore::TokenGenerationSuccess,
    "TokenGenerationFailure" => MessageTypeCore::TokenGenerationFailure,
            _ => MessageTypeCore::default(),
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
    pub message_type: Option<MessageType>,
    pub fragment: Option<FragmentMessage>,
    _marker: PhantomData<&'a Event>,
}

/// Default trait of the MessageBuilder
impl<'a> Default for MessageBuilder<'a> {
    fn default() -> Self {
        let ptr: *mut blpapi_Message_t = ptr::null_mut();
        let elements: *mut blpapi_Element_t = ptr::null_mut();
        let message_type = MessageType {
            message_type: MessageTypeCore::default(),
            market_data_type: MktDataEventType::default(),
            market_data_subtype: MktDataEventSubtype::default(),
        };
        Self {
            ptr,
            elements,
            message_type: Some(message_type),
            fragment: Some(FragmentMessage::default()),
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
        let msg_type_name = unsafe { blpapi_Message_messageType(self.ptr as *const _) };
        let msg_type = NameBuilder::default().by_ptr(msg_type_name).build();
        dbg!(msg_type);
        Ok(())
    }

    pub fn build(mut self) -> Message<'a> {
        let _msg_type = self.msg_type();
        Message {
            ptr: self.ptr,
            _phantom: PhantomData,
            elements: self.elements,
            message_type: self.message_type.unwrap_or_default(),
        }
    }
}

/// A message
#[derive(Debug)]
pub struct Message<'a> {
    pub(crate) ptr: *mut blpapi_Message_t,
    pub(crate) _phantom: PhantomData<&'a Event>,
    pub(crate) elements: *mut blpapi_Element_t,
    pub message_type: MessageType,
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
                let mut ptr = blpapi_Message_correlationId(self.ptr, index);
                Some(CorrelationId {
                    id: &mut ptr,
                    value: 0,
                    value_type: 0,
                    reserved: 0,
                    class_id: 0,
                })
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
}

//TODO:
//check if we must release it.
//from the doc, it appears that messages are reference counted (when cloned) and
//release just decrease the refcount ...
//
//impl<'a> Drop for Message<'a> {
//    fn drop(&mut self) {
//        unsafe { let _ = blpapi_Message_release(self.ptr); }
//    }
//}

//pub enum RecapType {
//    None = BLPAPI_MESSAGE_RECAPTYPE_NONE,
//    Solicited = BLPAPI_MESSAGE_RECAPTYPE_SOLICITED,
//    Unsolicited = BLPAPI_MESSAGE_RECAPTYPE_UNSOLICITED }
//}
