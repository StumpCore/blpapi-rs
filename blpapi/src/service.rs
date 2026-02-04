use crate::{
    core::{
        BLPAPI_DEFAULT_SERVICE_IDENTIFIER_API_FIELDS,
        BLPAPI_DEFAULT_SERVICE_IDENTIFIER_CURVES_TOOLKIT,
        BLPAPI_DEFAULT_SERVICE_IDENTIFIER_INSTRUMENTS,
        BLPAPI_DEFAULT_SERVICE_IDENTIFIER_MARKET_BAR,
        BLPAPI_DEFAULT_SERVICE_IDENTIFIER_MARKET_DEPTH,
        BLPAPI_DEFAULT_SERVICE_IDENTIFIER_MARKET_LIST, BLPAPI_DEFAULT_SERVICE_IDENTIFIER_MKTDATA,
        BLPAPI_DEFAULT_SERVICE_IDENTIFIER_REFDATA, BLPAPI_DEFAULT_SERVICE_IDENTIFIER_SOURCE_REF,
        BLPAPI_DEFAULT_SERVICE_IDENTIFIER_STATIC_MKT,
        BLPAPI_DEFAULT_SERVICE_IDENTIFIER_TECHNICAL_ANALYSIS,
        BLPAPI_DEFAULT_SERVICE_IDENTIFIER_VWAP,
    },
    name::Name,
    request::{Request, RequestBuilder, RequestTypes},
    Error,
};
use blpapi_sys::*;
use std::{ffi::CStr, ptr};

pub struct Operation {
    pub(crate) ptr: *mut blpapi_Operation_t,
}

impl Operation {
    /// Get the name of the operation
    pub fn name(&self) -> String {
        let res = unsafe { CStr::from_ptr(blpapi_Operation_name(self.ptr)) };
        res.to_string_lossy().into_owned()
    }

    /// Get the description of the operation
    pub fn description(&self) -> String {
        let res = unsafe { CStr::from_ptr(blpapi_Operation_description(self.ptr)) };
        res.to_string_lossy().into_owned()
    }

    /// Get the request definition
    pub fn request_definition(&self) -> Result<(), Error> {
        let mut schema_ele: *mut blpapi_SchemaElementDefinition_t = ptr::null_mut();
        let rc = unsafe { blpapi_Operation_requestDefinition(self.ptr, &mut schema_ele) };
        println!("Req. def:{rc}");
        Ok(())
    }

    /// Number of Resonse Definition
    pub fn num_response_definition(&self) -> Result<(), Error> {
        let rc = unsafe { blpapi_Operation_numResponseDefinitions(self.ptr) };
        println!("No Response: {rc}");
        Ok(())
    }

    /// Response Definition on index
    pub fn response_definition(&self, index: usize) -> Result<(), Error> {
        let mut schema_ele: *mut blpapi_SchemaElementDefinition_t = ptr::null_mut();
        let rc = unsafe { blpapi_Operation_responseDefinition(self.ptr, &mut schema_ele, index) };
        println!("Resp. def:{rc}");
        Ok(())
    }

    /// Response Definition on name
    pub fn response_definition_from_name(&self, name: Name) -> Result<(), Error> {
        let mut schema_ele: *mut blpapi_SchemaElementDefinition_t = ptr::null_mut();
        let rc = unsafe {
            blpapi_Operation_responseDefinitionFromName(
                self.ptr,
                &mut schema_ele,
                name.ptr as *const _,
            )
        };
        println!("Resp. def:{rc}");
        Ok(())
    }
}

/// Service Status
#[derive(Clone, Debug, PartialEq, Default)]
pub enum BlpServiceStatus {
    Active,
    #[default]
    InActive,
}

/// ServiceTypes
#[derive(Clone, Debug, PartialEq, Default)]
pub enum BlpServices {
    MarketData,
    ReferenceData,
    StaticReferenceData,
    SourceReference,
    Vwap,
    /// B-Pipe Only
    MarketDepth,
    MarketBar,
    MarketList,
    ApiFields,
    Instruments,
    PageData,
    TechnicalAnalysis,
    CurvesToolkit,
    #[default]
    NoService,
}

