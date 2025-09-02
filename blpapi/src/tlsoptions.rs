use std::ffi::{c_int, CString};
use crate::Error;
use blpapi_sys::{blpapi_TlsOptions,
                 blpapi_TlsOptions_copy,
                 blpapi_TlsOptions_create,
                 blpapi_TlsOptions_createFromBlobs,
                 blpapi_TlsOptions_createFromFiles,
                 blpapi_TlsOptions_destroy,
                 blpapi_TlsOptions_setCrlFetchTimeoutMs,
                 blpapi_TlsOptions_setTlsHandshakeTimeoutMs};

pub const TLSOPTIONS_DEFAULT_HANDSHAKE_TIMEOUT:isize=10_000;
pub const TLSOPTIONS_DEFAULT_CRL_FETCH_TIMEOUT:isize=20_000;

pub trait Duplicate {
    fn duplicate<'a>(&'a self, option:TlsOptions<>)->Result<(),Error>;
}

/// Tls Options Struct
#[derive(Debug, PartialEq)]
pub struct TlsOptions{
    pub(crate) ptr: *mut blpapi_TlsOptions,
    handshake_timeout:isize,
    crl_timeout:isize,
}

/// Tls Options File
#[derive(Debug, PartialEq)]
pub struct TlsOptionsFile{
    cc_name:String,
    cc_password:String,
    cert_name:String,
}
#[derive(Debug, PartialEq)]
pub struct TlsOptionsFileBuilder{
    cc_name:Option<String>,
    cc_password:Option<String>,
    cert_name:Option<String>,
}

/// Tls Options Blobs
#[derive(Debug, PartialEq)]
pub struct TlsOptionsBlobs{
    cc_raw_data:String,
    cc_raw_data_length:isize,
    cc_password:String,
    cert_raw_data:String,
    cert_raw_data_length:isize,
}
#[derive(Debug, PartialEq)]
pub struct TlsOptionsBlobsBuilder{
    cc_raw_data:Option<String>,
    cc_raw_data_length:Option<isize>,
    cc_password:Option<String>,
    cert_raw_data:Option<String>,
    cert_raw_data_length:Option<isize>,
}

/// Implementing TlsOptions
impl TlsOptions {
    pub fn create_from_files(options:TlsOptionsFile)-> Self {
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
        TlsOptions{
            ptr,
            handshake_timeout:TLSOPTIONS_DEFAULT_HANDSHAKE_TIMEOUT,
            crl_timeout:TLSOPTIONS_DEFAULT_CRL_FETCH_TIMEOUT,
        }
    }

    pub fn create_from_blobs(options:TlsOptionsBlobs) -> Self {
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

        TlsOptions{
            ptr,
            handshake_timeout:TLSOPTIONS_DEFAULT_HANDSHAKE_TIMEOUT,
            crl_timeout:TLSOPTIONS_DEFAULT_CRL_FETCH_TIMEOUT,
        }
    }

    pub fn set_tls_handshake_timeout_ms(&mut self, ms:isize) -> &mut Self {
        self.handshake_timeout = ms;
        unsafe {
            blpapi_TlsOptions_setTlsHandshakeTimeoutMs(
                self.ptr,
                ms as c_int,
            )
        };
        self
    }

    pub fn set_crl_fetch_timeout_ms(&mut self, ms:isize) -> &mut Self {
        self.crl_timeout = ms;
        unsafe {
            blpapi_TlsOptions_setCrlFetchTimeoutMs(
                self.ptr,
                ms as c_int,
            )
        };
        self
    }
}

impl Default for TlsOptions{
    fn default() -> Self {
        unsafe {
            TlsOptions{
                ptr: blpapi_TlsOptions_create(),
                handshake_timeout:TLSOPTIONS_DEFAULT_HANDSHAKE_TIMEOUT,
                crl_timeout:TLSOPTIONS_DEFAULT_CRL_FETCH_TIMEOUT,
            }
        }
    }
}

impl Clone for TlsOptions{
    fn clone(&self) -> Self {
        let cloned:TlsOptions = TlsOptions::default();
        unsafe{ blpapi_TlsOptions_copy(
            self.ptr,
            cloned.ptr
        )};
        cloned
    }
}

