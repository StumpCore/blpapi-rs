use crate::{session::SessionSync, Error};
use blpapi_sys::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use regex::Regex;
use crate::tlsoptions::TlsOptions;

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
pub const BLPAPI_DEFAULT_SERVICE_CHECK_TIMEOUT:isize= 120_000;
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

///Builder for SessionOptions
#[derive(Debug)]
pub struct SessionOptionsBuilder {
    pub ptr: Option<*mut blpapi_SessionOptions_t>,
    pub server_host:Option<String>,
    pub server_port:Option<u16>,
    pub server_index:Option<usize>,
    pub timeout:Option<u32>,
    pub service_check_timeout:Option<isize>,
    pub service_download_timeout:Option<isize>,
    pub session_name: Option<String>,
    pub slow_consumer_warning_high_water_mark:Option<f32>,
    pub slow_consumer_warning_low_water_mark:Option<f32>,
    pub client_mode:Option<ClientMode>,
    pub authentication: Option<Authentication>,
    pub auto_restart:Option<usize>,
    pub multiple_corr_per_msg:Option<usize>,
    pub service:Option<String>,
    pub topic_prefix:Option<String>,
    pub max_pending_request:Option<u16>,
    pub max_attempts:Option<u16>,
    pub max_queue_size:Option<usize>,
    pub keep_alive_inactivity_time:Option<isize>,
    pub keep_alive_response_timeout:Option<isize>,
    pub keep_alive:Option<bool>,
    pub record_subscription:Option<bool>,
    pub flush_published_events_timeout:Option<isize>,
    pub tls_options:Option<TlsOptions>,
}

/// A SessionOptions
#[derive(Debug)]
pub struct SessionOptions {
    pub(crate) ptr: *mut blpapi_SessionOptions_t,
    pub server_host: String,
    pub server_port: u16,
    pub server_index: usize,
    pub timeout: u32,
    pub service_check_timeout: isize,
    pub service_download_timeout: isize,
    pub session_name: String,
    pub slow_consumer_warning_high_water_mark: f32,
    pub slow_consumer_warning_low_water_mark: f32,
    pub client_mode: ClientMode,
    pub authentication: Authentication,
    pub auto_restart: usize,
    pub multiple_corr_per_msg: usize,
    pub service: String,
    pub topic_prefix: String,
    pub max_request_tries: u16,
    pub max_attempts: u16,
    pub max_queue_size: usize,
    pub keep_alive_inactivity_time: isize,
    pub keep_alive_response_timeout: isize,
    pub keep_alive: bool,
    pub record_subscription: bool,
    pub flush_published_events_timeout: isize,
    pub tls_options: TlsOptions,
}

impl SessionOptionsBuilder {
    pub fn new() -> Self {
        // Ensure pointer is set to correct struct
        let ptr:*mut blpapi_SessionOptions_t = unsafe { blpapi_SessionOptions_create()};
        Self {
            ptr:Some(ptr),
            server_host:None,
            server_port:None,
            server_index:None,
            timeout:None,
            service:None,
            topic_prefix:None,
            multiple_corr_per_msg:None,
            auto_restart:None,
            authentication:None,
            max_pending_request:None,
            max_attempts:None,
            max_queue_size:None,
            slow_consumer_warning_high_water_mark:None,
            slow_consumer_warning_low_water_mark:None,
            client_mode:None,
            keep_alive_inactivity_time:None,
            keep_alive_response_timeout:None,
            keep_alive:None,
            service_check_timeout:None,
            record_subscription:None,
            service_download_timeout:None,
            tls_options:None,
            flush_published_events_timeout:None,
            session_name:None,
        }
    }

