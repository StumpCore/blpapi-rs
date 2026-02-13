use crate::{
    correlation_id::CorrelationId,
    data_series::{DataSeries, DataSeriesBuilder},
    message::Message,
    message_iterator::MessageIterator,
    names::{
        SERVICE_DOWN, SERVICE_OPEN_FAILURE, SERVICE_REGISTER_FAILURE, SESSION_STARTUP_FAILURE,
        SESSION_TERMINATED, SUBSCRIPTION_FAILURE, SUBSCRIPTION_TERMINATED,
    },
    session::{Session, SubscriptionStatus},
    subscription_list::SubscriptionRegistry,
    Error, RefData,
};
use blpapi_sys::*;
use std::{
    collections::{HashSet, VecDeque},
    marker::PhantomData,
    os::raw::c_int,
    ptr,
};

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
pub enum SubscriptionMsg<R> {
    Data {
        ticker: String,
        data: DataSeries<R>,
    },
    StatusChange {
        ticker: String,
        status: SubscriptionStatus,
    },
    Terminated,
}

// SubscriptionStream
pub struct SubscriptionStream<R> {
    session_ptr: *mut blpapi_Session_t,
    registry: SubscriptionRegistry,
    subscription_status: SubscriptionStatus,
    message_buffer: VecDeque<SubscriptionMsg<R>>,
    exit: bool,
    _marker: PhantomData<R>,
}

unsafe impl<R> Send for SubscriptionStream<R> {}

impl<R> SubscriptionStream<R>
where
    R: RefData,
{
    pub fn new(ptr: *mut blpapi_Session_t, registry: SubscriptionRegistry) -> Self {
        let subscription_status = SubscriptionStatus::Subscribing;
        let vec_d: VecDeque<SubscriptionMsg<R>> = VecDeque::new();
        SubscriptionStream {
            session_ptr: ptr,
            registry,
            subscription_status,
            message_buffer: vec_d,
            exit: false,
            _marker: PhantomData,
        }
    }

    fn process_subscription_message(
        &self,
        message: &Message,
        ticker: String,
        requested_fields: &HashSet<String>,
    ) -> Option<DataSeries<R>> {
        let ele = message.element();
        let len = ele.num_elements();
        let mut data_builder: DataSeriesBuilder<R> = DataSeriesBuilder::with_capacity(len, ticker);
        for field in ele.elements() {
            let mut value = R::default();
            let name = field.string_name();
            if requested_fields.contains(&name) {
                value.on_field(&field.string_name(), &field);
                data_builder.values.push(value);
            }
        }
        let mut data_rows = data_builder.to_rows();
        if !data_rows.is_empty() {
            let f_item = data_rows.remove(0);
            Some(f_item)
        } else {
            None
        }
    }

    pub fn process_raw_event(
        &mut self,
        msg: Message,
        event_type: EventType,
    ) -> Option<SubscriptionMsg<R>> {
        let cid = msg.correlation_id(0)?.value;
        let reg = self.registry.lock().unwrap();
        let info = reg.get(&cid)?;

        match event_type {
            EventType::SubscriptionData | EventType::PartialResponse => {
                let ticker = info.ticker.clone();
                self.subscription_status = SubscriptionStatus::Subscribed;
                let data = self.process_subscription_message(&msg, ticker, &info.requested_fields);
                if let Some(data) = data {
                    let ticker = info.ticker.clone();
                    Some(SubscriptionMsg::Data { ticker, data })
                } else {
                    None
                }
            }

            EventType::SubscriptionStatus => {
                let ticker = info.ticker.clone();
                let m_type = msg.message_type();

                if m_type == *SUBSCRIPTION_FAILURE || m_type == *SUBSCRIPTION_TERMINATED {
                    Some(SubscriptionMsg::StatusChange {
                        ticker,
                        status: SubscriptionStatus::Cancelled,
                    })
                } else {
                    None
                }
            }

            EventType::SessionStatus => {
                let m_type = msg.message_type();
                if m_type == *SESSION_TERMINATED {
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

impl<R> Iterator for SubscriptionStream<R>
where
    R: RefData,
{
    type Item = Result<SubscriptionMsg<R>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(msg) = self.message_buffer.pop_front() {
            return Some(Ok(msg));
        }

        loop {
            if self.exit {
                return None;
            }
            let mut event_ptr = std::ptr::null_mut();
            let res = unsafe { blpapi_Session_nextEvent(self.session_ptr, &mut event_ptr, 0) };

            if res != 0 {
                return Some(Err(Error::InternalError));
            }

            let event = EventBuilder::default().ptr(event_ptr).build();
            let event_type = event.event_type;

            for msg in event.messages() {
                if let Some(msg) = self.process_raw_event(msg, event_type) {
                    self.message_buffer.push_back(msg);
                }
            }
            if let Some(msg) = self.message_buffer.pop_front() {
                return Some(Ok(msg));
            }
        }
    }
}
