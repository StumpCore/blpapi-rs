use crate::{
    abstract_session::AbstractSession,
    correlation_id::{CorrelationId, CorrelationIdBuilder},
    data_series::{
        CurveOptions, DataSeries, DataSeriesBuilder, FieldSeries, FieldSeriesBuilder, FieldTypes,
        Language, Security, SecurityBuilder, SecurityLookUp, SecurityLookUpBuilder,
        SecuritySubType, SecurityType, YellowKey,
    },
    element::Element,
    event::{Event, EventBuilder, EventQueue, EventType, SessionEvents},
    event_dispatcher::{EventDispatcher, EventDispatcherBuilder},
    identity::{Identity, IdentityBuilder, SeatType},
    message::MessageStatus,
    names::{
        BBG_ID, COUNTRY_CODE, CURRENCY_CODE, CURVE_ID, EVENT_TYPES, FIELDS_EXCLUDE, FIELDS_NAME,
        FIELDS_REQUEST_ID, FIELDS_SEARCH, FIELD_DATA, FIELD_DATA_ERROR, FIELD_ID, FIELD_TYPE,
        FIELD_TYPE_DOCS, LANGUAGE_OVERRIDE, MAX_RESULTS, OVERRIDES, PARTIAL_MATCH, QUERY, RESULTS,
        SECURITIES, SECURITY, SECURITY_DATA, SECURITY_ERROR, SECURITY_NAME, SECURITY_SUBTYPE,
        SECURITY_TYPE, TICKER, TICK_DATA, VALUE, YELLOW_KEY_FILTER,
    },
    overrides::Override,
    ref_data::{self, RefData},
    request::{Request, RequestTypes},
    service::{BlpServiceStatus, BlpServices, Service},
    session_options::SessionOptions,
    time_series::{
        DateType, HistIntradayOptions, HistOptions, IntradayDateType, TickData, TickDataBuilder,
        TickTypes, TimeSerieBuilder, TimeSeries,
    },
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
            correlation_count: 1,
            event_queue: true,
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
            correlation_count: 1,
            event_queue: false,
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
    pub correlation_count: u64,
    pub time_out: u32,
    pub event_queue: bool,
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
        let event_queue = match self.event_queue {
            true => EventQueue::new(self.time_out as i32),
            false => EventQueue::default(),
        };
        let request_label = ptr::null_mut();
        let request_label_len = 0;
        unsafe {
            let res = blpapi_Session_sendRequest(
                self.ptr,
                request.ptr,
                &mut correlation_id.id,
                identity,
                event_queue.ptr,
                request_label,
                request_label_len,
            );
            Error::check(res)?;
        }
        Ok(SessionEvents::new(self, *correlation_id, event_queue))
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

    /// Request for try-next event, if there is no event continue
    pub fn try_next_event(&mut self) -> Option<Event> {
        let mut event: *mut blpapi_Event_t = ptr::null_mut();
        unsafe {
            let res = blpapi_Session_tryNextEvent(self.ptr, &mut event);

            match res == 0 {
                true => Some(EventBuilder::default().ptr(event).build()),
                false => None,
            }
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
        tickers: impl IntoIterator<Item = impl AsRef<str>>,
        overrides: Option<&Vec<Override>>,
        static_mkt: bool,
    ) -> Result<Vec<DataSeries<R>>, Error>
    where
        R: RefData,
    {
        // let mut ref_data: HashMap<String, R> = HashMap::new();
        let mut ref_data: Vec<DataSeries<R>> = vec![];
        let mut iter = tickers.into_iter();

        // split request as necessary to comply with bloomberg size limitations
        for fields in R::FIELDS.chunks(MAX_REFDATA_FIELDS) {
            loop {
                let service = match static_mkt {
                    true => BlpServices::StaticReferenceData,
                    false => BlpServices::ReferenceData,
                };
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

                // Setting Overrides
                if let Some(ors) = overrides {
                    for or_strct in ors {
                        let mut over_item = request.append_complex(&OVERRIDES)?;
                        let field_id = or_strct.field_id.name.to_uppercase();
                        let field_id = field_id.as_str();
                        let value = or_strct.value.as_str();
                        over_item.set_named(&FIELD_ID, field_id)?;
                        over_item.set_named(&VALUE, value)?;
                    }
                }

                // for event in self.send(request, &correlation_id)? {
                let mut correlation_id = self.new_correlation_id();
                for event in self.send(request, &mut correlation_id)? {
                    for message in event?.messages() {
                        process_message(message.element(), &mut ref_data)?;
                    }
                }
            }
        }
        Ok(ref_data)
    }

    /// Get reference data for `HistoricalData` items
    ///
    /// # Note
    /// For ease of use, you can activate the **derive** feature.
    /// This is blocking, since self.send(*) starts the SessionEvents Loop
    /// for event calls next > calls try_next > loop with event_types until Response
    /// or TimeOut reached > calls transpose to change Result<Option<T>,R> to Option<Result<T,R>>
    #[inline(always)]
    pub fn bdh<R>(
        &mut self,
        tickers: impl IntoIterator<Item = impl AsRef<str>>,
        options: HistOptions,
    ) -> Result<Vec<TimeSeries<R, DateType>>, Error>
    where
        R: RefData,
    {
        let mut ref_data: Vec<TimeSeries<R, DateType>> = vec![];

        let mut iter = tickers.into_iter();

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

    /// Get reference data for `HistoricalData` tick items
    ///
    /// # Note
    /// For ease of use, you can activate the **derive** feature.
    /// This is blocking, since self.send(*) starts the SessionEvents Loop
    /// for event calls next > calls try_next > loop with event_types until Response
    /// or TimeOut reached > calls transpose to change Result<Option<T>,R> to Option<Result<T,R>>
    #[inline(always)]
    pub fn bdib(
        &mut self,
        ticker: String,
        tick_types: Vec<TickTypes>,
        options: HistIntradayOptions,
    ) -> Result<Vec<TimeSeries<TickData, IntradayDateType>>, Error> {
        let mut ref_data: Vec<TimeSeries<TickData, IntradayDateType>> = vec![];

        // split request as necessary to comply with bloomberg size limitations
        loop {
            let mut is_over = true;
            // add next batch of securities and exit loop if empty
            let service = BlpServices::ReferenceData;
            let req_t = RequestTypes::IntradayTick;
            let mut request = self.create_request(service, req_t)?;

            request.element().set_named(&SECURITY, ticker.as_ref())?;

            options.apply(&mut request)?;

            for field in tick_types.iter() {
                let tick_type: &str = field.into();
                request.append_named(&EVENT_TYPES, tick_type.as_ref())?;
            }

            let mut correlation_id = self.new_correlation_id();
            for event in self.send(request, &mut correlation_id)? {
                let mut event = event?;
                for message in event.messages() {
                    let msg_status = &message.message_type.status;
                    match msg_status {
                        MessageStatus::Active => is_over = false,
                        _ => is_over = true,
                    }

                    process_message_ts_tick_data(
                        &mut message.element(),
                        ticker.as_str(),
                        &mut ref_data,
                    )?;
                }
                let event_type = event.event_type();
                if event_type == EventType::Response {
                    is_over = true;
                }
            }

            if is_over {
                break;
            }
        }
        Ok(ref_data)
    }

    /// Get reference data for `HistoricalData` items
    ///
    /// # Note
    /// For ease of use, you can activate the **derive** feature.
    /// This is blocking, since self.send(*) starts the SessionEvents Loop
    /// for event calls next > calls try_next > loop with event_types until Response
    /// or TimeOut reached > calls transpose to change Result<Option<T>,R> to Option<Result<T,R>>
    #[inline(always)]
    pub fn field_info<R>(
        &mut self,
        top_fields: Option<Vec<&str>>,
        sub_fields: Option<Vec<&str>>,
    ) -> Result<Vec<FieldSeries>, Error>
    where
        R: RefData,
    {
        let mut ref_data: Vec<FieldSeries> = vec![];

        // split request as necessary to comply with bloomberg size limitations
        for fields in R::FIELDS.chunks(MAX_HISTDATA_FIELDS) {
            loop {
                // add next batch of securities and exit loop if empty
                let service = BlpServices::ApiFields;
                let req_t = RequestTypes::FieldInfo;
                let mut request = self.create_request(service, req_t)?;

                let mut is_empty = true;

                for field in fields {
                    request.append_named(&FIELDS_REQUEST_ID, *field)?;
                    is_empty = false;
                }
                let mut element = request.element();
                element.set_named(&FIELD_TYPE_DOCS, true)?;

                let mut correlation_id = self.new_correlation_id();
                for event in self.send(request, &mut correlation_id)? {
                    for message in event?.messages() {
                        process_message_fields(
                            message.element(),
                            &mut ref_data,
                            &top_fields,
                            &sub_fields,
                        )?;
                        is_empty = true;
                    }
                }
                if is_empty {
                    break;
                }
            }
        }
        Ok(ref_data)
    }

    /// Get reference data for `HistoricalData` items
    ///
    /// # Note
    /// For ease of use, you can activate the **derive** feature.
    /// This is blocking, since self.send(*) starts the SessionEvents Loop
    /// for event calls next > calls try_next > loop with event_types until Response
    /// or TimeOut reached > calls transpose to change Result<Option<T>,R> to Option<Result<T,R>>
    #[inline(always)]
    pub fn field_search(
        &mut self,
        search: Vec<&str>,
        exclude: Option<Vec<&str>>,
    ) -> Result<Vec<FieldSeries>, Error> {
        let mut ref_data: Vec<FieldSeries> = vec![];

        // split request as necessary to comply with bloomberg size limitations
        for fields in search.chunks(MAX_REFDATA_FIELDS) {
            loop {
                // add next batch of securities and exit loop if empty
                let service = BlpServices::ApiFields;
                let req_t = RequestTypes::FieldSearch;
                let request = self.create_request(service, req_t)?;

                let mut is_empty = true;

                let mut element = request.element();
                for field in fields {
                    element.set_named(&FIELDS_SEARCH, *field)?;
                    is_empty = false;
                }

                if let Some(ex_field) = &exclude {
                    let element = request.element().get_named_element(&FIELDS_EXCLUDE);
                    if let Some(mut req_ele_exc) = element {
                        for field in ex_field {
                            req_ele_exc.set_named(&FIELD_TYPE, *field)?;
                        }
                    }
                }

                let mut correlation_id = self.new_correlation_id();
                for event in self.send(request, &mut correlation_id)? {
                    for message in event?.messages() {
                        process_message_fields(message.element(), &mut ref_data, &None, &None)?;
                        is_empty = true;
                    }
                }
                if is_empty {
                    break;
                }
            }
        }
        Ok(ref_data)
    }

    /// Get reference data for `HistoricalData` items
    ///
    /// # Note
    /// For ease of use, you can activate the **derive** feature.
    /// This is blocking, since self.send(*) starts the SessionEvents Loop
    /// for event calls next > calls try_next > loop with event_types until Response
    /// or TimeOut reached > calls transpose to change Result<Option<T>,R> to Option<Result<T,R>>
    #[inline(always)]
    pub fn field_list(&mut self, block: u64, field: FieldTypes) -> Result<Vec<FieldSeries>, Error> {
        let mut ref_data: Vec<FieldSeries> = vec![];
        let mut count = 1;
        let field_t: &str = field.into();

        // split request as necessary to comply with bloomberg size limitations
        loop {
            // add next batch of securities and exit loop if empty
            let service = BlpServices::ApiFields;
            let req_t = RequestTypes::FieldList;
            let request = self.create_request(service, req_t)?;

            let mut is_empty = true;

            let mut element = request.element();
            element.set_named(&FIELD_TYPE, field_t)?;
            element.set_named(&FIELD_TYPE_DOCS, true)?;

            let mut correlation_id = self.new_correlation_id();
            for event in self.send(request, &mut correlation_id)? {
                for message in event?.messages() {
                    process_message_fields(message.element(), &mut ref_data, &None, &None)?;
                }
                if count == block {
                    is_empty = true;
                    break;
                }
                count += 1;
            }
            if is_empty {
                break;
            }
        }
        Ok(ref_data)
    }

    #[inline(always)]
    pub fn lookup_security<S: Into<String>>(
        &mut self,
        name: S,
        max_results: i32,
        yk: Option<YellowKey>,
        lo: Option<Language>,
    ) -> Result<SecurityLookUp, Error> {
        let name = name.into();
        let q_name = name.clone();
        let mut ref_data: SecurityLookUpBuilder = SecurityLookUpBuilder::default();
        ref_data.query(q_name);
        let yk: &str = yk.unwrap_or_default().into();
        let lg: &str = lo.unwrap_or_default().into();

        // split request as necessary to comply with bloomberg size limitations
        loop {
            // add next batch of securities and exit loop if empty
            let service = BlpServices::Instruments;
            let req_t = RequestTypes::InstrumentList;
            let request = self.create_request(service, req_t)?;

            let mut is_empty = true;

            let mut element = request.element();
            element.set_named(&QUERY, name.as_str())?;
            element.set_named(&YELLOW_KEY_FILTER, yk)?;
            element.set_named(&LANGUAGE_OVERRIDE, lg)?;
            element.set_named(&MAX_RESULTS, max_results)?;

            let mut correlation_id = self.new_correlation_id();
            for event in self.send(request, &mut correlation_id)? {
                for message in event?.messages() {
                    process_message_sec_look_up(&mut message.element(), &mut ref_data)?;
                }
                is_empty = true
            }
            if is_empty {
                break;
            }
        }
        let ref_data = ref_data.build();
        Ok(ref_data)
    }

    #[inline(always)]
    pub fn lookup_security_curved(
        &mut self,
        max_results: i32,
        options: CurveOptions,
    ) -> Result<SecurityLookUp, Error> {
        let name = options.query;

        let q_name = name.clone();
        let mut ref_data: SecurityLookUpBuilder = SecurityLookUpBuilder::default();
        ref_data.query(q_name);

        // split request as necessary to comply with bloomberg size limitations
        loop {
            // add next batch of securities and exit loop if empty
            let service = BlpServices::Instruments;
            let req_t = RequestTypes::CurvedList;
            let request = self.create_request(service, req_t)?;

            let mut is_empty = false;

            let mut element = request.element();
            element.set_named(&QUERY, name.as_str())?;
            element.set_named(&MAX_RESULTS, max_results)?;
            if let Some(ref bbg_id) = options.bbg_id {
                element.set_named(&BBG_ID, bbg_id.as_str())?;
            }
            if let Some(ref country) = options.country {
                element.set_named(&COUNTRY_CODE, country.as_str())?;
            }
            if let Some(ref currency) = options.currency {
                element.set_named(&CURRENCY_CODE, currency.as_str())?;
            }
            if let Some(ref curve_id) = options.curve_id {
                element.set_named(&CURVE_ID, curve_id.as_str())?;
            }
            if let Some(sec_type) = options.sec_type {
                let sec_type: &str = sec_type.into();
                element.set_named(&SECURITY_TYPE, sec_type)?;
            }
            if let Some(sec_subtype) = options.sec_subtype {
                let sec_subtype: &str = sec_subtype.into();
                element.set_named(&SECURITY_SUBTYPE, sec_subtype)?;
            }

            let mut correlation_id = self.new_correlation_id();
            for event in self.send(request, &mut correlation_id)? {
                for message in event?.messages() {
                    process_message_sec_look_up(&mut message.element(), &mut ref_data)?;
                }
                is_empty = true
            }
            if is_empty {
                break;
            }
        }
        let ref_data = ref_data.build();
        Ok(ref_data)
    }

    #[inline(always)]
    pub fn lookup_security_govt<S>(
        &mut self,
        query: S,
        max_results: i32,
        partial_match: bool,
        ticker: Option<S>,
    ) -> Result<SecurityLookUp, Error>
    where
        S: AsRef<str>,
    {
        let name = query.as_ref();
        let q_name = name.to_string();
        let mut ref_data: SecurityLookUpBuilder = SecurityLookUpBuilder::default();
        ref_data.query(q_name);

        // split request as necessary to comply with bloomberg size limitations
        loop {
            // add next batch of securities and exit loop if empty
            let service = BlpServices::Instruments;
            let req_t = RequestTypes::GovtList;
            let request = self.create_request(service, req_t)?;

            let mut is_empty = false;

            let mut element = request.element();
            element.set_named(&QUERY, name)?;
            element.set_named(&MAX_RESULTS, max_results)?;
            element.set_named(&PARTIAL_MATCH, partial_match)?;
            if let Some(ref ticker) = ticker {
                element.set_named(&TICKER, ticker.as_ref())?;
            }

            let mut correlation_id = self.new_correlation_id();
            for event in self.send(request, &mut correlation_id)? {
                for message in event?.messages() {
                    process_message_sec_look_up(&mut message.element(), &mut ref_data)?;
                }
                is_empty = true
            }
            if is_empty {
                break;
            }
        }
        let ref_data = ref_data.build();
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
    data_vec: &mut Vec<DataSeries<R>>,
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

        if let Some(fields) = security.get_named_element(&FIELD_DATA) {
            let len = fields.num_values();
            let mut data_builder = DataSeriesBuilder::<_>::with_capacity(len, ticker);
            let mut value = R::default();

            for field in fields.elements() {
                value.on_field(&field.string_name(), &field);
            }
            data_builder.values.push(value);

            let data_rows = data_builder.to_rows();

            data_vec.extend(data_rows);
        }
    }
    Ok(())
}

#[inline(always)]
fn process_message_ts<R: RefData>(
    message: &mut Element,
    ts_vec: &mut Vec<TimeSeries<R, DateType>>,
) -> Result<(), Error> {
    message.create();

    if let Some(ref mut security_data) = message.security_data {
        security_data.create();

        // Get Ticker
        if let Some(ref mut ticker) = security_data.security_name {
            let ticker_str: String = ticker.get_at(0).unwrap_or_default();

            // Check for error
            if let Some(ref mut error) = security_data.security_error {
                return Err(Error::security(ticker_str, *error.clone()));
            }

            // Get the field data
            if let Some(ref mut fields) = security_data.field_data {
                fields.create();
                let len = fields.num_values();
                let mut ts_builder =
                    TimeSerieBuilder::<R, DateType>::with_capacity(len, ticker_str);

                for points in fields.values::<Element>() {
                    let mut value = R::default();

                    for mut field in points.elements() {
                        field.create();
                        let name = field.string_name();

                        if name == "date" {
                            if let Some(d) = field.get_at::<DateType>(0) {
                                ts_builder.dates.push(d);
                            }
                        } else {
                            value.on_field(&name, &field);
                        }
                    }
                    ts_builder.values.push(value);
                }
                let ts_rows = ts_builder.to_rows();

                ts_vec.extend(ts_rows);
            }
        }
    }

    Ok(())
}

#[inline(always)]
fn process_message_ts_tick_data(
    message: &mut Element,
    ticker: &str,
    ts_vec: &mut Vec<TimeSeries<TickData, IntradayDateType>>,
) -> Result<(), Error> {
    message.create();

    let msg_respone = message.string_name();
    if msg_respone == "IntradayTickResponse" {
        let outer: Element = message.get_named_element(&TICK_DATA).unwrap_or_default();
        if let Some(fields) = outer.get_named_element(&TICK_DATA) {
            let len = fields.num_values();
            let mut ts_builder = TimeSerieBuilder::<TickData, IntradayDateType>::with_capacity(
                len,
                ticker.to_string(),
            );
            for points in fields.values::<Element>() {
                let mut td = TickDataBuilder::default();
                for mut field in points.elements() {
                    field.create();
                    let name = field.string_name();
                    let value = field.get_at::<String>(0).unwrap_or_default();

                    match name.as_str() {
                        "time" => {
                            if let Some(d) = field.get_at::<IntradayDateType>(0) {
                                ts_builder.dates.push(d);
                            }
                        }
                        "type" => {
                            td.tick_type(value);
                        }
                        "value" => {
                            let p_value = value.parse::<f64>().unwrap_or_default();
                            td.value(p_value);
                        }
                        "size" => {
                            let p_value = value.parse::<i32>().unwrap_or_default();
                            td.size(p_value);
                        }
                        "exch_code" => {
                            td.exchange_code(value);
                        }
                        "conditionCodes" => {
                            td.conditional_codes(value);
                        }
                        _ => {
                            td.other(name, value);
                        }
                    };
                }
                let td_s = td.build();
                ts_builder.values.push(td_s);
            }
            let ts_rows = ts_builder.to_rows();

            ts_vec.extend(ts_rows);
        }
    }

    Ok(())
}

#[inline(always)]
fn process_message_fields(
    message: Element,
    data_vec: &mut Vec<FieldSeries>,
    top_fields: &Option<Vec<&str>>,
    sub_fields: &Option<Vec<&str>>,
) -> Result<(), Error> {
    let mut tl_attr = vec!["id", "fieldInfo"];
    if let Some(tl_fields) = top_fields {
        let mut tl_f = tl_fields.clone();
        tl_attr.append(&mut tl_f);
    };
    let mut sub_attr = vec![
        "datatype",
        "ftype",
        "categoryName",
        "defaultFormatting",
        "documentation",
        "mnemonic",
        "description",
        "property",
        "overrides",
    ];
    if let Some(sl_fields) = sub_fields {
        let mut sb_f = sl_fields.clone();
        sub_attr.append(&mut sb_f);
    };
    if let Some(fields) = message.get_named_element(&FIELD_DATA) {
        let len = fields.num_values();

        for i in 0..=len {
            let ele: Option<Element> = fields.get_at(i);
            if let Some(mut field) = ele {
                field.create();
                // Check for specific field errors
                if let Some(error) = field.get_named_element(&FIELD_DATA_ERROR) {
                    return Err(Error::field(error));
                }
                let mut field_builder = FieldSeriesBuilder::default();
                for ele_name in tl_attr.iter() {
                    let field_name = field.get_element(ele_name);

                    if let Some(name) = field_name {
                        let name_str = name.string_name();
                        let value = name.get_at::<String>(0).unwrap_or_default();

                        match name_str.as_str() {
                            "id" => {
                                field_builder.id(value);
                            }
                            "fieldInfo" => {
                                let info = field.get_element("fieldInfo");
                                if let Some(info) = &info {
                                    for sub_field in sub_attr.iter() {
                                        let sub_field_name = info.get_element(sub_field);
                                        if let Some(mut sub_ele) = sub_field_name {
                                            let sub_name_str = sub_ele.string_name();
                                            let sub_value =
                                                sub_ele.get_at::<String>(0).unwrap_or_default();
                                            match sub_name_str.as_str() {
                                                "mnemonic" => {
                                                    field_builder.mnemonic(sub_value);
                                                }
                                                "description" => {
                                                    field_builder.desc(sub_value);
                                                }
                                                "datatype" => {
                                                    field_builder.data_type(sub_value);
                                                }
                                                "ftype" => {
                                                    field_builder.field_type(sub_value);
                                                }
                                                "documentation" => {
                                                    field_builder.field_documentation(sub_value);
                                                }
                                                "categoryName" => {
                                                    field_builder.field_category(sub_value);
                                                }
                                                "overrides" => {
                                                    let len = sub_ele.num_values();
                                                    for i in 0..len {
                                                        let sub_value = sub_ele
                                                            .get_at::<String>(i)
                                                            .unwrap_or_default();
                                                        field_builder.overrides(sub_value);
                                                    }
                                                }
                                                "property" => {
                                                    for ele in sub_ele.elements() {
                                                        let sub_name_str = ele.string_name();
                                                        let sub_value = ele
                                                            .get_at::<String>(0)
                                                            .unwrap_or_default();
                                                        field_builder.field_property(
                                                            sub_name_str,
                                                            sub_value,
                                                        );
                                                    }
                                                }
                                                "defaultFormatting" => {
                                                    for ele in sub_ele.elements() {
                                                        let sub_name_str = ele.string_name();
                                                        let sub_value = ele
                                                            .get_at::<String>(0)
                                                            .unwrap_or_default();
                                                        field_builder.field_default_formatting(
                                                            sub_name_str,
                                                            sub_value,
                                                        );
                                                    }
                                                }
                                                _ => {
                                                    for ele in sub_ele.elements() {
                                                        let sub_name_str = ele.string_name();
                                                        let sub_value = ele
                                                            .get_at::<String>(0)
                                                            .unwrap_or_default();
                                                        field_builder.field_property(
                                                            sub_name_str,
                                                            sub_value,
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                            }
                            _ => {}
                        };
                    }
                }
                let field_s = field_builder.build();
                data_vec.push(field_s);
            }
        }
    }
    Ok(())
}

#[inline(always)]
fn process_message_sec_look_up(
    message: &mut Element,
    look_up: &mut SecurityLookUpBuilder,
) -> Result<(), Error> {
    let attributs = vec![
        "id",
        "description",
        "currency",
        "security",
        "country",
        "curveid",
        "type",
        "subtype",
        "publisher",
        "bbgid",
        "parseky",
        "name",
        "ticker",
        "yellowkey",
    ];

    let mut security_vec: Vec<Security> = vec![];
    let query = message.get_named_element(&QUERY);
    if let Some(query) = query {
        look_up.query(query.string_name());
    };

    let results = message.get_named_element(&RESULTS);
    if let Some(results) = results {
        let len = results.num_values();
        look_up.total_results(len as i32);
        for _ in 0..len {
            let mut sec_builder = SecurityBuilder::default();
            let entries = results.values::<Element>();
            for entry in entries {
                for at in attributs.iter() {
                    let ele = entry.get_element(at);
                    let name = *at;

                    if let Some(ele) = ele {
                        let value = ele.get_at::<String>(0).unwrap_or_default();
                        match name {
                            "id" => {
                                sec_builder.id(value);
                            }
                            "description" => {
                                sec_builder.description(value);
                            }
                            "currency" => {
                                sec_builder.currency(value);
                            }
                            "security" => {
                                sec_builder.security(value);
                            }
                            "country" => {
                                sec_builder.country_code(value);
                            }
                            "curveid" => {
                                sec_builder.curve_id(value);
                            }
                            "type" => {
                                sec_builder.security_type(value);
                            }
                            "subtype" => {
                                sec_builder.security_subtype(value);
                            }
                            "publisher" => {
                                sec_builder.publisher(value);
                            }
                            "bbgid" => {
                                sec_builder.bbg_id(value);
                            }
                            "parseky" => {
                                sec_builder.parse_key(value);
                            }
                            "name" | "curve" => {
                                sec_builder.name(value);
                            }
                            "isin" => {
                                sec_builder.isin(value);
                            }
                            "sedol" => {
                                sec_builder.sedol(value);
                            }
                            "ticker" => {
                                sec_builder.ticker(value);
                            }
                            "yellowkey" => {
                                sec_builder.yellow_key(value.as_str());
                            }
                            _ => {}
                        }
                    }
                }
            }
            let sec_s = sec_builder.build();
            security_vec.push(sec_s);
        }
    }
    look_up.results(security_vec);

    Ok(())
}
