use std::{
    collections::HashMap,
    convert::TryInto,
    ffi::{c_char, CStr, CString},
    ptr::null_mut,
};

use blpapi_sys::{
    blpapi_SubscriptionList_add, blpapi_SubscriptionList_append, blpapi_SubscriptionList_clear,
    blpapi_SubscriptionList_correlationIdAt, blpapi_SubscriptionList_create,
    blpapi_SubscriptionList_destroy, blpapi_SubscriptionList_isResolvedAt,
    blpapi_SubscriptionList_size, blpapi_SubscriptionList_t, blpapi_SubscriptionList_topicStringAt,
};

use crate::{
    correlation_id::{self, CorrelationId, CorrelationIdBuilder},
    service::{self, BlpServices},
    Error,
};

/// SubscriptionListBuilder Struct

#[derive(Clone, Debug, Default)]
pub struct SubscriptionListBuilder<'a> {
    pub fields: Option<Vec<&'a str>>,
    pub options: Option<Vec<&'a str>>,
    pub no_fields: usize,
    pub no_options: usize,
    pub service: BlpServices,
    pub correlation_map: HashMap<u64, String>,
}

impl<'a> SubscriptionListBuilder<'a> {
    /// Get new fields list
    pub fn fields(mut self, new_fields: Vec<&'a str>) -> Self {
        let no_fields = new_fields.len();
        self.fields = Some(new_fields);
        self.no_fields = no_fields;
        self
    }

    /// Get new options list
    pub fn options(mut self, new_options: Vec<&'a str>) -> Self {
        let no_options = new_options.len();
        self.options = Some(new_options);
        self.no_options = no_options;
        self
    }

    /// Set new service
    pub fn service(mut self, new_service: BlpServices) -> Self {
        self.service = new_service;
        self
    }

    /// Set new hashmap
    pub fn correlation_map(mut self, hm: HashMap<u64, String>) -> Self {
        self.correlation_map = hm;
        self
    }

    pub fn build(self) -> SubscriptionList<'a> {
        let ptr = unsafe { blpapi_SubscriptionList_create() };
        let fields = self.fields.unwrap_or_default();
        let options = self.options.unwrap_or_default();
        let no_fields = self.no_fields;
        let no_options = self.no_options;
        let service = self.service;
        let correlation_map = self.correlation_map;
        SubscriptionList {
            ptr,
            fields,
            options,
            no_fields,
            no_options,
            service,
            correlation_map,
        }
    }
}

/// Subscription List Struct
#[derive(Clone, Debug)]
pub struct SubscriptionList<'a> {
    pub(crate) ptr: *mut blpapi_SubscriptionList_t,
    pub fields: Vec<&'a str>,
    pub options: Vec<&'a str>,
    pub no_fields: usize,
    pub no_options: usize,
    pub service: BlpServices,
    pub correlation_map: HashMap<u64, String>,
}

impl<'a> SubscriptionList<'a> {
    pub fn new(self, service: BlpServices, fields: Vec<&'a str>, options: Vec<&'a str>) -> Self {
        let no_fields = fields.len();
        let no_options = options.len();
        let correlation_map: HashMap<u64, String> = HashMap::new();
        let ptr = unsafe { blpapi_SubscriptionList_create() };

        Self {
            ptr,
            fields,
            options,
            no_fields,
            no_options,
            service,
            correlation_map,
        }
    }

    pub fn add(&mut self, ticker: String, corr_id: CorrelationId) -> Result<(), Error> {
        let fields = self.fields.clone();

        if (fields.is_empty()) || (self.service == BlpServices::NoService) {
            eprintln!("Fields: {:#?}", fields);
            eprintln!("Service: {:#?}", self.service);
            return Err(Error::NotFound(String::from(
                "No Fields or invalid Service",
            )));
        }

        let cor_id_ = corr_id.value;
        self.correlation_map.insert(cor_id_, ticker.clone());

        let service: &str = (&self.service).into();
        let sub_str = format!("{}/ticker/{}", service, ticker);
        let c_subscription = CString::new(sub_str).expect("CString conversion failed");

        let c_corr_id = corr_id.id;
        let field_strings: Vec<CString> = self
            .fields
            .iter()
            .map(|s| CString::new(*s).unwrap_or_default())
            .collect();
        let c_fields: Vec<*const c_char> = field_strings.iter().map(|s| s.as_ptr()).collect();

        let option_strings: Vec<CString> = self
            .options
            .iter()
            .map(|s| CString::new(*s).unwrap_or_default())
            .collect();
        let c_options: Vec<*const c_char> = option_strings.iter().map(|s| s.as_ptr()).collect();

        let res = unsafe {
            blpapi_SubscriptionList_add(
                self.ptr,
                c_subscription.as_ptr(),
                &c_corr_id, // Ensure this is the actual struct, not a pointer to a pointer
                c_fields.as_ptr() as *mut *const c_char, // Pass the pointer to the array
                c_options.as_ptr() as *mut *const c_char, // Pass the pointer to the array
                c_fields.len(),
                c_options.len(),
            )
        };

        Error::check(res)?;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        let res = unsafe { blpapi_SubscriptionList_clear(self.ptr) };
        Error::check(res)?;
        Ok(())
    }

    pub fn append(&mut self, list: SubscriptionList) -> Result<(), Error> {
        let res = unsafe { blpapi_SubscriptionList_append(list.ptr, self.ptr as *const _) };
        Error::check(res)?;
        Ok(())
    }

    pub fn size(&self) -> usize {
        unsafe { blpapi_SubscriptionList_size(self.ptr as *const _) as usize }
    }
    pub fn get_corr_id(&self, index: usize) -> CorrelationId {
        let ptr = null_mut();
        let res =
            unsafe { blpapi_SubscriptionList_correlationIdAt(self.ptr as *const _, ptr, index) };
        match res == 0 {
            true => {
                let cor_id = unsafe { *ptr };
                CorrelationIdBuilder::default().from_pointer(cor_id)
            }
            false => CorrelationIdBuilder::default().build(),
        }
    }

    pub fn get_topic_string(&self, index: usize) -> String {
        let ptr = null_mut();
        let res = unsafe { blpapi_SubscriptionList_topicStringAt(self.ptr, ptr, index) };
        match res == 0 {
            true => {
                let top_str = unsafe { CStr::from_ptr(*ptr).to_string_lossy().into_owned() };
                top_str
            }
            false => String::from("Invalid Topic String received"),
        }
    }

    pub fn get_resolved(&self, index: usize) -> i32 {
        let ptr = null_mut();
        let res = unsafe { blpapi_SubscriptionList_isResolvedAt(self.ptr, ptr, index) };
        match res == 0 {
            true => ptr as i32,
            false => 99999,
        }
    }
}

impl<'a> Drop for SubscriptionList<'a> {
    fn drop(&mut self) {
        unsafe { blpapi_SubscriptionList_destroy(self.ptr) }
    }
}
