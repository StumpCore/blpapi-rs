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

/// Message Types
pub struct MessageType {
    pub msg_type: MktDataEventType,
    pub msg_sub_type: MktDataEventSubtype,
}

/// A message Builder
pub struct MessageBuilder {
    pub(crate) ptr: *mut blpapi_Message_t,
    pub message_type: Option<MessageType>,
    pub fragment: Option<FragmentMessage>,
}

/// Default trait of the MessageBuilder
impl Default for MessageBuilder {
    fn default() -> Self {
        let ptr: *mut blpapi_Message_t = ptr::null_mut();
        let message_type = MessageType {
            msg_type: MktDataEventType::default(),
            msg_sub_type: MktDataEventSubtype::default(),
        };
        Self {
            ptr,
            message_type: Some(message_type),
            fragment: Some(FragmentMessage::default()),
        }
    }
}

impl MessageBuilder {
    /// Adding new message ptr to the builder
    pub fn ptr(mut self, ptr: *mut blpapi_Message_t) -> Self {
        self.ptr = ptr;
        self
    }

    /// Adding Message Type
    fn msg_type(&mut self) -> Result<(), Error> {
        let msg_type_name = unsafe { blpapi_Message_messageType(self.ptr as *const _) };
        let msg_type = NameBuilder::default().by_ptr(msg_type_name).build();
        dbg!(msg_type);
        Ok(())
    }

    pub fn build(self) -> Message {
        self.msg_type();
        todo!();
        Message { ptr: self.ptr }
    }
}

/// A message
pub struct Message<'a> {
    pub(crate) ptr: *mut blpapi_Message_t,
    pub(crate) _phantom: PhantomData<&'a Event>,
    pub(crate) elements: *mut blpapi_Element_t,
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
