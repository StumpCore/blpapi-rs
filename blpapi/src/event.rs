use crate::{
    correlation_id::CorrelationId,
    message_iterator::MessageIterator,
    names::{
        SERVICE_DOWN, SERVICE_OPEN_FAILURE, SERVICE_REGISTER_FAILURE, SESSION_STARTUP_FAILURE,
        SESSION_TERMINATED, SUBSCRIPTION_FAILURE, SUBSCRIPTION_TERMINATED,
    },
    session::{Session, SubscriptionStatus},
    Error,
};
use blpapi_sys::*;
use std::{collections::HashMap, os::raw::c_int, ptr};

/// Event Builder
#[derive(Debug, Default)]
pub struct EventBuilder {
    pub ptr: Option<*mut blpapi_Event_t>,
    pub event_type: Option<EventType>,
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

impl Clone for Event {
    fn clone(&self) -> Self {
        unsafe {
            blpapi_Event_addRef(self.ptr);
        }
        Self {
            ptr: self.ptr,
            event_type: self.event_type,
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
    event_queue: EventQueue,
}

impl<'a> SessionEvents<'a> {
    pub fn new(
        session: &'a mut Session,
        correlation_id: CorrelationId,
        event_queue: EventQueue,
    ) -> Self {
        SessionEvents {
            session,
            correlation_id,
            exit: false,
            event_queue,
        }
    }
    fn try_next(&mut self) -> Result<Option<Event>, Error> {
        if self.exit {
            return Ok(None);
        }
        loop {
            let event = match self.session.event_queue {
                true => self.event_queue.next_event()?,
                false => self.session.next_event()?,
            };
            let event_type = event.event_type;
            match event_type {
                EventType::SessionStatus => {
                    if event
                        .messages()
                        .map(|m| m.message_type())
                        .any(|m| m == *SESSION_TERMINATED || m == *SESSION_STARTUP_FAILURE)
                    {
                        return Ok(None);
                    }
                }
                EventType::ServiceStatus => {
                    if event.messages().map(|m| m.message_type()).any(|m| {
                        m == *SERVICE_DOWN
                            || m == *SERVICE_OPEN_FAILURE
                            || m == *SERVICE_REGISTER_FAILURE
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

/// Create an eventQueue
pub struct EventQueue {
    pub(crate) ptr: *mut blpapi_EventQueue_t,
    pub timeout: i32,
    pub exit: bool,
}

impl EventQueue {
    // Creating new EventQueue
    pub fn new(timeout: i32) -> EventQueue {
        let ptr = unsafe { blpapi_EventQueue_create() };
        EventQueue {
            ptr,
            timeout,
            exit: false,
        }
    }

    // Setting new timeout
    pub fn timeout(&mut self, timeout: i32) -> &mut Self {
        self.timeout = timeout;
        self
    }

    // Getting the next event
    pub fn next_event(&mut self) -> Result<Event, Error> {
        unsafe {
            let res = blpapi_EventQueue_nextEvent(self.ptr, self.timeout);
            let event = EventBuilder::default().ptr(res).build();
            Ok(event)
        }
    }

    // Try getting the next event
    pub fn try_next_event(&mut self) -> Option<Event> {
        let mut next_event_ptr: *mut blpapi_Event_t = ptr::null_mut();
        unsafe {
            let res = blpapi_EventQueue_tryNextEvent(self.ptr, &mut next_event_ptr);
            match res == 0 {
                true => Some(EventBuilder::default().ptr(next_event_ptr).build()),
                false => None,
            }
        }
    }

    // Getting the next event
    pub fn purge(&mut self) -> Result<(), Error> {
        unsafe {
            let res = blpapi_EventQueue_purge(self.ptr);
            Error::check(res)?;
            Ok(())
        }
    }

    fn match_event_type(
        &mut self,
        event_type: &EventType,
        event: Event,
    ) -> Result<Option<Event>, Error> {
        match event_type {
            EventType::SessionStatus => {
                if event
                    .messages()
                    .map(|m| m.message_type())
                    .any(|m| m == *SESSION_TERMINATED || m == *SESSION_STARTUP_FAILURE)
                {
                    Err(Error::struct_error(
                        "Event",
                        "match_event_type",
                        "Session Terminated/StartUp Failure",
                    ))
                } else {
                    Ok(None)
                }
            }
            EventType::ServiceStatus => {
                if event.messages().map(|m| m.message_type()).any(|m| {
                    m == *SERVICE_DOWN
                        || m == *SERVICE_OPEN_FAILURE
                        || m == *SERVICE_REGISTER_FAILURE
                }) {
                    Err(Error::struct_error(
                        "Event",
                        "match_event_type",
                        "Service Down/Register/Open Failure",
                    ))
                } else {
                    Ok(None)
                }
            }
            EventType::PartialResponse => Ok(Some(event)),
            EventType::Response => {
                self.exit = true;
                Ok(Some(event))
            }
            EventType::Timeout => Err(Error::TimeOut),
            _ => Ok(None),
        }
    }

    // Next Iterator for EventQueue
    fn next(&mut self) -> Result<Option<Event>, Error> {
        if self.exit {
            return Ok(None);
        }
        loop {
            let event = self.next_event()?;
            let event_type = event.event_type;
            match self.match_event_type(&event_type, event) {
                Ok(fin_event) => {
                    if let Some(ev) = fin_event {
                        return Ok(Some(ev));
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        let ptr = ptr::null_mut();
        let timeout = 0;
        let exit = false;
        EventQueue { ptr, timeout, exit }
    }
}

impl Drop for EventQueue {
    fn drop(&mut self) {
        let res = unsafe { blpapi_EventQueue_destroy(self.ptr) };
        let _ = Error::check(res);
    }
}

/// Iterator for EventQueue
impl Iterator for EventQueue {
    type Item = Result<Event, Error>;
    fn next(&mut self) -> Option<Result<Event, Error>> {
        self.next().transpose()
    }
}

/// Subscription messages
#[derive(Debug, Clone)]
pub enum SubscriptionMsg {
    Data {
        ticker: String,
        event: Event,
    },
    StatusChange {
        ticker: String,
        status: SubscriptionStatus,
    },
    Terminated,
}

/// New Subscription Events Iterator
pub struct SubscriptionEvents<'a> {
    session: &'a mut Session,
    registry: HashMap<u64, String>,
    subscription_status: SubscriptionStatus,
    exit: bool,
}

impl<'a> SubscriptionEvents<'a> {
    pub fn new(session: &'a mut Session, mapping: HashMap<u64, String>) -> Self {
        let subscription_status = SubscriptionStatus::Subscribing;
        SubscriptionEvents {
            session,
            exit: false,
            registry: mapping,
            subscription_status,
        }
    }

    fn process_raw_event(&mut self, event: Event) -> Option<SubscriptionMsg> {
        match event.event_type {
            EventType::SubscriptionData | EventType::PartialResponse => {
                // Get the ticker from the first message's CID
                let cid = event.messages().next()?.correlation_id(0);
                if let Some(cid) = cid {
                    let ticker = self.registry.get(&cid.value)?.clone();

                    self.subscription_status = SubscriptionStatus::Subscribed;
                    Some(SubscriptionMsg::Data { ticker, event })
                } else {
                    None
                }
            }

            EventType::SubscriptionStatus => {
                let msg = event.messages().next()?;
                let cid = msg.correlation_id(0);

                if let Some(cid) = cid {
                    let ticker = self.registry.get(&cid.value)?.clone();
                    let m_type = msg.message_type();

                    if m_type == *SUBSCRIPTION_FAILURE || m_type == *SUBSCRIPTION_TERMINATED {
                        Some(SubscriptionMsg::StatusChange {
                            ticker,
                            status: SubscriptionStatus::Cancelled,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            EventType::SessionStatus => {
                if event
                    .messages()
                    .any(|m| m.message_type() == *SESSION_TERMINATED)
                {
                    self.exit = true;
                    Some(SubscriptionMsg::Terminated)
                } else {
                    None
                }
            }

            _ => None,
        }
    }
}

impl<'a> Iterator for SubscriptionEvents<'a> {
    type Item = Result<SubscriptionMsg, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.exit {
            return None;
        }

        loop {
            match self.session.next_event() {
                Ok(raw_event) => {
                    if let Some(msg) = self.process_raw_event(raw_event) {
                        return Some(Ok(msg));
                    }
                }
                Err(Error::TimeOut) => {
                    self.exit = true;
                    return Some(Err(Error::TimeOut));
                }
                Err(e) => return Some(Err(e)),
            }
        }
    }
}
