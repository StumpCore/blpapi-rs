use blpapi_sys::{blpapi_AbstractSession_cancel, blpapi_AbstractSession_t, blpapi_CorrelationId_t};
use std::ffi::{c_char, c_int};

use crate::{correlation_id::CorrelationId, Error};

pub trait AbstractSession: Sized {
    /// Return the raw pointer
    fn as_abstract_ptr(&self) -> *mut blpapi_AbstractSession_t;

    /// Cancel the session
    fn cancel(
        &mut self,
        corr_ids: &[CorrelationId],
        request_label: Option<&str>,
    ) -> Result<(), Error> {
        let session_ptr: *mut blpapi_AbstractSession_t = self.as_abstract_ptr();
        let corr_id = corr_ids.as_ptr() as *const blpapi_CorrelationId_t;
        let corr_id_len = corr_ids.len();
        let (label_ptr, label_len) = if let Some(label) = request_label {
            (label.as_ptr() as *const c_char, label.len() as c_int)
        } else {
            // Null pointer and zero length if no label is provided
            (std::ptr::null(), 0)
        };
        let res = unsafe {
            blpapi_AbstractSession_cancel(session_ptr, corr_id, corr_id_len, label_ptr, label_len)
        };
        if res != 0 {
            return Err(Error::Session);
        }
        Ok(())
    }
}