impl Duplicate for TlsOptions{
    fn duplicate<'a>(&'a self, option: TlsOptions) -> Result<(), Error> {
        let res = unsafe{
            blpapi_TlsOptions_copy(self.ptr, option.ptr)
        };
        Ok(res)
    }
}

impl Drop for TlsOptions{
    fn drop(&mut self){
        unsafe {
            blpapi_TlsOptions_destroy(self.ptr)
        }
    }
}


/// Implementing the TlsOptionsFile
impl TlsOptionsFileBuilder{
    // Creating new instance of TlsOptionsFileBuilder
    pub fn new() -> TlsOptionsFileBuilder {
        TlsOptionsFileBuilder{
            cc_name:None,
            cc_password:None,
            cert_name:None,
        }
    }

    // Setting new name
    pub fn name<T:Into<String>>(mut self, name:T) -> TlsOptionsFileBuilder{
        self.cc_name = Some(name.into());
        self
    }

    // Setting new password
    pub fn password<T:Into<String>>(mut self, password:T) -> TlsOptionsFileBuilder{
        self.cc_password = Some(password.into());
        self
    }

    // Setting new certificate name
    pub fn cert_name<T:Into<String>>(mut self, cert_name:T) -> TlsOptionsFileBuilder{
        self.cert_name = Some(cert_name.into());
        self
    }

    // Building TlsOptionsFile
    pub fn build(self)->TlsOptionsFile{
        TlsOptionsFile{
            cc_name:self.cc_name.expect("Set name"),
            cc_password:self.cc_password.expect("Set password"),
            cert_name:self.cert_name.expect("Set cert_name"),
        }
    }

}

impl TlsOptionsFile{
    pub fn new<T:Into<String>>(cc_name:T, cc_password:T, cert_name:T) -> Self{
        TlsOptionsFile{
            cc_name:cc_name.into(),
            cc_password:cc_password.into(),
            cert_name:cert_name.into(),
        }
    }
}

impl Default for TlsOptionsFile{
    fn default() -> Self {
        TlsOptionsFile{
            cc_name:String::from("User"),
            cc_password:String::from("Password"),
            cert_name:String::from("Certificate"),
        }
    }
}

/// Implementing the TlsOptionsBlobs
impl TlsOptionsBlobsBuilder{
    // Creating new instance of TlsOptionsFileBuilder
    pub fn new() -> TlsOptionsBlobsBuilder {
        TlsOptionsBlobsBuilder{
            cc_raw_data:None,
            cc_raw_data_length:None,
            cc_password:None,
            cert_raw_data:None,
            cert_raw_data_length:None,
        }
    }

    // Setting new raw_data
    pub fn cc_raw_data<T:Into<String>>(mut self, data:T) -> TlsOptionsBlobsBuilder{
        self.cc_raw_data= Some(data.into());
        self
    }

    // Setting new password
    pub fn cc_password<T:Into<String>>(mut self, password:T) -> TlsOptionsBlobsBuilder{
        self.cc_password = Some(password.into());
        self
    }

    // Setting new cc raw data length
    pub fn cc_raw_data_length(mut self, data_length:isize) -> TlsOptionsBlobsBuilder{
        self.cc_raw_data_length = Some(data_length);
        self
    }

    // Setting cert raw data
    pub fn cert_raw_data<T:Into<String>>(mut self, data:T) -> TlsOptionsBlobsBuilder{
        self.cert_raw_data = Some(data.into());
        self
    }

    // Seeting cert raw data length
    pub fn cert_raw_data_length(mut self, data_length:isize) -> TlsOptionsBlobsBuilder{
        self.cert_raw_data_length = Some(data_length);
        self
    }

    // Building TlsOptionsBlobs
    pub fn build(self)->TlsOptionsBlobs{
        TlsOptionsBlobs{
            cc_raw_data:self.cc_raw_data.expect("Set cc raw data"),
            cc_password:self.cc_password.expect("Set cc password"),
            cc_raw_data_length:self.cc_raw_data_length.expect("Set cc raw data length"),
            cert_raw_data:self.cert_raw_data.expect("Set cert raw data"),
            cert_raw_data_length:self.cert_raw_data_length.expect("Set cert raw data length"),
        }
    }
}