    /// Setting a new host
    /// In case of an empty or invalid string, nothing will be changed
    /// Invalid strings or empty pointers will cause an Error
    pub fn set_server_host<T: Into<String>>(&mut self, host:T) -> &mut Self {
        let binding = host.into();
        let chost = CString::new(&*binding);
        let res = match chost {
            Ok(val) if !val.is_empty()=>{
                let res = match self.ptr {
                    Some(inner) => {
                        unsafe {
                            blpapi_SessionOptions_setServerHost(
                                inner,
                                val.as_ptr(),
                            ) as i32
                        }
                    }
                    None => 110,
                };

                match Error::check(res) {
                    Ok(_)=> Ok(binding),
                    Err(e)=>Err(Error::session_options(
                        "SessionOptionsBuilder",
                        "host",
                        "Invalid blpapi_SessionOptions_setServerHost call"
                    )),
                }
            },
            _ => {
                let r_string = String::from(BLPAPI_DEFAULT_HOST);
                Ok(r_string)
            },
        };

        match res {
            Ok(val)=>{self.server_host = Some(val);},
            Err(e)=>{
                println!("Error setting server host call: {}", e);
                println!("Setting default host: {}", BLPAPI_DEFAULT_HOST);
                self.server_host = Some(String::from(BLPAPI_DEFAULT_HOST));
            }
        }
        self
    }

    /// Setting a new port
    pub fn set_server_port(&mut self, port: u16) -> &mut Self {
        let res = match self.ptr {
            Some(inner) => unsafe {
                unsafe { blpapi_SessionOptions_setServerPort(inner, port) as i32}
            }
            None => 110,
        };

        let check = match Error::check(res) {
            Ok(_) => Ok(res),
            Err(e)=>Err(Error::session_options(
                "SessionOptionsBuilder",
                "port",
                "Invalid blpapi_SessionOptions_setServerPort call"
            )),
        };

        match check {
            Ok(val)=>{self.server_port= Some(port);},
            Err(e)=>{
                println!("Error setting server port call: {}", e);
                println!("Setting default port: {}", BLPAPI_DEFAULT_PORT);
                self.server_port= Some(BLPAPI_DEFAULT_PORT);
            }
        }
        self
    }

    /// Setting new server address index
    pub fn set_index(&mut self, index:usize) -> &mut Self {
        self.server_index = Some(index);
        self
    }

    /// Setting new server address
    pub fn set_server_address<T: Into<String>>(&mut self, host:T, port:u16, index:usize) -> &mut Self {
        self.set_server_host(host);
        self.set_server_port(port);
        self.set_index(index);
        self
    }

    /// Setting the connection timeout
    pub fn set_connect_timeout(&mut self, ms:u32) -> &mut Self {
        self.timeout = Some(ms);
        self
    }

    /// Setting the Subscription Service
    pub fn set_default_subscription_service<T:Into<String>>(&mut self, service_id:T) -> &mut Self {
        let re = Regex::new(r"^//[-_.a-zA-Z0-9]+/[-_.a-zA-Z0-9]+$").unwrap();
        let id = service_id.into();
        match re.is_match(&id){
            true=>{self.service = Some(id);},
            false=>{
                println!("Invalid subscription service or format. Setting to default: {}", BLPAPI_DEFAULT_SERVICE_IDENTIFIER);
                self.service = Some(BLPAPI_DEFAULT_SERVICE_IDENTIFIER.into());}
        };
        self
    }

    /// Setting the defaul topic prefix
    pub fn set_default_topic_prefix<T: Into<String>>(&mut self, prefix:T) -> &mut Self {
        let re = Regex::new(r"^/([-_.a-zA-Z0-9]+/)?").unwrap();
        let id = prefix.into();
        match re.is_match(&id){
            true=>{self.topic_prefix= Some(id);},
            false=>{
                println!("Invalid topic prefix or format. Setting to default: {}", BLPAPI_DEFAULT_TOPIC_PREFIX);
                self.topic_prefix= Some(BLPAPI_DEFAULT_TOPIC_PREFIX.into());}
        };
        self
    }

    /// Setting allowance of multiple correlation IDs with a message
    pub fn set_allow_multiple_correlators_per_msg(&mut self, allow:bool)-> &mut Self {
        match allow == true {
            true => {self.multiple_corr_per_msg = Some(0 as usize)},
            false => {self.multiple_corr_per_msg = Some(1 as usize)},
        };
        self
    }

