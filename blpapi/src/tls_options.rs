use crate::Error;
use blpapi_sys::{
    blpapi_TlsOptions_copy, blpapi_TlsOptions_create, blpapi_TlsOptions_createFromBlobs,
    blpapi_TlsOptions_createFromFiles, blpapi_TlsOptions_destroy,
    blpapi_TlsOptions_setCrlFetchTimeoutMs, blpapi_TlsOptions_setTlsHandshakeTimeoutMs,
    blpapi_TlsOptions_t,
};
use std::ffi::{c_int, CString};

pub const TLSOPTIONS_DEFAULT_HANDSHAKE_TIMEOUT: isize = 10_000;
pub const TLSOPTIONS_DEFAULT_CRL_FETCH_TIMEOUT: isize = 20_000;

pub trait Duplicate {
    fn duplicate(&self, option: TlsOptions) -> Result<(), Error>;
}

/// Tls Options Struct
#[derive(Debug, PartialEq)]
pub struct TlsOptions {
    pub(crate) ptr: *mut blpapi_TlsOptions_t,
    pub handshake_timeout: isize,
    pub crl_timeout: isize,
}

/// Tls Options File
#[derive(Debug, PartialEq)]
pub struct TlsOptionsFile {
    pub cc_name: String,
    pub cc_password: String,
    pub cert_name: String,
}
#[derive(Debug, PartialEq, Default)]
pub struct TlsOptionsFileBuilder {
    pub cc_name: Option<String>,
    pub cc_password: Option<String>,
    pub cert_name: Option<String>,
}

/// Tls Options Blobs
#[derive(Debug, PartialEq)]
pub struct TlsOptionsBlobs {
    pub cc_raw_data: String,
    pub cc_raw_data_length: isize,
    pub cc_password: String,
    pub cert_raw_data: String,
    pub cert_raw_data_length: isize,
}
#[derive(Debug, PartialEq, Default)]
pub struct TlsOptionsBlobsBuilder {
    pub cc_raw_data: Option<String>,
    pub cc_raw_data_length: Option<isize>,
    pub cc_password: Option<String>,
    pub cert_raw_data: Option<String>,
    pub cert_raw_data_length: Option<isize>,
}

/// Implementing TlsOptions
impl TlsOptions {
    pub fn create_from_files(options: TlsOptionsFile) -> Self {
        let c_name = CString::new(options.cc_name).unwrap();
        let c_password = CString::new(options.cc_password).unwrap();
        let cert_name = CString::new(options.cert_name).unwrap();

        let ptr = unsafe {
            blpapi_TlsOptions_createFromFiles(
                c_name.as_ptr(),
                c_password.as_ptr(),
                cert_name.as_ptr(),
            )
        };
        TlsOptions {
            ptr,
            handshake_timeout: TLSOPTIONS_DEFAULT_HANDSHAKE_TIMEOUT,
            crl_timeout: TLSOPTIONS_DEFAULT_CRL_FETCH_TIMEOUT,
        }
    }

    pub fn create_from_blobs(options: TlsOptionsBlobs) -> Self {
        let cc_raw_data = CString::new(options.cc_raw_data).unwrap();
        let cc_raw_data_length = options.cc_raw_data_length;
        let cc_password = CString::new(options.cc_password).unwrap();
        let cert_raw_data = CString::new(options.cert_raw_data).unwrap();
        let cert_raw_data_length = options.cert_raw_data_length;

        let ptr = unsafe {
            blpapi_TlsOptions_createFromBlobs(
                cc_raw_data.as_ptr(),
                cc_raw_data_length as c_int,
                cc_password.as_ptr(),
                cert_raw_data.as_ptr(),
                cert_raw_data_length as c_int,
            )
        };

        TlsOptions {
            ptr,
            handshake_timeout: TLSOPTIONS_DEFAULT_HANDSHAKE_TIMEOUT,
            crl_timeout: TLSOPTIONS_DEFAULT_CRL_FETCH_TIMEOUT,
        }
    }

    pub fn set_tls_handshake_timeout_ms(&mut self, ms: isize) -> &mut Self {
        self.handshake_timeout = ms;
        unsafe { blpapi_TlsOptions_setTlsHandshakeTimeoutMs(self.ptr, ms as c_int) };
        self
    }

    pub fn set_crl_fetch_timeout_ms(&mut self, ms: isize) -> &mut Self {
        self.crl_timeout = ms;
        unsafe { blpapi_TlsOptions_setCrlFetchTimeoutMs(self.ptr, ms as c_int) };
        self
    }
}

impl Default for TlsOptions {
    fn default() -> Self {
        unsafe {
            TlsOptions {
                ptr: blpapi_TlsOptions_create(),
                handshake_timeout: TLSOPTIONS_DEFAULT_HANDSHAKE_TIMEOUT,
                crl_timeout: TLSOPTIONS_DEFAULT_CRL_FETCH_TIMEOUT,
            }
        }
    }
}

