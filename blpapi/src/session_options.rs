use crate::{session::SessionSync, Error};
use blpapi_sys::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_int;

/// Const Values
pub const BLPAPI_DEFAULT_HOST: &'static str = "127.0.0.1";
pub const BLPAPI_DEFAULT_PORT: u16= 8_194;
pub const BLPAPI_DEFAULT_INDEX: usize = 0;
pub const BLPAPI_DEFAULT_AUTO_RESTART:usize = 0;
pub const BLPAPI_DEFAULT_TIMEOUT:u32=5_000;
pub const BLPAPI_DEFAULT_MULTIPLE_CORR_PER_MSG: usize= 0;
pub const BLPAPI_DEFAULT_SERVICE_IDENTIFIER: &'static str = "//blp/mktdata";
pub const BLPAPI_DEFAULT_TOPIC_PREFIX: &'static str = "/ticker/";
pub const BLPAPI_AUTHENTICATION_OS_LOGON: &'static str = "OS_LOGON";
pub const BLPAPI_AUTHENTICATION_DIRECTORY_SERVICE: &'static str = "DIRECTORY_SERVICE";
pub const BLPAPI_AUTHENTICATION_APPLICATION_ONLY: &'static str = "APPLICATION_ONLY";
pub const BLPAPI_AUTHENTICATION_APPNAME_AND_KEY: &'static str = "APPNAME_AND_KEY";
pub const BLPAPI_DEFAULT_MAX_PENDING_REQUEST:u16 = 1024;
pub const BLPAPI_DEFAULT_MAX_START_ATTEMPTS:u16=1;
pub const BLPAPI_DEFAULT_MAX_EVENT_QUEUE_SIZE:usize = 10_000;
pub const BLPAPI_DEFAULT_HIGH_WATER_MARK:f32 = 0.75;
pub const BLPAPI_DEFAULT_LOW_WATER_MARK:f32 = 0.50;
pub const BLPAPI_DEFAULT_KEEP_ALIVE_INACTIVITY_TIME: isize = 20_000;
pub const BLPAPI_DEFAULT_KEEP_ALIVE_RESPONSE_TIMEOUT:isize = 5_000;
pub const BLPAPI_DEFAULT_KEEP_ALIVE:bool = true;
pub const BLPAPI_DEFAULT_RECORD_SUBSCRIPTION:bool=false;
pub const BLPAPI_DEFAULT_SERVICE_CHECK_TIMEOUT:isize= 60_000;
pub const BLPAPI_DEFAULT_SERVICE_DOWNLOAD_TIMEOUT:isize = 120_000;
pub const BLPAPI_DEFAULT_FLUSH_PUBLISHED_EVENTS_TIMEOUT:isize=2_000;

/// ClientMode
#[derive(Debug, Clone, Copy)]
pub enum ClientMode {
    /// Automatic
    Auto,
    /// Desktop API
    DApi,
    /// Server API
    SApi,
    /// Compat 33X
    Compat33X,
}

///Authentication
#[derive(Debug, Clone, Copy)]
pub enum Authentication {
    // user only
    OsLogon,
    DirectoryService,
    // application only
    ApplicationOnly,
    AppnameAndKey,
}


/// A SessionOptions
///
/// Behaves like a `Session` builder
///
pub struct SessionOptions(pub(crate) *mut blpapi_SessionOptions_t);

impl SessionOptions {
    /// Get client mode
    pub fn client_mode(&self) -> Result<ClientMode, Error> {
        let mode = unsafe { blpapi_SessionOptions_clientMode(self.0) };
        Error::check(mode)?;
        match mode as u32 {
            BLPAPI_CLIENTMODE_AUTO => Ok(ClientMode::Auto),
            BLPAPI_CLIENTMODE_DAPI => Ok(ClientMode::DApi),
            BLPAPI_CLIENTMODE_SAPI => Ok(ClientMode::SApi),
            BLPAPI_CLIENTMODE_COMPAT_33X => Ok(ClientMode::Compat33X),
            _ => Err(Error::Generic(mode)),
        }
    }

    /// Set client mode
    pub fn set_client_mode(&mut self, mode: ClientMode) {
        let mode = match mode {
            ClientMode::Auto => BLPAPI_CLIENTMODE_AUTO,
            ClientMode::DApi => BLPAPI_CLIENTMODE_DAPI,
            ClientMode::SApi => BLPAPI_CLIENTMODE_SAPI,
            ClientMode::Compat33X => BLPAPI_CLIENTMODE_COMPAT_33X,
        };
        unsafe {
            blpapi_SessionOptions_setClientMode(self.0, mode as c_int);
        }
    }

    /// Get server host
    pub fn server_host(&self) -> String {
        let chost = unsafe { CStr::from_ptr(blpapi_SessionOptions_serverHost(self.0)) };
        chost.to_string_lossy().into_owned()
    }

    /// Set server host
    pub fn with_server_host(self, host: &str) -> Result<Self, Error> {
        let chost = CString::new(host).unwrap();
        let res = unsafe { blpapi_SessionOptions_setServerHost(self.0, chost.as_ptr()) };
        Error::check(res)?;
        Ok(self)
    }

    /// Get server port
    pub fn server_port(&self) -> u16 {
        unsafe { blpapi_SessionOptions_serverPort(self.0) as u16 }
    }

    /// Set server port
    pub fn with_server_port(self, port: u16) -> Result<Self, Error> {
        let res = unsafe { blpapi_SessionOptions_setServerPort(self.0, port) };
        Error::check(res)?;
        Ok(self)
    }

    /// Build a session, transfer ownership
    pub fn sync(self) -> SessionSync {
        SessionSync::from_options(self)
    }
}

impl Drop for SessionOptions {
    fn drop(&mut self) {
        unsafe { blpapi_SessionOptions_destroy(self.0) }
    }
}

impl Clone for SessionOptions {
    fn clone(&self) -> Self {
        let cloned = SessionOptions::default();
        unsafe {
            blpapi_SessionOptions_copy(self.0, cloned.0);
        }
        cloned
    }
}

impl Default for SessionOptions {
    fn default() -> Self {
        unsafe { SessionOptions(blpapi_SessionOptions_create()) }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_host() {
        let host = "localhost";
        let options = SessionOptions::default().with_server_host(host).unwrap();
        assert_eq!(host, options.server_host());
        let session = options.sync();
    }
}