    /// Setting Authentication Options
    /// Defaults to OS_LOGON
    pub fn set_authentication_options(&mut self, auth:Authentication) -> &mut Self {
        match auth {
            Authentication::OsLogon=>BLPAPI_AUTHENTICATION_OS_LOGON,
            Authentication::DirectoryService=>BLPAPI_AUTHENTICATION_DIRECTORY_SERVICE,
            Authentication::ApplicationOnly=>BLPAPI_AUTHENTICATION_APPLICATION_ONLY,
            Authentication::AppnameAndKey=>BLPAPI_AUTHENTICATION_APPNAME_AND_KEY,
        };
        self.authentication = Some(auth);
        self
    }

    /// Setting auto restart on disconnect option
    pub fn set_auto_restart_on_disconnect(&mut self, option:bool) -> &mut Self {
        match option== true {
            true => {self.auto_restart = Some(0 as usize)},
            false=> {self.auto_restart = Some(1 as usize)},
        };
        self
    }

    /// Setting max pending requests
    pub fn set_max_pending_requests(&mut self, no:u16) -> &mut Self {
        self.max_pending_request = Some(no);
        self
    }

    /// setting max number of start attempts
    pub fn set_num_start_attempts(&mut self, no:u16) -> &mut Self {
        self.max_attempts = Some(no);
        self
    }

    /// Setting max queue size
    pub fn set_max_event_queue_size(&mut self, no:usize) -> &mut Self {
        self.max_queue_size= Some(no);
        self
    }

    /// Setting the slow consumer warning marks
    pub fn set_both_slow_consumer_warning_marks(&mut self, low:f32, high:f32)-> &mut Self {
        let low = match low >= 0.0 && low <= 1.0 {
            true=>low,
            false=>BLPAPI_DEFAULT_LOW_WATER_MARK,
        };
        let high= match high>= 0.0 && high<= 1.0 {
            true=>high,
            false=>BLPAPI_DEFAULT_HIGH_WATER_MARK,
        };
        match high > low {
            true => {
                self.slow_consumer_warning_high_water_mark = Some(high);
                self.slow_consumer_warning_low_water_mark = Some(low);
            },
            false =>{
                println!("Slow consumer warning high water mark are invalid");
                println!("High Water Mark: {}", high);
                println!("Low Water Mark: {}", low);
                println!("Setting default values for both");
                self.slow_consumer_warning_high_water_mark = Some(BLPAPI_DEFAULT_HIGH_WATER_MARK);
                self.slow_consumer_warning_low_water_mark = Some(BLPAPI_DEFAULT_LOW_WATER_MARK);
            }
        };
        self
    }

    /// Setting Client Mode
    pub fn set_client_mode(&mut self, mode: ClientMode)-> &mut Self {
        match mode {
            ClientMode::Auto => BLPAPI_CLIENTMODE_AUTO,
            ClientMode::DApi => BLPAPI_CLIENTMODE_DAPI,
            ClientMode::SApi => BLPAPI_CLIENTMODE_SAPI,
            ClientMode::Compat33X => BLPAPI_CLIENTMODE_COMPAT_33X,
        };
        self.client_mode = Some(mode);
        self
    }

    /// Setting keep alive
    pub fn set_keep_alive(&mut self, enable:bool) -> &mut Self {
        self.keep_alive = Some(enable);
        self
    }

    /// Setting default keep alive inactivity time
    pub fn set_default_keep_alive_inactivity_time(&mut self, ms:isize)-> &mut Self {
        match ms>=0 {
            true=>self.keep_alive_inactivity_time = Some(ms),
            false=>self.keep_alive_inactivity_time = Some(BLPAPI_DEFAULT_KEEP_ALIVE_INACTIVITY_TIME),
        };
        self
    }

