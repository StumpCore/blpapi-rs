use crate::{
    abstract_session::AbstractSession,
    correlation_id::{CorrelationId, CorrelationIdBuilder},
    element::Element,
    event::{Event, EventBuilder, SessionEvents},
    event_dispatcher::{EventDispatcher, EventDispatcherBuilder},
    identity::{Identity, IdentityBuilder, SeatType},
    names::{
        FIELDS_NAME, FIELD_DATA, FIELD_ID, OVERRIDES, SECURITIES, SECURITY_DATA, SECURITY_ERROR,
        SECURITY_NAME, VALUE,
    },
    overrides::Override,
    ref_data::RefData,
    request::{Request, RequestTypes},
    service::{BlpServiceStatus, BlpServices, Service},
    session_options::SessionOptions,
    time_series::{HistOptions, TimeSerie},
    Error,
};
use blpapi_sys::*;
use std::collections::HashMap;
use std::{
    ffi::{c_void, CString},
    ptr,
};

const MAX_PENDING_REQUEST: usize = 1024;
const MAX_REFDATA_FIELDS: usize = 400;
const MAX_HISTDATA_FIELDS: usize = 25;

pub enum SubscriptionStatus {
    Unsubscribed,
    Subscribing,
    Subscribed,
    Cancelled,
    PendingCancellation,
    Unknown,
}

impl From<u32> for SubscriptionStatus {
    fn from(arg: u32) -> Self {
        match arg {
            BLPAPI_SUBSCRIPTIONSTATUS_UNSUBSCRIBED => SubscriptionStatus::Unsubscribed,
            BLPAPI_SUBSCRIPTIONSTATUS_SUBSCRIBING => SubscriptionStatus::Subscribing,
            BLPAPI_SUBSCRIPTIONSTATUS_SUBSCRIBED => SubscriptionStatus::Subscribed,
            BLPAPI_SUBSCRIPTIONSTATUS_CANCELLED => SubscriptionStatus::Cancelled,
            BLPAPI_SUBSCRIPTIONSTATUS_PENDING_CANCELLATION => {
                SubscriptionStatus::PendingCancellation
            }
            _ => SubscriptionStatus::Unknown,
        }
    }
}

impl From<SubscriptionStatus> for u32 {
    fn from(arg: SubscriptionStatus) -> Self {
        match arg {
            SubscriptionStatus::Unsubscribed => BLPAPI_SUBSCRIPTIONSTATUS_UNSUBSCRIBED,
            SubscriptionStatus::Subscribing => BLPAPI_SUBSCRIPTIONSTATUS_SUBSCRIBING,
            SubscriptionStatus::Subscribed => BLPAPI_SUBSCRIPTIONSTATUS_SUBSCRIBED,
            SubscriptionStatus::Cancelled => BLPAPI_SUBSCRIPTIONSTATUS_CANCELLED,
            SubscriptionStatus::PendingCancellation => {
                BLPAPI_SUBSCRIPTIONSTATUS_PENDING_CANCELLATION
            }
            SubscriptionStatus::Unknown => 113,
        }
    }
}

#[allow(non_snake_case)]
pub type EventHandler = Option<
    unsafe extern "C" fn(
        event: *mut blpapi_Event_t,
        session: *mut blpapi_Session_t,
        userData: *mut c_void,
    ),
>;

/// SessionBuilder Struct to create Session
#[derive(Default)]
pub struct SessionBuilder {
    pub options: Option<SessionOptions>,
    pub dispatcher: Option<EventDispatcher>,
    pub time_out: Option<u32>,
    pub handler: EventHandler,
}

impl SessionBuilder {
    pub fn options(mut self, options: SessionOptions) -> Self {
        self.options = Some(options);
        self
    }

    pub fn dispatcher(mut self, dispatcher: EventDispatcher) -> Self {
        self.dispatcher = Some(dispatcher);
        self
    }

    pub fn time_out(mut self, ms: u32) -> Self {
        self.time_out = Some(ms);
        self
    }

    pub fn handler(mut self, handler: EventHandler) -> Self {
        self.handler = handler;
        self
    }

    fn sync_session(self, options: SessionOptions) -> Session {
        let handler = None;
        let time_out = self.time_out.unwrap_or_default();
        let dispatcher = EventDispatcherBuilder::default().build();
        let user_data = ptr::null_mut();
        let ptr = unsafe { blpapi_Session_create(options.ptr, handler, dispatcher.ptr, user_data) };

        Session {
            ptr,
            options,
            dispatcher,
            async_: false,
            open_services: vec![],
            time_out,
            act_services: HashMap::new(),
            correlation_ids: vec![],
            correlation_count: 1,
        }
    }

