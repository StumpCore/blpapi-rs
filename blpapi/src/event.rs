use crate::{message_iterator::MessageIterator, name, session::Session, Error};
use blpapi_sys::*;
use std::{os::raw::c_int, ptr};

/// Event Builder
#[derive(Debug, Default)]
pub struct EventBuilder {
    pub ptr: Option<*mut blpapi_Event_t>,
    pub event_type: Option<EventType>,
    pub event_refs: Option<Vec<Event>>,
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

    /// Adding new Events
    pub fn new_event_refs(mut self, event_arr: Vec<Event>) -> Self {
        self.event_refs = Some(event_arr);
        self
    }

    /// Add one additional ref
    pub fn add_ref(mut self, event: Event) -> Self {
        if let Some(ele_vec) = &mut self.event_refs {
            ele_vec.push(event);
        }
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

        let mut new_event = Event { ptr, event_type };
        new_event.event_type();

        // Adding event references
        if let Some(ele_vec) = self.event_refs {
            for element in ele_vec {
                new_event.add_ref(element);
            }
        }
        new_event
    }
}

/// An event
#[derive(Debug)]
pub struct Event {
    pub(crate) ptr: *mut blpapi_Event_t,
    pub event_type: EventType,
}

impl Event {
    /// Get event type
    pub fn event_type(&mut self) -> EventType {
        self.event_type = unsafe { blpapi_Event_eventType(self.ptr).into() };
        self.event_type
    }

    /// Add reference to event
    pub fn add_ref(&mut self, event: Event) -> i64 {
        unsafe { blpapi_Event_addRef(event.ptr as *const _) as i64 }
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
        Self { ptr, event_type }
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
        let _res = unsafe { blpapi_Event_release(self.ptr as *const _) };
    }
}

/// New Interim Events Iterator
pub struct SessionEvents<'a> {
    session: &'a mut Session,
    exit: bool,
}

impl<'a> SessionEvents<'a> {
    pub fn new(session: &'a mut Session) -> Self {
        SessionEvents {
            session,
            exit: false,
        }
    }
    fn try_next(&mut self) -> Result<Option<Event>, Error> {
        if self.exit {
            return Ok(None);
        }
        loop {
            let event = self.session.next_event()?;
            let event_type = event.event_type;
            match event_type {
                EventType::PartialResponse => return Ok(Some(event)),
                EventType::Response => {
                    self.exit = true;
                    return Ok(Some(event));
                }
                EventType::SessionStatus => {
                    if event.messages().map(|m| m.message_type()).any(|m| {
                        m == *name::SESSION_TERMINATED || m == *name::SESSION_STARTUP_FAILURE
                    }) {
                        return Ok(None);
                    }
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
