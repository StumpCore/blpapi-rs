use crate::{
    core::{
        BLPAPI_DEFAULT_BEQS_DATA_REQUEST, BLPAPI_DEFAULT_CATEGORIZED_FIELD_SEARCH_DATA_REQUEST,
        BLPAPI_DEFAULT_FIELD_INFO_REQUEST_DATA_REQUEST,
        BLPAPI_DEFAULT_FIELD_LIST_REQUEST_DATA_REQUEST,
        BLPAPI_DEFAULT_FIELD_SEARCH_REQUEST_DATA_REQUEST, BLPAPI_DEFAULT_HISTORICAL_DATA_REQUEST,
        BLPAPI_DEFAULT_INTRADAY_BAR_DATA_REQUEST, BLPAPI_DEFAULT_INTRADAY_TICK_DATA_REQUEST,
        BLPAPI_DEFAULT_REFERENCE_DATA_REQUEST, BLPAPI_DEFAULT_STUDY_DATA_REQUEST,
    },
    element::{Element, SetValue},
    name::Name,
    service::Service,
    Error,
};
use blpapi_sys::*;
use std::ffi::{c_char, CStr, CString};

#[derive(Clone, Copy)]
pub enum RequestTypes {
    ReferenceData,
    FieldList,
    FieldInfo,
    FieldSearch,
    CategorizedFieldSearch,
    Study,
    HistoricalData,
    IntradayBar,
    IntradayTick,
    Beqs,
}

impl From<RequestTypes> for &str {
    fn from(arg: RequestTypes) -> Self {
        match arg {
            RequestTypes::ReferenceData => BLPAPI_DEFAULT_REFERENCE_DATA_REQUEST,
            RequestTypes::FieldList => BLPAPI_DEFAULT_FIELD_LIST_REQUEST_DATA_REQUEST,
            RequestTypes::FieldInfo => BLPAPI_DEFAULT_FIELD_INFO_REQUEST_DATA_REQUEST,
            RequestTypes::FieldSearch => BLPAPI_DEFAULT_FIELD_SEARCH_REQUEST_DATA_REQUEST,
            RequestTypes::CategorizedFieldSearch => {
                BLPAPI_DEFAULT_CATEGORIZED_FIELD_SEARCH_DATA_REQUEST
            }
            RequestTypes::Study => BLPAPI_DEFAULT_STUDY_DATA_REQUEST,
            RequestTypes::HistoricalData => BLPAPI_DEFAULT_HISTORICAL_DATA_REQUEST,
            RequestTypes::IntradayBar => BLPAPI_DEFAULT_INTRADAY_BAR_DATA_REQUEST,
            RequestTypes::IntradayTick => BLPAPI_DEFAULT_INTRADAY_TICK_DATA_REQUEST,
            RequestTypes::Beqs => BLPAPI_DEFAULT_BEQS_DATA_REQUEST,
        }
    }
}

#[derive(Clone)]
pub struct RequestBuilder {
    pub request_type: RequestTypes,
    pub service: Option<Service>,
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self {
            service: None,
            request_type: RequestTypes::ReferenceData,
        }
    }
}

impl RequestBuilder {
    /// Setting new request type
    pub fn request_type(&mut self, new_req_t: &RequestTypes) -> &mut Self {
        self.request_type = new_req_t.clone();
        self
    }

    /// Setting new service
    pub fn service(&mut self, new_service: &Service) -> &mut Self {
        self.service = Some(new_service.clone());
        self
    }

    pub fn build(self) -> Result<Request, Error> {
        let service = self.service.expect("Service failed. Set Service first.");
        let req_t: &str = self.request_type.into();
        let operation = CString::new(req_t).expect("CString::new() failed.");
        let mut ptr = std::ptr::null_mut();
        let refptr = &mut ptr as *mut _;
        unsafe {
            let res = blpapi_Service_createRequest(service.ptr, refptr, operation.as_ptr());
            Error::check(res)?;
            let elements = blpapi_Request_elements(ptr);
            Ok(Request { ptr, elements })
        }
    }
}

/// A `Request`
/// Created from `Service::create_request`
///
/// A `Request` dereferences to an element
#[derive(Clone, Debug)]
pub struct Request {
    pub(crate) ptr: *mut blpapi_Request_t,
    elements: *mut blpapi_Element_t,
}

impl Request {
    /// Get the existing elements of request
    pub fn elements(&self) -> *mut blpapi_Element_t {
        let elements = unsafe { blpapi_Request_elements(self.ptr) };
        elements
    }

    /// Convert the request to an Element
    pub fn element(&self) -> Element {
        Element { ptr: self.elements }
    }

    /// Get the reqquest identifier
    pub fn request_id(&self) -> String {
        let mut ptr: *const c_char = std::ptr::null();
        let res = unsafe { blpapi_Request_getRequestId(self.ptr as *const _, &mut ptr) } as i64;
        match res == 0 {
            true => unsafe { CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string() },
            false => String::default(),
        }
    }

    /// Append a new value to the existing inner Element sequence defined by name
    pub fn append<V: SetValue>(&mut self, name: &str, value: V) -> Result<(), Error> {
        let mut element = self
            .element()
            .get_element(name)
            .ok_or_else(|| Error::NotFound(name.to_owned()))?;
        element.append(value)
    }

    /// Append a new value to the existing inner Element sequence defined by name
    pub fn append_named<V: SetValue>(&mut self, name: &Name, value: V) -> Result<(), Error> {
        self.element()
            .get_named_element(name)
            .ok_or_else(|| Error::NotFound(name.to_string()))?
            .append(value)
    }
}

impl Drop for Request {
    fn drop(&mut self) {
        unsafe { blpapi_Request_destroy(self.ptr) }
    }
}