    fn async_session(self, options: SessionOptions, handler: EventHandler) -> Session {
        let time_out = self.time_out.unwrap_or_default();
        let dispatcher = match self.dispatcher {
            Some(dp) => dp,
            None => EventDispatcherBuilder::default().build(),
        };
        let user_data = ptr::null_mut();
        let ptr = unsafe { blpapi_Session_create(options.ptr, handler, dispatcher.ptr, user_data) };

        Session {
            ptr,
            options,
            dispatcher,
            time_out,
            async_: true,
            open_services: vec![],
            act_services: HashMap::new(),
            correlation_ids: vec![],
            correlation_count: 1,
        }
    }

    pub fn build(self) -> Session {
        let opt = self.options.to_owned().unwrap_or_default();
        match self.handler {
            Some(handler) => self.async_session(opt, Some(handler)),
            None => self.sync_session(opt),
        }
    }
}

#[derive(Debug)]
pub struct Session {
    pub(crate) ptr: *mut blpapi_Session_t,
    pub options: SessionOptions,
    pub dispatcher: EventDispatcher,
    pub async_: bool,
    pub open_services: Vec<BlpServices>,
    pub act_services: HashMap<String, Service>,
    pub correlation_ids: Vec<CorrelationId>,
    pub correlation_count: u64,
    pub time_out: u32,
}

impl AbstractSession for Session {
    fn as_abstract_ptr(&self) -> *mut blpapi_AbstractSession_t {
        self.ptr as *mut blpapi_AbstractSession_t
    }

    /// Generating new correlation id
    fn new_correlation_id(&mut self) -> CorrelationId {
        let id = CorrelationIdBuilder::default()
            .set_value_type(crate::correlation_id::OwnValueType::IntValue(
                self.correlation_count,
            ))
            .build();
        self.correlation_ids.push(id);
        self.correlation_count += 1;
        id
    }

    /// Create Identity
    fn create_identity(&self) -> Result<Identity, Error> {
        let id = unsafe { blpapi_Session_createIdentity(self.ptr) };
        let mut id = IdentityBuilder::default()
            .ptr(id)
            .valid(true)
            .seat_type(SeatType::InvalidSeat)
            .build()?;
        id.get_seat_type()?;
        Ok(id)
    }
}

impl Session {
    /// Setting new timeout
    pub fn new_time_out(&mut self, ms: u32) -> Result<(), Error> {
        self.time_out = ms;
        Ok(())
    }

    /// Open service
    pub fn open_service(&mut self, service: &BlpServices) -> Result<&mut Self, Error> {
        let service_str: &str = service.into();
        let c_service = CString::new(service_str).unwrap_or_default();
        let res = match self.async_ {
            true => unsafe {
                let mut id = self.new_correlation_id();
                blpapi_Session_openServiceAsync(self.ptr, c_service.as_ptr(), &mut id.id)
            },
            false => unsafe { blpapi_Session_openService(self.ptr, c_service.as_ptr()) },
        } as i32;
        match res == 0 {
            true => {
                self.open_services.push(service.clone());
                Ok(self)
            }
            false => Err(Error::Session),
        }
    }

    /// Start the session
    pub fn start(&mut self) -> Result<(), Error> {
        let res = match self.async_ {
            true => {
                self.dispatcher.start()?;
                unsafe { blpapi_Session_startAsync(self.ptr) }
            }
            false => unsafe { blpapi_Session_start(self.ptr) },
        } as i32;
        match res == 0 {
            true => Ok(()),
            false => Err(Error::Session),
        }
    }

    /// Stop the session
    pub fn stop(&mut self) -> Result<(), Error> {
        let res = match self.async_ {
            true => {
                self.dispatcher.stop(&true)?;
                unsafe { blpapi_Session_stopAsync(self.ptr) }
            }
            false => unsafe { blpapi_Session_stop(self.ptr) },
        } as i32;
        match res == 0 {
            true => Ok(()),
            false => Err(Error::Session),
        }
    }

    /// Get opened service
    pub fn get_service(&self, service: &BlpServices) -> Result<Service, Error> {
        let blp_serv: &str = service.into();
        let name = CString::new(blp_serv).unwrap();
        let mut service_ptr = ptr::null_mut();
        let res = unsafe {
            blpapi_Session_getService(self.ptr, &mut service_ptr as *mut _, name.as_ptr())
        };
        Error::check(res)?;
        let if_open = self.open_services.iter().find(|x| *x == service);
        let status = match if_open {
            Some(_) => BlpServiceStatus::Active,
            None => BlpServiceStatus::InActive,
        };

        Ok(Service {
            ptr: service_ptr,
            service: service.clone(),
            status,
        })
    }