impl From<&BlpServices> for &str {
    fn from(arg: &BlpServices) -> Self {
        match arg {
            BlpServices::MarketData => BLPAPI_DEFAULT_SERVICE_IDENTIFIER_MKTDATA,
            BlpServices::ReferenceData => BLPAPI_DEFAULT_SERVICE_IDENTIFIER_REFDATA,
            BlpServices::StaticReferenceData => BLPAPI_DEFAULT_SERVICE_IDENTIFIER_STATIC_MKT,
            BlpServices::SourceReference => BLPAPI_DEFAULT_SERVICE_IDENTIFIER_SOURCE_REF,
            BlpServices::Vwap => BLPAPI_DEFAULT_SERVICE_IDENTIFIER_VWAP,
            BlpServices::MarketDepth => BLPAPI_DEFAULT_SERVICE_IDENTIFIER_MARKET_DEPTH,
            BlpServices::MarketBar => BLPAPI_DEFAULT_SERVICE_IDENTIFIER_MARKET_BAR,
            BlpServices::MarketList => BLPAPI_DEFAULT_SERVICE_IDENTIFIER_MARKET_LIST,
            BlpServices::ApiFields => BLPAPI_DEFAULT_SERVICE_IDENTIFIER_API_FIELDS,
            BlpServices::Instruments => BLPAPI_DEFAULT_SERVICE_IDENTIFIER_INSTRUMENTS,
            BlpServices::PageData => BLPAPI_DEFAULT_SERVICE_IDENTIFIER_TECHNICAL_ANALYSIS,
            BlpServices::CurvesToolkit => BLPAPI_DEFAULT_SERVICE_IDENTIFIER_CURVES_TOOLKIT,
            BlpServices::TechnicalAnalysis => BLPAPI_DEFAULT_SERVICE_IDENTIFIER_TECHNICAL_ANALYSIS,
            BlpServices::NoService => "No-Service",
        }
    }
}

/// A `Service`
/// created from a `Session::get_service`
#[derive(Debug)]
pub struct Service {
    pub(crate) ptr: *mut blpapi_Service_t,
    pub service: BlpServices,
    pub status: BlpServiceStatus,
}

impl Default for Service {
    fn default() -> Self {
        let ptr: *mut blpapi_Service_t = ptr::null_mut();
        let service = BlpServices::ReferenceData;
        let status = BlpServiceStatus::InActive;
        Self {
            ptr,
            service,
            status,
        }
    }
}

impl Service {
    /// Get service name
    pub fn name(&self) -> String {
        let name = unsafe { CStr::from_ptr(blpapi_Service_name(self.ptr)) };
        name.to_string_lossy().into_owned()
    }

    /// Get the authorized name
    pub fn authorization_name(&self) -> String {
        let name = unsafe { CStr::from_ptr(blpapi_Service_authorizationServiceName(self.ptr)) };
        name.to_string_lossy().into_owned()
    }

    /// Get service description
    pub fn description(&self) -> String {
        let des = unsafe { CStr::from_ptr(blpapi_Service_description(self.ptr)) };
        des.to_string_lossy().into_owned()
    }

    /// Get number of operations
    pub fn num_operations(&self) -> i64 {
        unsafe { blpapi_Service_numOperations(self.ptr) as i64 }
    }

    /// Get the number of event definitions
    pub fn num_event_definitions(&self) -> i64 {
        unsafe { blpapi_Service_numEventDefinitions(self.ptr) as i64 }
    }

    /// release
    pub fn release(&self) -> Result<(), Error> {
        unsafe { blpapi_Service_release(self.ptr) };
        Ok(())
    }

    /// add new reference
    pub fn add_ref(&self) -> Result<(), Error> {
        let res = unsafe { blpapi_Service_addRef(self.ptr) };
        if res != 0 {
            return Err(Error::Service);
        };
        Ok(())
    }

    /// Create a new request
    pub fn create_request(&self, operation: RequestTypes) -> Result<Request, Error> {
        let mut req_b = RequestBuilder::default();
        req_b.request_type(&operation).service(self);
        let req = req_b.build()?;
        Ok(req)
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        self.status = BlpServiceStatus::InActive;
        unsafe { blpapi_Service_release(self.ptr) }
    }
}

impl Clone for Service {
    fn clone(&self) -> Self {
        unsafe { blpapi_Service_addRef(self.ptr) };
        Service {
            ptr: self.ptr,
            service: self.service.clone(),
            status: self.status.clone(),
        }
    }
}