    /// Setting default keep alive response timeout
    pub fn set_default_keep_alive_response_timeout(&mut self, ms:isize)-> &mut Self {
        match ms>=0 {
            true=>self.keep_alive_response_timeout= Some(ms),
            false=>self.keep_alive_response_timeout= Some(BLPAPI_DEFAULT_KEEP_ALIVE_RESPONSE_TIMEOUT),
        };
        self
    }

    /// Setting record subscription data receive times
    pub fn set_record_subscription_data_receive_times(&mut self, record:bool)-> &mut Self {
        self.record_subscription = Some(record);
        self
    }

    /// Setting the service check timeout ms
    pub fn set_service_check_timeout(&mut self, ms:isize)-> &mut Self {
        match ms>=0 {
            true=>self.service_check_timeout= Some(ms),
            false=>self.service_check_timeout= Some(BLPAPI_DEFAULT_SERVICE_CHECK_TIMEOUT),
        };
        self
    }

    /// Setting service download timeout
    pub fn set_service_download_timeout(&mut self, ms:isize)-> &mut Self {
        match ms>=0 {
            true=>self.service_download_timeout= Some(ms),
            false=>self.service_download_timeout= Some(BLPAPI_DEFAULT_SERVICE_DOWNLOAD_TIMEOUT),
        };
        self
    }

    /// Setting tls options
    pub fn set_tls_options(&mut self, options:TlsOptions) -> &mut Self {
        self.tls_options = Some(options);
        self
    }

    /// Setting flush publish events timeout
    pub fn set_flush_published_events_timeout(&mut self, ms:isize)-> &mut Self {
        match ms>=0 {
            true=>self.flush_published_events_timeout= Some(ms),
            false=>self.flush_published_events_timeout= Some(BLPAPI_DEFAULT_FLUSH_PUBLISHED_EVENTS_TIMEOUT),
        };
        self
    }

    /// Setting session name
    pub fn set_session_name<T:Into<String>>(&mut self, name:T)-> &mut Self {
        self.session_name = Some(name.into());
        self
    }

    /// Builder function
    pub fn build(self) -> SessionOptions{
        SessionOptions{
            ptr: self.ptr.expect("Expected pointer"),
            server_host:self.server_host.expect("Expected server host"),
            server_port:self.server_port.expect("Expected server port"),
            server_index:self.server_index.expect("Expected server index"),
            timeout:self.timeout.expect("Expected timeout"),
            service:self.service.expect("Expected subscription service"),
            topic_prefix:self.topic_prefix.expect("Expected topic prefix"),
            multiple_corr_per_msg:self.multiple_corr_per_msg.expect("Expect multiple_corr_per_msg"),
            client_mode:self.client_mode.expect("Expected client mode"),
            authentication:self.authentication.expect("Expected authentication option"),
            auto_restart:self.auto_restart.expect("Expected auto restart"),
            max_request_tries:self.max_pending_request.expect("Expected max_request_tries"),
            max_attempts:self.max_attempts.expect("Expected max_attempts"),
            max_queue_size:self.max_queue_size.expect("Expected max queue size"),
            slow_consumer_warning_low_water_mark:self.slow_consumer_warning_low_water_mark.expect("Expected low water mark"),
            slow_consumer_warning_high_water_mark:self.slow_consumer_warning_high_water_mark.expect("Expected high water mark"),
            keep_alive:self.keep_alive.expect("Expected keep alive"),
            keep_alive_inactivity_time:self.keep_alive_inactivity_time.expect("Expected keep alive inactivity time"),
            keep_alive_response_timeout:self.keep_alive_response_timeout.expect("Expect keep alive response timeout"),
            record_subscription:self.record_subscription.expect("Expected record subscription"),
            service_check_timeout:self.service_check_timeout.expect("Expected service check timeout"),
            service_download_timeout:self.service_download_timeout.expect("Expected service download timeout"),
            flush_published_events_timeout:self.flush_published_events_timeout.expect("Expected flush published events timeout"),
            session_name:self.session_name.expect("Expected session name"),
            tls_options:self.tls_options.expect("Expected TLS options"),
        }
    }
}

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