    /// Close open service
    pub fn close_service(&mut self, service: &BlpServices) -> Result<bool, Error> {
        let open_service = self.open_services.iter().find(|s| *s == service);
        if let Some(service) = open_service {
            let serv: &str = (service).into();
            self.act_services.remove(serv);
            return Ok(true);
        }
        Ok(false)
    }

    /// Create a service with the provided RequestType
    /// Opens the service and forwards the request to the
    /// RequestBuilder which than provides a complete Request
    pub fn create_request(
        &mut self,
        service: BlpServices,
        request: RequestTypes,
    ) -> Result<Request, Error> {
        let open_service = self.open_services.iter().find(|s| *s == &service);
        let service = match open_service {
            Some(blp_service) => {
                let service: &str = (blp_service).into();
                self.act_services.get(service).unwrap()
            }
            None => {
                self.open_service(&service)?;
                let new_service = self.get_service(&service)?;
                let service: &str = (&service).into();
                self.act_services.insert(service.to_string(), new_service);
                self.act_services.get(service).unwrap()
            }
        };
        let res = service.create_request(request)?;
        Ok(res)
    }

    /// Send request and get `Events` iterator
    pub fn send(
        &mut self,
        request: Request,
        correlation_id: &mut CorrelationId,
    ) -> Result<SessionEvents<'_>, Error> {
        let identity = ptr::null_mut();
        let event_queue = ptr::null_mut();
        let request_label = ptr::null_mut();
        let request_label_len = 0;
        unsafe {
            let res = blpapi_Session_sendRequest(
                self.ptr,
                request.ptr,
                &mut correlation_id.id,
                identity,
                event_queue,
                request_label,
                request_label_len,
            );
            Error::check(res)?;
        }
        Ok(SessionEvents::new(self, *correlation_id))
    }

    /// Request for next event, optionally waiting timeout_ms if there is no event
    pub fn next_event(&mut self) -> Result<Event, Error> {
        let mut event: *mut blpapi_Event_t = ptr::null_mut();
        unsafe {
            let res = blpapi_Session_nextEvent(self.ptr, &mut event as *mut _, self.time_out);
            Error::check(res)?;
            let event = EventBuilder::default().ptr(event).build();
            Ok(event)
        }
    }

    /// Get reference data for `RefData` items
    ///
    /// # Note
    /// For ease of use, you can activate the **derive** feature.
    /// This is blocking, since self.send(*) starts the SessionEvents Loop
    /// for event calls next > calls try_next > loop with event_types until Response
    /// or TimeOut reached > calls transpose to change Result<Option<T>,R> to Option<Result<T,R>>
    #[inline(always)]
    pub fn bdp<R>(
        &mut self,
        securities: impl IntoIterator<Item = impl AsRef<str>>,
        overrides: Option<&Vec<Override>>,
    ) -> Result<HashMap<String, R>, Error>
    where
        R: RefData,
    {
        let mut ref_data: HashMap<String, R> = HashMap::new();
        let mut iter = securities.into_iter();

        // split request as necessary to comply with bloomberg size limitations
        for fields in R::FIELDS.chunks(MAX_REFDATA_FIELDS) {
            loop {
                let service = BlpServices::ReferenceData;
                let req_t = RequestTypes::ReferenceData;
                let mut request = self.create_request(service, req_t)?;

                // add next batch of securities and exit loop if empty
                let mut is_empty = true;

                for security in iter.by_ref().take(MAX_PENDING_REQUEST / fields.len()) {
                    request.append_named(&SECURITIES, security.as_ref())?;
                    is_empty = false;
                }

                if is_empty {
                    break;
                }

                for field in fields {
                    request.append_named(&FIELDS_NAME, *field)?;
                }

                if let Some(ors) = overrides {
                    for or_strct in ors {
                        let mut over_item = request.append_complex(&OVERRIDES)?;
                        let field_id = or_strct.field_id.to_str().unwrap_or_default();
                        let value = or_strct.value.as_str();
                        over_item.set_named(&FIELD_ID, field_id)?;
                        over_item.set_named(&VALUE, value)?;
                        over_item.create();
                    }
                }

                // let mut output_buffer = Vec::new();
                // let res = request.element().print(&mut output_buffer, 2, 4);
                // let output_string = String::from_utf8(output_buffer).unwrap();
                // println!("{}", output_string);

                // let session_events = self.send(request)?;
                // let cor_id = session_events.correlation_id;

                // for event in self.send(request, &correlation_id)? {
                let mut correlation_id = self.new_correlation_id();
                for event in self.send(request, &mut correlation_id)? {
                    for message in event?.messages() {
                        process_message(message.element(), &mut ref_data)?;
                    }
                }
                let _ = self.remove_active_correlation_id(correlation_id);
            }
        }
        Ok(ref_data)
    }

    pub fn remove_active_correlation_id(
        &mut self,
        correlation_id: CorrelationId,
    ) -> Result<(), Error> {
        let cor_index = self
            .correlation_ids
            .iter()
            .position(|x| x.value == correlation_id.value)
            .unwrap();
        self.correlation_ids.remove(cor_index);
        drop(correlation_id);
        Ok(())
    }

    /// Get reference data for `HistoricalData` items
    ///
    /// # Note
    /// For ease of use, you can activate the **derive** feature.
    /// This is blocking, since self.send(*) starts the SessionEvents Loop
    /// for event calls next > calls try_next > loop with event_types until Response
    /// or TimeOut reached > calls transpose to change Result<Option<T>,R> to Option<Result<T,R>>
    #[inline(always)]
    pub fn hist_data_sync<R>(
        &mut self,
        securities: impl IntoIterator<Item = impl AsRef<str>>,
        options: HistOptions,
    ) -> Result<HashMap<String, TimeSerie<R>>, Error>
    where
        R: RefData,
    {
        let mut ref_data: HashMap<String, TimeSerie<R>> = HashMap::new();
        let mut iter = securities.into_iter();

        // split request as necessary to comply with bloomberg size limitations
        for fields in R::FIELDS.chunks(MAX_HISTDATA_FIELDS) {
            loop {
                // add next batch of securities and exit loop if empty
                let service = BlpServices::ReferenceData;
                let req_t = RequestTypes::HistoricalData;
                let mut request = self.create_request(service, req_t)?;

                let mut is_empty = true;

                for security in iter.by_ref().take(MAX_PENDING_REQUEST / fields.len()) {
                    request.append_named(&SECURITIES, security.as_ref())?;
                    is_empty = false;
                }

                if is_empty {
                    break;
                }

                options.apply(&mut request)?;

                for field in fields {
                    request.append_named(&FIELDS_NAME, *field)?;
                }

                let mut correlation_id = self.new_correlation_id();
                for event in self.send(request, &mut correlation_id)? {
                    for message in event?.messages() {
                        process_message_ts(&mut message.element(), &mut ref_data)?;
                    }
                }
            }
        }
        Ok(ref_data)
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe { blpapi_Session_destroy(self.ptr) }
    }
}

