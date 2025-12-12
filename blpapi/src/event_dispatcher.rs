use crate::Error;
use blpapi_sys::{
    blpapi_EventDispatcher_create, blpapi_EventDispatcher_destroy,
    blpapi_EventDispatcher_dispatchEvents, blpapi_EventDispatcher_start,
    blpapi_EventDispatcher_stop, blpapi_EventDispatcher_t,
};
use std::ffi::c_int;

pub struct EventDispatcherBuilder {
    pub num_dispatcher_threads: usize,
}

impl Default for EventDispatcherBuilder {
    fn default() -> Self {
        Self {
            num_dispatcher_threads: 1,
        }
    }
}

impl EventDispatcherBuilder {
    pub fn new(num_dispatcher_threads: usize) -> Self {
        Self {
            num_dispatcher_threads,
        }
    }

    pub fn build(self) -> EventDispatcher {
        let ptr = unsafe { blpapi_EventDispatcher_create(self.num_dispatcher_threads) };
        EventDispatcher { ptr }
    }
}

#[derive(Debug)]
pub struct EventDispatcher {
    pub(crate) ptr: *mut blpapi_EventDispatcher_t,
}

impl Drop for EventDispatcher {
    fn drop(&mut self) {
        unsafe {
            blpapi_EventDispatcher_destroy(self.ptr);
        }
    }
}

impl EventDispatcher {
    /// Starting the Event Dispatcher
    pub fn start(&self) -> Result<(), Error> {
        let res = unsafe { blpapi_EventDispatcher_start(self.ptr) } as i32;
        match res {
            0 => Ok(()),
            _ => Error::check(111),
        }
    }

    /// Stopping the Event Dispatcher
    pub fn stop(&self, async_val: &bool) -> Result<(), Error> {
        let async_ = match async_val {
            true => 1,
            false => 0,
        } as c_int;
        let res = unsafe { blpapi_EventDispatcher_stop(self.ptr, async_) } as i32;
        match res {
            0 => Ok(()),
            _ => Error::check(111),
        }
    }

    /// Dispatch Events
    pub fn dispatch_events(&self) -> Result<(), Error> {
        let res = unsafe { blpapi_EventDispatcher_dispatchEvents(self.ptr) } as i32;
        match res {
            0 => Ok(()),
            _ => Error::check(111),
        }
    }
}
