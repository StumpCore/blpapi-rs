use crate::{
    correlation_id::{self, CorrelationId},
    message_iterator::MessageIterator,
    name,
    session::Session,
    Error,
};
use blpapi_sys::*;
use std::{os::raw::c_int, ptr};

/// Event Builder
#[derive(Debug, Default)]
pub struct EventBuilder {
    pub ptr: Option<*mut blpapi_Event_t>,
    pub event_type: Option<EventType>,
    pub correlation_id: Option<CorrelationId>,
}

impl EventBuilder {
    /// Setting pointer
    pub fn ptr(mut self, ptr: *mut blpapi_Event_t) -> Self {
        self.ptr = Some(ptr);
        self
    }

    /// Setting event Type
    pub fn event_type(mut self, event_type: EventType) -> Self {
        self.event_type = Some(event_type);
        self
    }

    /// Setting the correlation id
    pub fn correlation_id(mut self, correlation_id: CorrelationId) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    /// Creating the Event
    pub fn build(self) -> Event {
        let ptr = match self.ptr {
            Some(p) => p,
            None => ptr::null_mut(),
        };

        let event_type = match self.event_type {
            Some(e) => e,
            None => EventType::Unknown,
        };

        let correlation_id = match self.correlation_id {
            Some(e) => e,
            None => CorrelationId::new_u64(0),
        };

        let mut new_event = Event {
            ptr,
            event_type,
            correlation_id,
        };

        new_event.event_type();

        new_event
    }
}

/// An event
#[derive(Debug)]
pub struct Event {
    pub(crate) ptr: *mut blpapi_Event_t,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
}

impl Event {
    /// Get event type
    pub fn event_type(&mut self) -> EventType {
        self.event_type = unsafe { blpapi_Event_eventType(self.ptr).into() };
        self.event_type
    }

    /// Get an iterator over all messages of this event
    pub fn messages(&self) -> MessageIterator<'_> {
        MessageIterator::new(self)
    }
}

impl Default for Event {
    fn default() -> Self {
        let ptr: *mut blpapi_Event_t = ptr::null_mut();
        let event_type = EventType::Unknown;
        let correlation_id = CorrelationId::new_u64(0);
        Self {
            ptr,
            event_type,
            correlation_id,
        }
    }
}

impl Clone for Event {
    fn clone(&self) -> Self {
        unsafe {
            blpapi_Event_addRef(self.ptr);
        }
        Self {
            ptr: self.ptr,
            event_type: self.event_type,
            correlation_id: self.correlation_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    Admin,
    SessionStatus,
    SubscriptionStatus,
    RequestStatus,
    Response,
    PartialResponse,
    SubscriptionData,
    ServiceStatus,
    Timeout,
    AuthorizationStatus,
    ResolutionStatus,
    TopicStatus,
    TokenStatus,
    Request,
    Unknown = -1,
}

impl From<c_int> for EventType {
    fn from(v: c_int) -> Self {
        match v as u32 {
            BLPAPI_EVENTTYPE_ADMIN => EventType::Admin,
            BLPAPI_EVENTTYPE_SESSION_STATUS => EventType::SessionStatus,
            BLPAPI_EVENTTYPE_SUBSCRIPTION_STATUS => EventType::SubscriptionStatus,
            BLPAPI_EVENTTYPE_REQUEST_STATUS => EventType::RequestStatus,
            BLPAPI_EVENTTYPE_RESPONSE => EventType::Response,
            BLPAPI_EVENTTYPE_PARTIAL_RESPONSE => EventType::PartialResponse,
            BLPAPI_EVENTTYPE_SUBSCRIPTION_DATA => EventType::SubscriptionData,
            BLPAPI_EVENTTYPE_SERVICE_STATUS => EventType::ServiceStatus,
            BLPAPI_EVENTTYPE_TIMEOUT => EventType::Timeout,
            BLPAPI_EVENTTYPE_AUTHORIZATION_STATUS => EventType::AuthorizationStatus,
            BLPAPI_EVENTTYPE_RESOLUTION_STATUS => EventType::ResolutionStatus,
            BLPAPI_EVENTTYPE_TOPIC_STATUS => EventType::TopicStatus,
            BLPAPI_EVENTTYPE_TOKEN_STATUS => EventType::TokenStatus,
            BLPAPI_EVENTTYPE_REQUEST => EventType::Request,
            _ => EventType::Unknown,
        }
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { blpapi_Event_release(self.ptr as *const _) };
        }
    }
}

/// New Interim Events Iterator
pub struct SessionEvents<'a> {
    session: &'a mut Session,
    exit: bool,
    correlation_id: CorrelationId,
}

impl<'a> SessionEvents<'a> {
    pub fn new(session: &'a mut Session, correlation_id: CorrelationId) -> Self {
        SessionEvents {
            session,
            correlation_id,
            exit: false,
        }
    }
    fn try_next(&mut self) -> Result<Option<Event>, Error> {
        if self.exit {
            return Ok(None);
        }
        loop {
            let mut event = self.session.next_event()?;
            event.correlation_id = self.correlation_id;
            dbg!(&event);
            let event_type = event.event_type;
            match event_type {
                EventType::SessionStatus => {
                    if event.messages().map(|m| m.message_type()).any(|m| {
                        m == *name::SESSION_TERMINATED || m == *name::SESSION_STARTUP_FAILURE
                    }) {
                        return Ok(None);
                    }
                }
                EventType::ServiceStatus => {
                    if event.messages().map(|m| m.message_type()).any(|m| {
                        m == *name::SERVICE_DOWN
                            || m == *name::SERVICE_OPEN_FAILURE
                            || m == *name::SERVICE_REGISTER_FAILURE
                    }) {
                        return Ok(None);
                    }
                }
                EventType::PartialResponse => return Ok(Some(event)),
                EventType::Response => {
                    self.exit = true;
                    return Ok(Some(event));
                }
                EventType::Timeout => return Err(Error::TimeOut),
                _ => (),
            }
        }
    }
}

impl<'a> Iterator for SessionEvents<'a> {
    type Item = Result<Event, Error>;
    fn next(&mut self) -> Option<Result<Event, Error>> {
        self.try_next().transpose()
    }
}