#[inline(always)]
fn process_message<R: RefData>(
    message: Element,
    ref_data: &mut HashMap<String, R>,
) -> Result<(), Error> {
    let securities_data = match message.get_named_element(&SECURITY_DATA) {
        Some(el) => el,
        None => return Ok(()),
    };

    for security in securities_data.values::<Element>() {
        let ticker = security
            .get_named_element(&SECURITY_NAME)
            .and_then(|s| s.get_at(0))
            .unwrap_or_default();

        // Check for specific security errors
        if let Some(error) = security.get_named_element(&SECURITY_ERROR) {
            return Err(Error::security(ticker, error));
        }

        // Update the entry
        // Use .entry() because we might visit this security multiple times
        // if we had to split the Fields into multiple requests
        let entry = ref_data.entry(ticker).or_default();

        if let Some(fields) = security.get_named_element(&FIELD_DATA) {
            for mut field in fields.elements() {
                field.create();
                entry.on_field(&field.string_name(), &field);
            }
        }
    }
    Ok(())
}

#[inline(always)]
fn process_message_ts<R: RefData>(
    message: &mut Element,
    ref_data: &mut HashMap<String, TimeSerie<R>>,
) -> Result<(), Error> {
    message.create();
    if let Some(ref mut security_data) = message.security_data {
        security_data.create();
        // Get Ticker
        if let Some(ref mut ticker) = security_data.security_name {
            ticker.create();
            let ticker_str = ticker
                .values
                .get(&0)
                .unwrap_or(&String::from("security_name"))
                .to_string();

            // Check for error
            if let Some(ref mut error) = security_data.security_error {
                return Err(Error::security(ticker_str, *error.clone()));
            }

            // Get the field data
            if let Some(ref mut fields) = security_data.field_data {
                fields.create();
                let entry = ref_data.entry(ticker_str).or_insert_with(|| {
                    let len = fields.num_values();
                    TimeSerie::<_>::with_capacity(len)
                });
                for mut points in fields.values::<Element>() {
                    points.create();
                    let mut value = R::default();
                    for mut field in points.elements() {
                        field.create();
                        let name = field.string_name();
                        if name == "date" {
                            #[cfg(feature = "dates")]
                            entry.dates.extend(field.get_at::<chrono::NaiveDate>(0));
                            #[cfg(not(feature = "dates"))]
                            entry.dates.extend(field.get_at(0));
                        } else {
                            value.on_field(&name, &field);
                        }
                    }
                    entry.values.push(value);
                }
            }
        }
    }

    Ok(())
}