impl Clone for TlsOptions {
    fn clone(&self) -> Self {
        let cloned: TlsOptions = TlsOptions::default();
        unsafe { blpapi_TlsOptions_copy(self.ptr, cloned.ptr) };
        cloned
    }
}

impl Duplicate for TlsOptions {
    fn duplicate(&self, option: TlsOptions) -> Result<(), Error> {
        unsafe { blpapi_TlsOptions_copy(self.ptr, option.ptr) };
        Ok(())
    }
}

impl Drop for TlsOptions {
    fn drop(&mut self) {
        unsafe { blpapi_TlsOptions_destroy(self.ptr) }
    }
}

/// Implementing the TlsOptionsFile
impl TlsOptionsFileBuilder {
    // Setting new name
    pub fn name<T: Into<String>>(mut self, name: T) -> TlsOptionsFileBuilder {
        self.cc_name = Some(name.into());
        self
    }

    // Setting new password
    pub fn password<T: Into<String>>(mut self, password: T) -> TlsOptionsFileBuilder {
        self.cc_password = Some(password.into());
        self
    }

    // Setting new certificate name
    pub fn cert_name<T: Into<String>>(mut self, cert_name: T) -> TlsOptionsFileBuilder {
        self.cert_name = Some(cert_name.into());
        self
    }

    // Building TlsOptionsFile
    pub fn build(self) -> TlsOptionsFile {
        TlsOptionsFile {
            cc_name: self.cc_name.expect("Set name"),
            cc_password: self.cc_password.expect("Set password"),
            cert_name: self.cert_name.expect("Set cert_name"),
        }
    }
}

impl TlsOptionsFile {
    pub fn new<T: Into<String>>(cc_name: T, cc_password: T, cert_name: T) -> Self {
        TlsOptionsFile {
            cc_name: cc_name.into(),
            cc_password: cc_password.into(),
            cert_name: cert_name.into(),
        }
    }
}

impl Default for TlsOptionsFile {
    fn default() -> Self {
        TlsOptionsFile {
            cc_name: String::from("User"),
            cc_password: String::from("Password"),
            cert_name: String::from("Certificate"),
        }
    }
}

/// Implementing the TlsOptionsBlobs
impl TlsOptionsBlobsBuilder {
    // Setting new raw_data
    pub fn cc_raw_data<T: Into<String>>(mut self, data: T) -> TlsOptionsBlobsBuilder {
        self.cc_raw_data = Some(data.into());
        self
    }

    // Setting new password
    pub fn cc_password<T: Into<String>>(mut self, password: T) -> TlsOptionsBlobsBuilder {
        self.cc_password = Some(password.into());
        self
    }

    // Setting new cc raw data length
    pub fn cc_raw_data_length(mut self, data_length: isize) -> TlsOptionsBlobsBuilder {
        self.cc_raw_data_length = Some(data_length);
        self
    }

    // Setting cert raw data
    pub fn cert_raw_data<T: Into<String>>(mut self, data: T) -> TlsOptionsBlobsBuilder {
        self.cert_raw_data = Some(data.into());
        self
    }

    // Seeting cert raw data length
    pub fn cert_raw_data_length(mut self, data_length: isize) -> TlsOptionsBlobsBuilder {
        self.cert_raw_data_length = Some(data_length);
        self
    }

    // Building TlsOptionsBlobs
    pub fn build(self) -> TlsOptionsBlobs {
        TlsOptionsBlobs {
            cc_raw_data: self.cc_raw_data.expect("Set cc raw data"),
            cc_password: self.cc_password.expect("Set cc password"),
            cc_raw_data_length: self.cc_raw_data_length.expect("Set cc raw data length"),
            cert_raw_data: self.cert_raw_data.expect("Set cert raw data"),
            cert_raw_data_length: self.cert_raw_data_length.expect("Set cert raw data length"),
        }
    }
}

impl TlsOptionsBlobs {
    pub fn new<T: Into<String>>(
        cc_raw_data: T,
        cc_password: T,
        cc_raw_data_length: isize,
        cert_raw_data: T,
        cert_raw_data_length: isize,
    ) -> Self {
        TlsOptionsBlobs {
            cc_raw_data: cc_raw_data.into(),
            cc_password: cc_password.into(),
            cc_raw_data_length,
            cert_raw_data: cert_raw_data.into(),
            cert_raw_data_length,
        }
    }
}

impl Default for TlsOptionsBlobs {
    fn default() -> Self {
        TlsOptionsBlobs {
            cc_raw_data: String::from("RawData"),
            cc_password: String::from("Password"),
            cc_raw_data_length: 0,
            cert_raw_data: String::from("Certificate"),
            cert_raw_data_length: 0,
        }
    }
}