impl TlsOptionsBlobs{
    pub fn new<T:Into<String>>(cc_raw_data:T, cc_password:T, cc_raw_data_length:isize,cert_raw_data:T, cert_raw_data_length:isize) -> Self{
        TlsOptionsBlobs{
            cc_raw_data:cc_raw_data.into(),
            cc_password:cc_password.into(),
            cc_raw_data_length:cc_raw_data_length,
            cert_raw_data:cert_raw_data.into(),
            cert_raw_data_length:cert_raw_data_length,
        }
    }
}

impl Default for TlsOptionsBlobs{
    fn default() -> Self {
        TlsOptionsBlobs{
            cc_raw_data:String::from("RawData"),
            cc_password:String::from("Password"),
            cc_raw_data_length:0,
            cert_raw_data:String::from("Certificate"),
            cert_raw_data_length:0,
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_tlsoptions_default() {
        let _options = TlsOptions::default();
    }

    #[test]
    fn test_tlsoptions_clone() {
        let options = TlsOptions::default();
        let _clone = options.clone();
    }

    #[test]
    fn test_tlsoptions_duplicate() {
        let options = TlsOptions::default();
        let _clone = options.duplicate(TlsOptions::default());
    }

    #[test]
    fn test_tlsoptions_drop() {
        let options = TlsOptions::default();
        drop(options);
    }

    #[test]
    fn test_tlsoptions_filebuilder(){
        let new_cert = "New Cert";
        let new_pw = "New Pw";
        let new_name = "New Name";

        let builder = TlsOptionsFileBuilder::new();
        let tls = builder
            .cert_name(new_cert)
            .name(new_name)
            .password(new_pw);

        assert_eq!(tls.cert_name.unwrap(), new_cert);
        assert_eq!(tls.cc_name.unwrap(), new_name);
        assert_eq!(tls.cc_password.unwrap(), new_pw);

        let builder = TlsOptionsFileBuilder::new();
        let tls = builder
            .cert_name(new_cert)
            .name(new_name)
            .password(new_pw);

        let tls = tls.build();
    }

    #[test]
    fn test_tlsoptions_from_files(){
        let tls= TlsOptionsFile::new(
            "test",
            "testpw",
            "testcert"
        );
        let _options = TlsOptions::create_from_files(tls);
    }

    #[test]
    fn test_tlsoptions_blobs() {
        let blobs = TlsOptionsBlobs::default();
        let _options = TlsOptions::create_from_blobs(blobs);
    }

    #[test]
    fn test_tlsoptions_blobsbuilder(){
        let new_raw_data = "New Raw Data";
        let new_raw_len= 10 as isize;
        let new_pw = "New Pw";
        let new_raw_cert= "New Cert Data";
        let new_raw_cert_len= 15 as isize;

        let builder = TlsOptionsBlobsBuilder::new();
        let tls = builder
            .cert_raw_data(new_raw_cert)
            .cert_raw_data_length(new_raw_cert_len)
            .cc_raw_data(new_raw_data)
            .cc_raw_data_length(new_raw_len)
            .cc_password(new_pw);

        assert_eq!(tls.cc_raw_data.unwrap(), new_raw_data);
        assert_eq!(tls.cc_password.unwrap(), new_pw);
        assert_eq!(tls.cc_raw_data_length.unwrap(), new_raw_len);
        assert_eq!(tls.cert_raw_data.unwrap(), new_raw_cert);
        assert_eq!(tls.cert_raw_data_length.unwrap(), new_raw_cert_len);

        let builder = TlsOptionsBlobsBuilder::new();
        let tls = builder
            .cert_raw_data(new_raw_cert)
            .cert_raw_data_length(new_raw_cert_len)
            .cc_raw_data(new_raw_data)
            .cc_raw_data_length(new_raw_len)
            .cc_password(new_pw);

        let _options =tls.build();
    }

    #[test]
    fn test_tlsoptions_set_handshake() {
        let new_ms = 23_000;
        let mut tlsoptions = TlsOptions::default();
        tlsoptions.set_tls_handshake_timeout_ms(new_ms);
        assert_eq!(tlsoptions.handshake_timeout, new_ms)
    }

    #[test]
    fn test_tlsoptions_set_fetch_timeout() {
        let new_ms = 23_000;
        let mut tlsoptions = TlsOptions::default();
        tlsoptions.set_crl_fetch_timeout_ms(new_ms);
        assert_eq!(tlsoptions.crl_timeout, new_ms)
    }

}











