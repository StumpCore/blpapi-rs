use crate::correlation_id::CorrelationId;
use crate::message::MessageBuilder;
use crate::{event::Event, message::Message};
use blpapi_sys::*;
use std::marker::PhantomData;
use std::ptr;

/// A message iterator
pub struct MessageIterator<'a> {
    pub(crate) ptr: *mut blpapi_MessageIterator_t,
    pub correlation_id: &'a CorrelationId,
    _phantom: PhantomData<&'a Event>,
}

impl<'a> MessageIterator<'a> {
    pub fn new(event: &'a Event) -> Self {
        unsafe {
            let ptr = blpapi_MessageIterator_create(event.ptr);
            MessageIterator {
                ptr,
                correlation_id: &event.correlation_id,
                _phantom: PhantomData,
            }
        }
    }
}

impl<'a> Drop for MessageIterator<'a> {
    fn drop(&mut self) {
        unsafe { blpapi_MessageIterator_destroy(self.ptr) }
    }
}

impl<'a> Clone for MessageIterator<'a> {
    fn clone(&self) -> Self {
        unsafe {
            blpapi_MessageIterator_addRef(self.ptr as *const _);
        }

        MessageIterator {
            ptr: self.ptr,
            correlation_id: self.correlation_id,
            _phantom: PhantomData,
        }
    }
}

impl<'a> Iterator for MessageIterator<'a> {
    type Item = Message<'a>;
    fn next(&mut self) -> Option<Message<'a>> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let res = blpapi_MessageIterator_next(self.ptr, &mut ptr as *mut _);
            if res == 0 {
                let elements = blpapi_Message_elements(ptr);
                let new_msg = MessageBuilder::new().elements(elements).ptr(ptr).build();
                if new_msg.correlation_id_by_id(self.correlation_id) {
                    Some(new_msg)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}
