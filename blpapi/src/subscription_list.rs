use std::{
    ffi::{CStr, CString},
    ptr::null_mut,
};

use blpapi_sys::{
    blpapi_SubscriptionList_add, blpapi_SubscriptionList_append, blpapi_SubscriptionList_clear,
    blpapi_SubscriptionList_correlationIdAt, blpapi_SubscriptionList_create,
    blpapi_SubscriptionList_destroy, blpapi_SubscriptionList_isResolvedAt,
    blpapi_SubscriptionList_size, blpapi_SubscriptionList_t, blpapi_SubscriptionList_topicStringAt,
};

use crate::{
    correlation_id::{CorrelationId, CorrelationIdBuilder},
    Error,
};

/// SubscriptionListBuilder Struct

#[derive(Clone, Debug)]
pub struct SubscriptionListBuilder {
    pub correlation_id: Option<CorrelationId>,
    pub fields: Option<String>,
    pub options: Option<String>,
    pub no_fields: usize,
    pub no_options: usize,
}

impl SubscriptionListBuilder {
    /// Get new correlation id
    pub fn correlation_id(mut self, id: CorrelationId) -> Self {
        self.correlation_id = Some(id);
        self
    }

    /// Get new fields list
    pub fn fields(mut self, new_fields: Vec<&str>) -> Self {
        let no_fields = new_fields.len();
        let fields_str: String = new_fields.join(",").to_string();
        self.fields = Some(fields_str);
        self.no_fields = no_fields;
        self
    }

    /// Get new options list
    pub fn options(mut self, new_options: Vec<&str>) -> Self {
        let no_options = new_options.len();
        let options_str: String = new_options.into_iter().map(|i| format!("&{}", i)).collect();
        self.options = Some(options_str);
        self.no_options = no_options;
        self
    }

    pub fn build(self) -> SubscriptionList {
        let ptr = unsafe { blpapi_SubscriptionList_create() };
        let correlation_id = match self.correlation_id {
            Some(cid) => cid,
            None => CorrelationIdBuilder::default().build(),
        };
        let fields = self.fields.unwrap_or_default();
        let options = self.options.unwrap_or_default();
        let no_fields = self.no_fields;
        let no_options = self.no_options;
        SubscriptionList {
            ptr,
            correlation_id,
            fields,
            options,
            no_fields,
            no_options,
        }
    }
}

/// Subscription List Struct
#[derive(Clone, Debug)]
pub struct SubscriptionList {
    pub(crate) ptr: *mut blpapi_SubscriptionList_t,
    pub correlation_id: CorrelationId,
    pub fields: String,
    pub options: String,
    pub no_fields: usize,
    pub no_options: usize,
}

impl SubscriptionList {
    pub fn new(self, correlation_id: CorrelationId, fields: String, options: String) -> Self {
        let no_fields = fields.split(",").collect::<Vec<&str>>().len();
        let no_options = fields.split(";").collect::<Vec<&str>>().len();
        let ptr = unsafe { blpapi_SubscriptionList_create() };

        Self {
            ptr,
            correlation_id,
            fields,
            options,
            no_fields,
            no_options,
        }
    }

    pub fn add(&self, ticker: String) -> Result<(), Error> {
        let c_fields = CString::new(self.fields.clone()).unwrap_or_default();
        let c_options = CString::new(self.options.clone()).unwrap_or_default();
        let c_corr_id = self.correlation_id.id;
        let subscription = format!("//ticker/{}", ticker);
        let subscription = CString::new(subscription).unwrap_or_default();
        let res = unsafe {
            blpapi_SubscriptionList_add(
                self.ptr,
                subscription.as_ptr(),
                &c_corr_id,
                &mut c_fields.as_ptr(),
                &mut c_options.as_ptr(),
                self.no_fields,
                self.no_options,
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

impl Drop for SubscriptionList {
    fn drop(&mut self) {
        unsafe { blpapi_SubscriptionList_destroy(self.ptr) }
    }
}
