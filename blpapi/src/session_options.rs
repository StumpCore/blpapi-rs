use crate::core::*;
use crate::socks_5_config::Socks5Config;
use crate::tlsoptions::TlsOptions;
use crate::{session::SessionSync, Error};
use blpapi_sys::*;
use regex::Regex;
use std::ffi::{c_char, c_uint, c_ushort, c_void, CStr, CString};
use std::io::Write;
use std::os::raw::c_int;
use std::ptr;

/// Server Address
#[derive(Debug, Clone)]
pub struct ServerAddress {
    pub host: String,
    pub port: u16,
    pub index: usize,
    pub socks_5_config: Option<Socks5Config>,
    pub socks_5_host: Option<String>,
    pub socks_5_port: Option<u16>,
}

/// ClientMode
#[derive(Debug, Clone, Copy, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub server_host: Option<String>,
    pub server_port: Option<u16>,
    pub server_index: Option<usize>,
    pub server_addresses: Option<Vec<ServerAddress>>,
    pub timeout: Option<u32>,
    pub service_check_timeout: Option<isize>,
    pub service_download_timeout: Option<isize>,
    pub session_name: Option<String>,
    pub slow_consumer_warning_high_water_mark: Option<f32>,
    pub slow_consumer_warning_low_water_mark: Option<f32>,
    pub client_mode: Option<ClientMode>,
    pub authentication: Option<Authentication>,
    pub auto_restart: Option<usize>,
    pub multiple_corr_per_msg: Option<usize>,
    pub services: Option<Vec<String>>,
    pub topic_prefix: Option<String>,
    pub max_pending_request: Option<u16>,
    pub max_start_attempts: Option<u16>,
    pub max_queue_size: Option<usize>,
    pub keep_alive_inactivity_time: Option<isize>,
    pub keep_alive_response_timeout: Option<isize>,
    pub keep_alive: Option<bool>,
    pub record_subscription: Option<bool>,
    pub flush_published_events_timeout: Option<isize>,
    pub tls_options: Option<TlsOptions>,
    pub bandwidth_save_mode: Option<bool>,
    pub application_identifier: Option<String>,
    pub socks_5_config: Option<Socks5Config>,
}

/// A SessionOptions
#[derive(Debug)]
pub struct SessionOptions {
    pub(crate) ptr: *mut blpapi_SessionOptions_t,
    pub server_host: String,
    pub server_port: u16,
    pub server_index: usize,
    pub server_addresses: Vec<ServerAddress>,
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
    pub services: Vec<String>,
    pub topic_prefix: String,
    pub max_pending_request: u16,
    pub max_start_attempts: u16,
    pub max_queue_size: usize,
    pub keep_alive_inactivity_time: isize,
    pub keep_alive_response_timeout: isize,
    pub keep_alive: bool,
    pub record_subscription: bool,
    pub flush_published_events_timeout: isize,
    pub tls_options: TlsOptions,
    pub bandwidth_save_mode: bool,
    pub application_identifier: String,
    pub socks_5_config: Option<Socks5Config>,
}

impl SessionOptionsBuilder {
    pub fn new() -> Self {
        // Ensure pointer is set to correct struct
        let ptr: *mut blpapi_SessionOptions_t = unsafe { blpapi_SessionOptions_create() };
        Self {
            ptr: Some(ptr),
            server_host: None,
            server_port: None,
            server_index: None,
            server_addresses: None,
            timeout: None,
            services: None,
            topic_prefix: None,
            multiple_corr_per_msg: None,
            auto_restart: None,
            authentication: None,
            max_pending_request: None,
            max_start_attempts: None,
            max_queue_size: None,
            slow_consumer_warning_high_water_mark: None,
            slow_consumer_warning_low_water_mark: None,
            client_mode: None,
            keep_alive_inactivity_time: None,
            keep_alive_response_timeout: None,
            keep_alive: None,
            service_check_timeout: None,
            record_subscription: None,
            service_download_timeout: None,
            tls_options: None,
            flush_published_events_timeout: None,
            session_name: None,
            bandwidth_save_mode: None,
            application_identifier: None,
            socks_5_config: None,
        }
    }

    /// Setting a new host
    /// In case of an empty or invalid string, nothing will be changed
    /// Invalid strings or empty pointers will cause an Error
    pub fn set_server_host<T: Into<String>>(mut self, host: T) -> Self {
        self.server_host = Some(host.into());
        self
    }

    /// Setting a new port
    pub fn set_server_port(mut self, port: u16) -> Self {
        self.server_port = Some(port);
        self
    }

    /// Setting new server address index
    pub fn set_index(mut self, index: usize) -> Self {
        self.server_index = Some(index);
        self
    }

    /// Setting new server address
    pub fn set_server_address<T: Into<String>>(mut self, host: T, port: u16, index: usize) -> Self {
        let host_string = host.into().clone();
        let new_address_host = host_string.clone();
        if self.server_host.is_none() && self.server_port.is_none() && self.server_index.is_none() {
            self = self.set_server_host(host_string);
            self = self.set_server_port(port);
            self = self.set_index(index);
        }
        let new_address = ServerAddress {
            host: new_address_host.into(),
            port,
            index,
            socks_5_config: None,
            socks_5_host: None,
            socks_5_port: None,
        };
        if self.server_addresses.is_none() {
            self.server_addresses = Some(vec![new_address]);
        } else {
            let mut current_addresses = self.server_addresses.expect("server_addresses is None");
            current_addresses.push(new_address);
            self.server_addresses = Some(current_addresses);
        };
        self
    }

    /// Setting new server Address from Socks5config
    pub fn set_server_address_socks5config(mut self, socks5config: Socks5Config) -> Self {
        self.socks_5_config = Some(socks5config);
        self
    }

    /// Setting the connection timeout
    pub fn set_connect_timeout(mut self, ms: u32) -> Self {
        self.timeout = Some(ms);
        self
    }

    /// Setting the Subscription Service
    pub fn set_default_subscription_service<T: Into<String>>(mut self, service_id: T) -> Self {
        let re = Regex::new(r"^//[-_.a-zA-Z0-9]+/[-_.a-zA-Z0-9]+$").unwrap();
        let id = service_id.into();
        match re.is_match(&id) {
            true => {
                if self.services.is_none() {
                    self.services = Some(vec![id]);
                } else {
                    let mut cur_services = self.services.expect("services is None");
                    cur_services.push(id);
                    self.services = Some(cur_services);
                };
            }
            false => {
                println!("Invalid subscription service or format.");
            }
        };
        self
    }

    /// Setting the defaul topic prefix
    pub fn set_default_topic_prefix<T: Into<String>>(mut self, prefix: T) -> Self {
        let re = Regex::new(r"^/([-_.a-zA-Z0-9]+/)?").unwrap();
        let id = prefix.into();
        match re.is_match(&id) {
            true => { self.topic_prefix = Some(id); }
            false => {
                println!("Invalid topic prefix or format. Setting to default: {}", BLPAPI_DEFAULT_TOPIC_PREFIX);
                self.topic_prefix = Some(BLPAPI_DEFAULT_TOPIC_PREFIX.into());
            }
        };
        self
    }

    /// Setting allowance of multiple correlation IDs with a message
    pub fn set_allow_multiple_correlators_per_msg(mut self, allow: bool) -> Self {
        match allow == true {
            true => { self.multiple_corr_per_msg = Some(0usize) }
            false => { self.multiple_corr_per_msg = Some(1usize) }
        };
        self
    }

    /// Setting Authentication Options
    /// Defaults to OS_LOGON
    pub fn set_authentication_options(mut self, auth: Authentication) -> Self {
        self.authentication = Some(auth);
        self
    }

    /// Setting auto restart on disconnect option
    pub fn set_auto_restart_on_disconnect(mut self, option: bool) -> Self {
        match option == true {
            true => { self.auto_restart = Some(0usize) }
            false => { self.auto_restart = Some(1usize) }
        };
        self
    }

    /// Setting max pending requests
    pub fn set_max_pending_requests(mut self, no: u16) -> Self {
        self.max_pending_request = Some(no);
        self
    }

    /// setting max number of start attempts
    pub fn set_num_start_attempts(mut self, no: u16) -> Self {
        self.max_start_attempts = Some(no);
        self
    }

    /// Setting max queue size
    pub fn set_max_event_queue_size(mut self, no: usize) -> Self {
        self.max_queue_size = Some(no);
        self
    }

    /// Setting the slow consumer warning marks
    pub fn set_both_slow_consumer_warning_marks(mut self, low: f32, high: f32) -> Self {
        let low = match low >= 0.0 && low <= 1.0 {
            true => low,
            false => BLPAPI_DEFAULT_LOW_WATER_MARK,
        };
        let high = match high >= 0.0 && high <= 1.0 {
            true => high,
            false => BLPAPI_DEFAULT_HIGH_WATER_MARK,
        };
        match high > low {
            true => {
                self.slow_consumer_warning_high_water_mark = Some(high);
                self.slow_consumer_warning_low_water_mark = Some(low);
            }
            false => {
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
    pub fn set_client_mode(mut self, mode: ClientMode) -> Self {
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
    pub fn set_keep_alive(mut self, enable: bool) -> Self {
        self.keep_alive = Some(enable);
        self
    }

    /// Setting default keep alive inactivity time
    pub fn set_default_keep_alive_inactivity_time(mut self, ms: isize) -> Self {
        match ms >= 0 {
            true => self.keep_alive_inactivity_time = Some(ms),
            false => self.keep_alive_inactivity_time = Some(BLPAPI_DEFAULT_KEEP_ALIVE_INACTIVITY_TIME),
        };
        self
    }

    /// Setting default keep alive response timeout
    pub fn set_default_keep_alive_response_timeout(mut self, ms: isize) -> Self {
        match ms >= 0 {
            true => self.keep_alive_response_timeout = Some(ms),
            false => self.keep_alive_response_timeout = Some(BLPAPI_DEFAULT_KEEP_ALIVE_RESPONSE_TIMEOUT),
        };
        self
    }

    /// Setting record subscription data receive times
    pub fn set_record_subscription_data_receive_times(mut self, record: bool) -> Self {
        self.record_subscription = Some(record);
        self
    }

    /// Setting the service check timeout ms
    pub fn set_service_check_timeout(mut self, ms: isize) -> Self {
        match ms >= 0 {
            true => self.service_check_timeout = Some(ms),
            false => self.service_check_timeout = Some(BLPAPI_DEFAULT_SERVICE_CHECK_TIMEOUT),
        };
        self
    }

    /// Setting service download timeout
    pub fn set_service_download_timeout(mut self, ms: isize) -> Self {
        match ms >= 0 {
            true => self.service_download_timeout = Some(ms),
            false => self.service_download_timeout = Some(BLPAPI_DEFAULT_SERVICE_DOWNLOAD_TIMEOUT),
        };
        self
    }

    /// Setting tls options
    pub fn set_tls_options(mut self, options: TlsOptions) -> Self {
        self.tls_options = Some(options);
        self
    }

    /// Setting flush publish events timeout
    pub fn set_flush_published_events_timeout(mut self, ms: isize) -> Self {
        match ms >= 0 {
            true => self.flush_published_events_timeout = Some(ms),
            false => self.flush_published_events_timeout = Some(BLPAPI_DEFAULT_FLUSH_PUBLISHED_EVENTS_TIMEOUT),
        };
        self
    }

    /// Setting session name
    pub fn set_session_name<T: Into<String>>(mut self, name: T) -> Self {
        self.session_name = Some(name.into());
        self
    }

    /// Setting bandwidth mode
    pub fn set_bandwidth_save_mode_disabled(mut self, is_disabled: bool) -> Self {
        self.bandwidth_save_mode = Some(is_disabled);
        self
    }
    /// Setting application identifier key
    pub fn set_application_identity_key<T: Into<String>>(mut self, application_id: T) -> Self {
        self.application_identifier = Some(application_id.into());
        self
    }

    /// Builder function
    pub fn build(self) -> SessionOptions {
        SessionOptions {
            ptr: self.ptr.expect("Expected pointer"),
            server_host: self.server_host.expect("Expected server host"),
            server_port: self.server_port.expect("Expected server port"),
            server_index: self.server_index.expect("Expected server index"),
            server_addresses: self.server_addresses.expect("Expected server addresses"),
            timeout: self.timeout.expect("Expected timeout"),
            services: self.services.expect("Expected subscription services"),
            topic_prefix: self.topic_prefix.expect("Expected topic prefix"),
            multiple_corr_per_msg: self.multiple_corr_per_msg.expect("Expect multiple_corr_per_msg"),
            client_mode: self.client_mode.expect("Expected client mode"),
            authentication: self.authentication.expect("Expected authentication option"),
            auto_restart: self.auto_restart.expect("Expected auto restart"),
            max_pending_request: self.max_pending_request.expect("Expected max_request_tries"),
            max_start_attempts: self.max_start_attempts.expect("Expected max_start_attempts"),
            max_queue_size: self.max_queue_size.expect("Expected max queue size"),
            slow_consumer_warning_low_water_mark: self.slow_consumer_warning_low_water_mark.expect("Expected low water mark"),
            slow_consumer_warning_high_water_mark: self.slow_consumer_warning_high_water_mark.expect("Expected high water mark"),
            keep_alive: self.keep_alive.expect("Expected keep alive"),
            keep_alive_inactivity_time: self.keep_alive_inactivity_time.expect("Expected keep alive inactivity time"),
            keep_alive_response_timeout: self.keep_alive_response_timeout.expect("Expect keep alive response timeout"),
            record_subscription: self.record_subscription.expect("Expected record subscription"),
            service_check_timeout: self.service_check_timeout.expect("Expected service check timeout"),
            service_download_timeout: self.service_download_timeout.expect("Expected service download timeout"),
            flush_published_events_timeout: self.flush_published_events_timeout.expect("Expected flush published events timeout"),
            session_name: self.session_name.expect("Expected session name"),
            tls_options: self.tls_options.expect("Expected TLS options"),
            bandwidth_save_mode: self.bandwidth_save_mode.expect("Expected bandwidth save mode"),
            application_identifier: self.application_identifier.expect("Expected application identifier"),
            socks_5_config: self.socks_5_config,
        }
    }
}

impl Default for SessionOptionsBuilder {
    fn default() -> Self {
        let server_addresses = vec![ServerAddress {
            host: BLPAPI_DEFAULT_HOST.into(),
            port: BLPAPI_DEFAULT_PORT,
            index: BLPAPI_DEFAULT_INDEX,
            socks_5_config: None,
            socks_5_host: None,
            socks_5_port: None,
        }];
        unsafe {
            SessionOptionsBuilder {
                ptr: Some(blpapi_SessionOptions_create()),
                server_host: Some(BLPAPI_DEFAULT_HOST.into()),
                server_port: Some(BLPAPI_DEFAULT_PORT),
                server_index: Some(BLPAPI_DEFAULT_INDEX),
                server_addresses: Some(server_addresses),
                timeout: Some(BLPAPI_DEFAULT_TIMEOUT),
                service_check_timeout: Some(BLPAPI_DEFAULT_SERVICE_CHECK_TIMEOUT),
                service_download_timeout: Some(BLPAPI_DEFAULT_SERVICE_DOWNLOAD_TIMEOUT),
                session_name: Some(BLPAPI_DEFAULT_SESSION_NAME.into()),
                slow_consumer_warning_high_water_mark: Some(BLPAPI_DEFAULT_HIGH_WATER_MARK),
                slow_consumer_warning_low_water_mark: Some(BLPAPI_DEFAULT_LOW_WATER_MARK),
                client_mode: Some(BLPAPI_DEFAULT_CLIENT_MODE),
                authentication: Some(BLPAPI_DEFAULT_AUTHENTICATION),
                auto_restart: Some(BLPAPI_DEFAULT_AUTO_RESTART),
                multiple_corr_per_msg: Some(BLPAPI_DEFAULT_MULTIPLE_CORR_PER_MSG),
                services: Some(vec![
                    BLPAPI_DEFAULT_SERVICE_IDENTIFIER_MKTDATA.into(),
                    BLPAPI_DEFAULT_SERVICE_IDENTIFIER_REFDATA.into(),
                ]),
                topic_prefix: Some(BLPAPI_DEFAULT_TOPIC_PREFIX.into()),
                max_pending_request: Some(BLPAPI_DEFAULT_MAX_PENDING_REQUEST),
                max_start_attempts: Some(BLPAPI_DEFAULT_MAX_START_ATTEMPTS),
                max_queue_size: Some(BLPAPI_DEFAULT_MAX_EVENT_QUEUE_SIZE),
                keep_alive_inactivity_time: Some(BLPAPI_DEFAULT_KEEP_ALIVE_INACTIVITY_TIME),
                keep_alive_response_timeout: Some(BLPAPI_DEFAULT_KEEP_ALIVE_RESPONSE_TIMEOUT),
                keep_alive: Some(BLPAPI_DEFAULT_KEEP_ALIVE),
                record_subscription: Some(BLPAPI_DEFAULT_RECORD_SUBSCRIPTION),
                flush_published_events_timeout: Some(BLPAPI_DEFAULT_FLUSH_PUBLISHED_EVENTS_TIMEOUT),
                tls_options: Some(TlsOptions::default()),
                bandwidth_save_mode: Some(BLPAPI_DEFAULT_BANDWIDTH_SAVE_MODE),
                application_identifier: Some(BLPAPI_DEFAULT_APPLICATION_IDENTIFICATION_KEY.into()),
                socks_5_config: None,
            }
        }
    }
}

impl SessionOptions {
    pub fn create(&self) {
        // Creating a new instance based on the provided parameter
        let server_host_con = CString::new(&*self.server_host).expect("Failed to generated host");
        let server_port_con = self.server_port;
        unsafe {
            blpapi_SessionOptions_setServerHost(self.ptr, server_host_con.as_ptr());
            blpapi_SessionOptions_setServerPort(self.ptr, server_port_con as c_ushort);
        }
        for adr in self.server_addresses.iter() {
            let server_host = adr.host.clone();
            let server_port = adr.port;
            let server_index = adr.index;
            let host: CString;
            match &self.socks_5_config {
                Some(socks) => {
                    unsafe {
                        host = CString::new(server_host).expect("Failed to generated host");
                        let socks_ptr = socks.ptr as *const _;
                        let res = blpapi_SessionOptions_setServerAddressWithProxy(
                            self.ptr,
                            host.as_ptr(),
                            server_port as c_ushort,
                            socks_ptr,
                            server_index,
                        );
                        if res != 0 {
                            panic!("Failed to set server address with proxy");
                        }
                    };
                }
                None => {
                    unsafe {
                        host = CString::new(server_host).expect("Failed to generated host");
                        let res = blpapi_SessionOptions_setServerAddress(self.ptr, host.as_ptr(), server_port as c_ushort, server_index);
                        if res != 0 {
                            panic!("Failed to set server address");
                        }
                    }
                }
            };
        }

        // Setting die services
        for service in self.services.iter() {
            let new_service = CString::new(service.clone()).expect("Failed to generated service");
            unsafe {
                blpapi_SessionOptions_setDefaultSubscriptionService(self.ptr, new_service.as_ptr());
            }
        }
        let topic_prefix = CString::new(&*self.topic_prefix).expect("Failed to generated topic prefix");
        let session_name = CString::new(&*self.session_name).expect("Failed to generated session name");
        let session_name_len = self.session_name.len();
        let aik = CString::new(&*self.application_identifier).expect("Failed to generate application identifier");
        let aik_len = self.application_identifier.len();
        let auth = match self.authentication {
            Authentication::OsLogon => BLPAPI_AUTHENTICATION_OS_LOGON,
            Authentication::DirectoryService => BLPAPI_AUTHENTICATION_DIRECTORY_SERVICE,
            Authentication::ApplicationOnly => BLPAPI_AUTHENTICATION_APPLICATION_ONLY,
            Authentication::AppnameAndKey => BLPAPI_AUTHENTICATION_APPNAME_AND_KEY,
        };
        let c_auth = CString::new(auth).expect("Failed to generate authentication");

        let keep_alive = match self.keep_alive {
            true => 0,
            false => 1,
        };
        let bandwidth = match self.bandwidth_save_mode {
            true => 0,
            false => 1,
        };

        unsafe {
            blpapi_SessionOptions_setConnectTimeout(self.ptr, self.timeout as c_uint);
            blpapi_SessionOptions_setDefaultTopicPrefix(self.ptr, topic_prefix.as_ptr());
            blpapi_SessionOptions_setAutoRestart(self.ptr, self.auto_restart as c_int);
            blpapi_SessionOptions_setMaxPendingRequests(self.ptr, self.max_pending_request as c_int);
            blpapi_SessionOptions_setNumStartAttempts(self.ptr, self.max_start_attempts as c_int);
            blpapi_SessionOptions_setMaxEventQueueSize(self.ptr, self.max_queue_size);
            blpapi_SessionOptions_setSlowConsumerWarningLoWaterMark(self.ptr, self.slow_consumer_warning_low_water_mark);
            blpapi_SessionOptions_setSlowConsumerWarningHiWaterMark(self.ptr, self.slow_consumer_warning_high_water_mark);
            blpapi_SessionOptions_setDefaultKeepAliveInactivityTime(self.ptr, self.keep_alive_inactivity_time as c_int);
            blpapi_SessionOptions_setDefaultKeepAliveResponseTimeout(self.ptr, self.keep_alive_response_timeout as c_int);
            blpapi_SessionOptions_setKeepAliveEnabled(self.ptr, keep_alive as c_int);
            blpapi_SessionOptions_setServiceCheckTimeout(self.ptr, self.service_check_timeout as c_int);
            blpapi_SessionOptions_setServiceDownloadTimeout(self.ptr, self.service_download_timeout as c_int);
            blpapi_SessionOptions_setFlushPublishedEventsTimeout(self.ptr, self.flush_published_events_timeout as c_int);
            blpapi_SessionOptions_setSessionName(self.ptr, session_name.as_ptr(), session_name_len);
            blpapi_SessionOptions_setBandwidthSaveModeDisabled(self.ptr, bandwidth as c_int);
            blpapi_SessionOptions_setApplicationIdentityKey(self.ptr, aik.as_ptr(), aik_len);
            blpapi_SessionOptions_setAuthenticationOptions(self.ptr, c_auth.as_ptr());
        }
    }


    /// Get client mode
    pub fn client_mode(&self) -> Result<ClientMode, Error> {
        let mode = unsafe { blpapi_SessionOptions_clientMode(self.ptr) };
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
            blpapi_SessionOptions_setClientMode(self.ptr, mode as c_int);
        }
    }

    /// Get server host
    pub fn server_host(&self) -> String {
        let chost = unsafe { CStr::from_ptr(blpapi_SessionOptions_serverHost(self.ptr)) };
        chost.to_string_lossy().into_owned()
    }

    /// Get server port
    pub fn server_port(&self) -> u16 {
        unsafe { blpapi_SessionOptions_serverPort(self.ptr) as u16 }
    }

    /// Get server address at specific index (not socks5)
    pub fn get_server_address(&self, index: usize) -> Result<ServerAddress, Error> {
        let mut host_ptr: *const c_char = ptr::null();
        let mut port: c_ushort = 0;

        let res = unsafe {
            blpapi_SessionOptions_getServerAddress(
                self.ptr,
                &mut host_ptr,
                &mut port,
                index)
        };
        if res != 0 {
            return Err(Error::session_options(
                "SessionOptions",
                "get_server_address",
                "Error when trying to receive Server Address",
            ));
        };

        if host_ptr.is_null() {
            return Err(Error::NotFound("Server address not found for index".to_string()));
        };

        let c_str = unsafe { CStr::from_ptr(host_ptr) };
        let host_string = c_str.to_string_lossy().into_owned();
        Ok(ServerAddress {
            host: host_string,
            port,
            index,
            socks_5_config: None,
            socks_5_host: None,
            socks_5_port: None,
        })
    }

    /// Get server address for specific Socks5Config
    pub fn get_server_address_socks5config(&self, index: usize) -> Result<ServerAddress, Error> {
        let mut server_host_ptr: *const c_char = ptr::null();
        let mut server_port: c_ushort = 0;
        let mut socks5_host_ptr: *const c_char = ptr::null();
        let port: u16 = self.socks_5_config.clone().unwrap().port;

        unsafe {
            let res = blpapi_SessionOptions_getServerAddressWithProxy(
                self.ptr,
                &mut server_host_ptr as *mut _,
                &mut server_port,
                &mut socks5_host_ptr as *mut _,
                port as c_ushort,
                index,
            ) as i32;

            if res != 0 {
                return Err(Error::session_options(
                    "SessionOptions",
                    "get_server_address_socks5config",
                    "Error when trying to receive Server Address",
                ));
            };
        };


        let server_host = unsafe {
            CStr::from_ptr(server_host_ptr)
                .to_string_lossy()
                .into_owned()
        };

        let socks5_host = unsafe {
            CStr::from_ptr(socks5_host_ptr)
                .to_string_lossy()
                .into_owned()
        };

        Ok(ServerAddress {
            host: server_host,
            port: server_port as u16,
            index,
            socks_5_config: None,
            socks_5_host: Some(socks5_host),
            socks_5_port: Some(port),
        })
    }

    /// Remove server address
    pub fn remove_server_address(&mut self, index: usize) -> Result<(), Error> {
        unsafe {
            let res = blpapi_SessionOptions_removeServerAddress(
                self.ptr,
                index);
            if res != 0 {
                return Err(Error::session_options(
                    "SessionOptions",
                    "remove_server_address",
                    "Error when trying to remove Server Address",
                ));
            };
        }
        let current_addresses = &mut self.server_addresses;
        current_addresses.remove(index);
        for adr in current_addresses.iter_mut() {
            if adr.index >= index {
                adr.index -= 1;
            }
        };

        Ok(())
    }

    /// Get the number of serveraddresses
    pub fn num_server_addresses(&self) -> Result<i16, Error> {
        let adr = unsafe {
            blpapi_SessionOptions_numServerAddresses(self.ptr)
        };

        match adr >= 0 {
            true => Ok(adr as i16),
            false => Err(Error::NotFound(format!("Invalid amount of server addresses"))),
        }
    }

    /// Get the time (milliseconds) of connection timeout
    pub fn connect_timeout(&self) -> Result<u32, Error> {
        let to = unsafe {
            blpapi_SessionOptions_connectTimeout(self.ptr) as u32
        };
        match to > 0 {
            true => Ok(to),
            false => Err(Error::session_options(
                "SessionOptions",
                "connect_timeout",
                "Error when trying to receive connect timeout",
            ))
        }
    }

    /// Get value of the allow multiple correlators per message
    pub fn allow_multiple_correlators_per_msg(&self) -> Result<bool, Error> {
        let res = unsafe {
            blpapi_SessionOptions_allowMultipleCorrelatorsPerMsg(self.ptr)
        };

        match res == 0 {
            true => Ok(true),
            false => Err(Error::session_options(
                "SessionOptions",
                "allow_multiple_correlators_per_msg",
                "Error when trying to receive status of allow multiple correlators per msg",
            ))
        }
    }

    pub fn max_pending_requests(&self) -> Result<u16, Error> {
        let max_req = unsafe {
            blpapi_SessionOptions_maxPendingRequests(self.ptr)
        } as u16;
        match max_req > 0 {
            true => Ok(max_req),
            false => Err(Error::session_options(
                "SessionOptions",
                "max_pending_requests",
                "Error when trying to receive status of max pending requests",
            ))
        }
    }

    /// Get the default service
    pub fn default_services(&self) -> Result<String, Error> {
        let res = unsafe {
            blpapi_SessionOptions_defaultServices(
                self.ptr
            )
        };

        let c_services = unsafe { CStr::from_ptr(res).to_owned() };
        let c_str = c_services.to_string_lossy().into_owned();
        match c_str.len() > 0 {
            true => Ok(c_str),
            false => Err(Error::session_options(
                "SessionOptions",
                "default_services",
                "Error when trying to receive default services string",
            ))
        }
    }

    ///Get current default subscription service
    pub fn default_subscription_service(&self) -> Result<String, Error> {
        let service = unsafe {
            blpapi_SessionOptions_defaultSubscriptionService(
                self.ptr
            )
        };
        let c_services = unsafe { CStr::from_ptr(service).to_owned() };
        let c_str = c_services.to_string_lossy().into_owned();
        match c_str.len() > 0 {
            true => Ok(c_str),
            false => Err(Error::session_options(
                "SessionOptions",
                "default_subscription_service",
                "Error when trying to receive default subscription service string",
            ))
        }
    }

    /// Get the defaul Topic prefix
    pub fn default_topic_prefix(&self) -> Result<String, Error> {
        let service = unsafe {
            blpapi_SessionOptions_defaultTopicPrefix(
                self.ptr
            )
        };
        let c_services = unsafe { CStr::from_ptr(service).to_owned() };
        let c_str = c_services.to_string_lossy().into_owned();
        match c_str.len() > 0 {
            true => Ok(c_str),
            false => Err(Error::session_options(
                "SessionOptions",
                "default_topic_prefix",
                "Error when trying to receive default topic prefix string",
            ))
        }
    }

    /// Getting authentication options string
    pub fn authentication_options(&self) -> Result<String, Error> {
        let res = unsafe {
            blpapi_SessionOptions_authenticationOptions(self.ptr)
        };
        let c_services = unsafe { CStr::from_ptr(res).to_owned() };
        let c_str = c_services.to_string_lossy().into_owned();
        match c_str.len() > 0 {
            true => Ok(c_str),
            false => Err(Error::session_options(
                "SessionOptions",
                "authentication_options",
                "Error when trying to receive authentication options",
            ))
        }
    }

    /// Get the auto restart on disconnection status
    pub fn auto_restart_on_disconnection(&self) -> Result<bool, Error> {
        let res = unsafe {
            blpapi_SessionOptions_autoRestartOnDisconnection(self.ptr)
        };

        match res == 0 {
            true => Ok(true),
            false => Err(Error::session_options(
                "SessionOptions",
                "auto_restart_on_disconnection",
                "Error when trying to receive status of auto restart on disconnection",
            ))
        }
    }

    /// Get the number of start attempts in milliseconds
    pub fn num_start_attempts(&self) -> Result<u16, Error> {
        let res = unsafe {
            blpapi_SessionOptions_numStartAttempts(
                self.ptr
            )
        };
        match res > 0 {
            true => Ok(res as u16),
            false => Err(Error::session_options(
                "SessionOptions",
                "num_start_attempts",
                "Error when trying to receive number of start attempts",
            ))
        }
    }

    /// Get the number of max event queue size
    pub fn max_event_queue_size(&self) -> Result<usize, Error> {
        let res = unsafe {
            blpapi_SessionOptions_maxEventQueueSize(
                self.ptr
            )
        };
        match res > 0 {
            true => Ok(res as usize),
            false => Err(Error::session_options(
                "SessionOptions",
                "max_event_queue_size",
                "Error when trying to receive number of max event queue size",
            ))
        }
    }

    /// Get the value for the slow consumer warning low water mark
    pub fn slow_consumer_warning_lo_water_mark(&self) -> Result<f32, Error> {
        let res = unsafe {
            blpapi_SessionOptions_slowConsumerWarningLoWaterMark(
                self.ptr
            )
        };
        match res > 0.0 {
            true => Ok(res),
            false => Err(Error::session_options(
                "SessionOptions",
                "slow_consumer_warning_lo_water_mark",
                "Error when trying to receive value of slow consumer warning lo water mark",
            ))
        }
    }

    /// Get the value for the slow consumer warning high water mark
    pub fn slow_consumer_warning_hi_water_mark(&self) -> Result<f32, Error> {
        let res = unsafe {
            blpapi_SessionOptions_slowConsumerWarningHiWaterMark(
                self.ptr
            )
        };
        match res > 0.0 {
            true => Ok(res),
            false => Err(Error::session_options(
                "SessionOptions",
                "slow_consumer_warning_hi_water_mark",
                "Error when trying to receive value of slow consumer warning high water mark",
            ))
        }
    }

    /// Get the value of the keep alive inactivety time
    pub fn default_keep_alive_inactivity_time(&self) -> Result<isize, Error> {
        let res = unsafe {
            blpapi_SessionOptions_defaultKeepAliveInactivityTime(
                self.ptr
            )
        };
        match res > 0 {
            true => Ok(res as isize),
            false => Err(Error::session_options(
                "SessionOptions",
                "default_keep_alive_inactivity_time",
                "Error when trying to receive value of default keep-alive inactivity time",
            ))
        }
    }

    /// Get the value of the keep response timeout
    pub fn default_keep_alive_response_timeout(&self) -> Result<isize, Error> {
        let res = unsafe {
            blpapi_SessionOptions_defaultKeepAliveResponseTimeout(
                self.ptr
            )
        };
        match res > 0 {
            true => Ok(res as isize),
            false => Err(Error::session_options(
                "SessionOptions",
                "default_keep_alive_response_timeout",
                "Error when trying to receive value of default keep-alive response timeout",
            ))
        }
    }

    /// Get the value of the keep alive status
    pub fn keep_alive_enabled(&self) -> Result<bool, Error> {
        let res = unsafe {
            blpapi_SessionOptions_keepAliveEnabled(
                self.ptr
            )
        };
        match res == 0 {
            true => Ok(true),
            false => Err(Error::session_options(
                "SessionOptions",
                "default_keep_alive_enabled",
                "Error when trying to receive value of default keep-alive status",
            ))
        }
    }

    /// Get the value of record subscription data receive times
    pub fn record_subscription_data_receive_times(&self) -> Result<bool, Error> {
        let res = unsafe {
            blpapi_SessionOptions_recordSubscriptionDataReceiveTimes(
                self.ptr
            )
        };
        match res == 0 {
            true => Ok(true),
            false => Err(Error::session_options(
                "SessionOptions",
                "record_subscription_data_receive_times",
                "Error when trying to receive status of record subscription data receive times",
            ))
        }
    }

    /// Get the value of service check timeout
    pub fn service_check_timeout(&self) -> Result<isize, Error> {
        let res = unsafe {
            blpapi_SessionOptions_serviceCheckTimeout(
                self.ptr
            )
        };
        match res >= 0 {
            true => Ok(res as isize),
            false => Err(Error::session_options(
                "SessionOptions",
                "service_check_timeout",
                "Error when trying to receive status of record subscription data receive times",
            ))
        }
    }

    /// Get the value of service download timeout
    pub fn service_download_timeout(&self) -> Result<isize, Error> {
        let res = unsafe {
            blpapi_SessionOptions_serviceDownloadTimeout(
                self.ptr
            )
        };
        match res >= 0 {
            true => Ok(res as isize),
            false => Err(Error::session_options(
                "SessionOptions",
                "service_download_timeout",
                "Error when trying to receive status of download timeout",
            ))
        }
    }

    /// Get the value of flush published events timeout
    pub fn flush_published_events_timeout(&self) -> Result<isize, Error> {
        let res = unsafe {
            blpapi_SessionOptions_flushPublishedEventsTimeout(
                self.ptr
            )
        };
        match res >= 0 {
            true => Ok(res as isize),
            false => Err(Error::session_options(
                "SessionOptions",
                "flush_published_events_timeout",
                "Error when trying to receive value of the flush published events timeout",
            ))
        }
    }

    /// Get the value of band width save mode
    pub fn band_width_save_mode_disabled(&self) -> Result<bool, Error> {
        let res = unsafe {
            blpapi_SessionOptions_bandwidthSaveModeDisabled(
                self.ptr
            )
        };
        match res == 0 {
            true => Ok(true),
            false => Err(Error::session_options(
                "SessionOptions",
                "band_width_save_mode_disabled",
                "Error when trying to receive status band width save mode disabled",
            ))
        }
    }

    /// Get the application identity key (AIK)
    pub fn application_identity_key(&self) -> Result<String, Error> {
        let mut app_id: *const c_char = ptr::null();
        let mut app_size: usize = self.application_identifier.len();
        unsafe {
            let res = blpapi_SessionOptions_applicationIdentityKey(
                &mut app_id as *mut _,
                &mut app_size as *mut _,
                self.ptr,
            );
            if res != 0 {
                return Err(Error::session_options(
                    "SessionOptions",
                    "application_identity_key",
                    "Error when trying to receive application key",
                ));
            };
        };

        let app_key = unsafe {
            CStr::from_ptr(app_id)
                .to_string_lossy()
                .into_owned()
        };
        Ok(app_key)
    }
    /// Get the session name
    pub fn session_name(&self) -> Result<String, Error> {
        let mut session_name: *const c_char = ptr::null();
        let mut session_name_size: usize = self.application_identifier.len();
        unsafe {
            let res = blpapi_SessionOptions_sessionName(
                &mut session_name as *mut _,
                &mut session_name_size as *mut _,
                self.ptr,
            );
            if res != 0 {
                return Err(Error::session_options(
                    "SessionOptions",
                    "session_name",
                    "Error when trying to receive session name",
                ));
            };
        };

        let ses_name = unsafe {
            CStr::from_ptr(session_name)
                .to_string_lossy()
                .into_owned()
        };
        Ok(ses_name)
    }

    pub fn print<T: Write>(&self, writer: &mut T, indent: i32, spaces: i32) -> Result<(), Error> {
        let mut context = StreamWriterContext { writer };
        unsafe {
            let res = blpapi_SessionOptions_print(
                self.ptr,
                Some(write_to_stream_cb),
                &mut context as *mut _ as *mut c_void,
                indent as std::ffi::c_int,
                spaces as std::ffi::c_int,
            );
            if res != 0 {
                return Err(Error::session_options(
                    "SessionOptions",
                    "print",
                    "Error when trying to write to stream writer",
                ));
            };
        };
        Ok(())
    }

    /// Build a session, transfer ownership
    pub fn sync(self) -> SessionSync {
        SessionSync::from_options(self)
    }
}

impl Drop for SessionOptions {
    fn drop(&mut self) {
        unsafe { blpapi_SessionOptions_destroy(self.ptr) }
    }
}

impl Clone for SessionOptions {
    fn clone(&self) -> Self {
        let cloned = SessionOptions::default();
        unsafe {
            blpapi_SessionOptions_copy(self.ptr, cloned.ptr);
        }
        cloned
    }
}

impl Default for SessionOptions {
    fn default() -> Self {
        let default_server_addresses = vec![ServerAddress {
            host: BLPAPI_DEFAULT_HOST.into(),
            port: BLPAPI_DEFAULT_PORT,
            index: BLPAPI_DEFAULT_INDEX,
            socks_5_config: None,
            socks_5_host: None,
            socks_5_port: None,
        }];
        unsafe {
            SessionOptions {
                ptr: blpapi_SessionOptions_create(),
                server_host: BLPAPI_DEFAULT_HOST.into(),
                server_port: BLPAPI_DEFAULT_PORT,
                server_index: BLPAPI_DEFAULT_INDEX,
                server_addresses: default_server_addresses,
                timeout: BLPAPI_DEFAULT_TIMEOUT,
                service_check_timeout: BLPAPI_DEFAULT_SERVICE_CHECK_TIMEOUT,
                service_download_timeout: BLPAPI_DEFAULT_SERVICE_DOWNLOAD_TIMEOUT,
                session_name: BLPAPI_DEFAULT_SESSION_NAME.into(),
                slow_consumer_warning_high_water_mark: BLPAPI_DEFAULT_HIGH_WATER_MARK,
                slow_consumer_warning_low_water_mark: BLPAPI_DEFAULT_LOW_WATER_MARK,
                client_mode: BLPAPI_DEFAULT_CLIENT_MODE,
                authentication: BLPAPI_DEFAULT_AUTHENTICATION,
                auto_restart: BLPAPI_DEFAULT_AUTO_RESTART,
                multiple_corr_per_msg: BLPAPI_DEFAULT_MULTIPLE_CORR_PER_MSG,
                services: vec![
                    BLPAPI_DEFAULT_SERVICE_IDENTIFIER_MKTDATA.into(),
                    BLPAPI_DEFAULT_SERVICE_IDENTIFIER_REFDATA.into(),
                ],
                topic_prefix: BLPAPI_DEFAULT_TOPIC_PREFIX.into(),
                max_pending_request: BLPAPI_DEFAULT_MAX_PENDING_REQUEST,
                max_start_attempts: BLPAPI_DEFAULT_MAX_START_ATTEMPTS,
                max_queue_size: BLPAPI_DEFAULT_MAX_EVENT_QUEUE_SIZE,
                keep_alive_inactivity_time: BLPAPI_DEFAULT_KEEP_ALIVE_INACTIVITY_TIME,
                keep_alive_response_timeout: BLPAPI_DEFAULT_KEEP_ALIVE_RESPONSE_TIMEOUT,
                keep_alive: BLPAPI_DEFAULT_KEEP_ALIVE,
                record_subscription: BLPAPI_DEFAULT_RECORD_SUBSCRIPTION,
                flush_published_events_timeout: BLPAPI_DEFAULT_FLUSH_PUBLISHED_EVENTS_TIMEOUT,
                tls_options: TlsOptions::default(),
                bandwidth_save_mode: BLPAPI_DEFAULT_BANDWIDTH_SAVE_MODE,
                application_identifier: BLPAPI_DEFAULT_APPLICATION_IDENTIFICATION_KEY.into(),
                socks_5_config: None,
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::socks_5_config::Socks5ConfigBuilder;

    #[test]
    fn test_session_options_builder() -> Result<(), Error> {
        let _builder = SessionOptionsBuilder::new();
        Ok(())
    }

    #[test]
    fn test_session_options_builder_set_server_host() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_server_host("localhost");
        assert_eq!(_builder.server_host.unwrap(), "localhost");
        Ok(())
    }

    #[test]
    fn test_session_options_builder_set_server_port() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_server_port(9999);
        assert_eq!(_builder.server_port.unwrap(), 9999);
        Ok(())
    }

    #[test]
    fn test_session_options_builder_set_server_index() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_index(22);
        assert_eq!(_builder.server_index.unwrap(), 22);
        Ok(())
    }

    #[test]
    fn test_session_options_builder_set_server_address() -> Result<(), Error> {
        let host = "localhost";
        let port: u16 = 8888;
        let index: usize = 1;
        let builder = SessionOptionsBuilder::new();
        let builder = builder.set_server_address(
            host,
            port,
            index,
        );
        assert_eq!(builder.server_host.unwrap(), host);
        assert_eq!(builder.server_port.unwrap(), port);
        assert_eq!(builder.server_index.unwrap(), index);
        Ok(())
    }

    #[test]
    fn test_session_options_builder_set_server_address_socks5config() -> Result<(), Error> {
        let socks_builder = Socks5ConfigBuilder::new();
        let socks_builder = socks_builder.set_host_name("127.1.1.1").unwrap();
        let socks_builder = socks_builder.set_host_name_size(9).unwrap();
        let socks_builder = socks_builder.set_port(8194);
        let config = socks_builder.build();

        let builder = SessionOptionsBuilder::new();
        let builder = builder.set_server_address_socks5config(config);

        let builder_config = builder.socks_5_config.unwrap();
        assert_eq!(builder_config.host_name, "127.1.1.1");
        assert_eq!(builder_config.host_name_size, 9);
        assert_eq!(builder_config.port, 8194);

        Ok(())
    }

    #[test]
    fn test_session_options_builder_set_connect_timeout() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_connect_timeout(22);
        assert_eq!(_builder.timeout.unwrap(), 22);
        Ok(())
    }

    #[test]
    fn test_session_options_builder_set_subscription_service() -> Result<(), Error> {
        let service = "//blpapi/service";
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_default_subscription_service(service);
        let _cor_service = _builder.services.unwrap();
        Ok(())
    }
    #[test]
    fn test_session_options_builder_set_topic_prefix() -> Result<(), Error> {
        let service = "/prefix/";
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_default_topic_prefix(service);
        let prefix = _builder.topic_prefix.unwrap();
        assert_eq!(&prefix, service);
        assert_ne!(&prefix, "invalid_prefix");
        Ok(())
    }
    #[test]
    fn test_session_options_builder_set_multiple_corr() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_allow_multiple_correlators_per_msg(true);
        assert_eq!(_builder.multiple_corr_per_msg.unwrap(), 0);
        let _builder = _builder.set_allow_multiple_correlators_per_msg(false);
        assert_eq!(_builder.multiple_corr_per_msg.unwrap(), 1);
        Ok(())
    }

    #[test]
    fn test_session_options_builder_set_auth_options() -> Result<(), Error> {
        let auth = Authentication::OsLogon;
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_authentication_options(auth);
        assert_eq!(_builder.authentication.unwrap(), Authentication::OsLogon);

        Ok(())
    }

    #[test]
    fn test_session_options_builder_auto_restart_disconnect() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_auto_restart_on_disconnect(true);
        assert_eq!(_builder.auto_restart.unwrap(), 0);
        let _builder = _builder.set_auto_restart_on_disconnect(false);
        assert_eq!(_builder.auto_restart.unwrap(), 1);
        Ok(())
    }
    #[test]
    fn test_session_options_builder_max_pend_req() -> Result<(), Error> {
        let no = 10_000;
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_max_pending_requests(no);
        assert_eq!(_builder.max_pending_request.unwrap(), no);
        Ok(())
    }

    #[test]
    fn test_session_options_builder_num_start_attempts() -> Result<(), Error> {
        let no = 10_000;
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_num_start_attempts(no);
        assert_eq!(_builder.max_start_attempts.unwrap(), no);
        Ok(())
    }

    #[test]
    fn test_session_options_builder_max_event_queue_size() -> Result<(), Error> {
        let no = 10_000;
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_max_event_queue_size(no);
        assert_eq!(_builder.max_queue_size.unwrap(), no);
        Ok(())
    }

    #[test]
    fn test_session_options_builder_slow_consumer_warn_marks() -> Result<(), Error> {
        let high = 0.9;
        let low = 0.7;
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_both_slow_consumer_warning_marks(low, high);
        assert_eq!(_builder.slow_consumer_warning_high_water_mark.unwrap(), high);
        assert_eq!(_builder.slow_consumer_warning_low_water_mark.unwrap(), low);

        // Change oder to test default
        let _builder = _builder.set_both_slow_consumer_warning_marks(high, low);
        assert_eq!(_builder.slow_consumer_warning_high_water_mark.unwrap(), BLPAPI_DEFAULT_HIGH_WATER_MARK);
        assert_eq!(_builder.slow_consumer_warning_low_water_mark.unwrap(), BLPAPI_DEFAULT_LOW_WATER_MARK);
        Ok(())
    }

    #[test]
    fn test_session_options_builder_client_mode() -> Result<(), Error> {
        let mode = ClientMode::Auto;
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_client_mode(mode);
        assert_eq!(_builder.client_mode.unwrap(), mode);
        Ok(())
    }

    #[test]
    fn test_session_options_builder_keep_alive() -> Result<(), Error> {
        let alive = true;
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_keep_alive(alive);
        assert_eq!(_builder.keep_alive.unwrap(), alive);
        let alive = false;
        let _builder = _builder.set_keep_alive(alive);
        assert_eq!(_builder.keep_alive.unwrap(), alive);
        Ok(())
    }

    #[test]
    fn test_session_options_builder_keep_alive_inactive_time() -> Result<(), Error> {
        let ms = 10_000;
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_default_keep_alive_inactivity_time(ms);
        assert_eq!(_builder.keep_alive_inactivity_time.unwrap(), ms);
        Ok(())
    }

    #[test]
    fn test_session_options_builder_keep_alive_response_time() -> Result<(), Error> {
        let ms = 10_000;
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_default_keep_alive_response_timeout(ms);
        assert_eq!(_builder.keep_alive_response_timeout.unwrap(), ms);
        Ok(())
    }

    #[test]
    fn test_session_options_subscription_data() -> Result<(), Error> {
        let record = true;
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_record_subscription_data_receive_times(record);
        assert_eq!(_builder.record_subscription.unwrap(), record);
        let record = false;
        let _builder = _builder.set_record_subscription_data_receive_times(record);
        assert_eq!(_builder.record_subscription.unwrap(), record);
        Ok(())
    }

    #[test]
    fn test_session_options_builder_service_check_timeout() -> Result<(), Error> {
        let ms = 10_000;
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_service_check_timeout(ms);
        assert_eq!(_builder.service_check_timeout.unwrap(), ms);

        // test for default
        let ms = -10_000;
        let _builder = _builder.set_service_check_timeout(ms);
        assert_eq!(_builder.service_check_timeout.unwrap(), BLPAPI_DEFAULT_SERVICE_CHECK_TIMEOUT);
        Ok(())
    }

    #[test]
    fn test_session_options_builder_service_download_timeout() -> Result<(), Error> {
        let ms = 10_000;
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_service_download_timeout(ms);
        assert_eq!(_builder.service_download_timeout.unwrap(), ms);

        // test for default
        let ms = -10_000;
        let _builder = _builder.set_service_download_timeout(ms);
        assert_eq!(_builder.service_download_timeout.unwrap(), BLPAPI_DEFAULT_SERVICE_DOWNLOAD_TIMEOUT);
        Ok(())
    }

    #[test]
    fn test_session_options_builder_published_events_timeout() -> Result<(), Error> {
        let ms = 10_000;
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_flush_published_events_timeout(ms);
        assert_eq!(_builder.flush_published_events_timeout.unwrap(), ms);

        // test for default
        let ms = -10_000;
        let _builder = _builder.set_flush_published_events_timeout(ms);
        assert_eq!(_builder.flush_published_events_timeout.unwrap(), BLPAPI_DEFAULT_FLUSH_PUBLISHED_EVENTS_TIMEOUT);
        Ok(())
    }

    #[test]
    fn test_session_options_builder_session_name() -> Result<(), Error> {
        let name = "neuer name";
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_session_name(name);
        assert_eq!(_builder.session_name.unwrap(), name);
        Ok(())
    }

    #[test]
    fn test_session_options_bandwidth_save_mode() -> Result<(), Error> {
        let record = true;
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_bandwidth_save_mode_disabled(record);
        assert_eq!(_builder.bandwidth_save_mode.unwrap(), record);
        let record = false;
        let _builder = _builder.set_bandwidth_save_mode_disabled(record);
        assert_eq!(_builder.bandwidth_save_mode.unwrap(), record);
        Ok(())
    }

    #[test]
    fn test_session_options_app_id() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::new();
        let id = "app_id";
        let _builder = builder.set_application_identity_key(id);
        assert_eq!(_builder.application_identifier.unwrap(), id);

        let builder = SessionOptionsBuilder::new();
        let id = String::from("app_id");
        let _builder = builder.set_application_identity_key(id);
        let id_check = String::from("app_id");
        assert_eq!(_builder.application_identifier.unwrap(), id_check);
        Ok(())
    }

    #[test]
    fn test_session_option_from_builder() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let _option = builder.build();
        Ok(())
    }

    #[test]
    fn test_session_options_builder_tls_option() -> Result<(), Error> {
        let tlsoption = TlsOptions::default();
        let builder = SessionOptionsBuilder::new();
        let _builder = builder.set_tls_options(tlsoption);
        assert_eq!(_builder.tls_options.unwrap().handshake_timeout, 10_000);

        Ok(())
    }


    #[test]
    fn test_session_options_get_server_host() {
        let options = SessionOptions::default();
        let server_address = options.server_host();
        assert_eq!(server_address, "127.0.0.1");
    }

    #[test]
    fn test_session_options_get_server_port() {
        let options = SessionOptions::default();
        let server_address = options.server_port();
        assert_eq!(server_address, BLPAPI_DEFAULT_PORT);
    }

    #[test]
    fn test_get_server_address() {
        let options = SessionOptions::default();
        let server_address = options.get_server_address(0);
        println!("Server address: {:?}", server_address);

        let options_two = SessionOptionsBuilder::default();
        let options_two = options_two.set_index(1);
        let options_two = options_two.build();
        options_two.create();

        let server_address = options_two.get_server_address(1);
        println!("Server address: {:?}", server_address);
    }

    #[test]
    fn test_get_server_address_proxy() -> Result<(), Error> {
        let socks_builder = Socks5ConfigBuilder::new();
        let socks_builder = socks_builder.set_host_name("127.1.1.1").unwrap();
        let socks_builder = socks_builder.set_host_name_size(9).unwrap();
        let socks_builder = socks_builder.set_port(8800);
        let socks_builder = socks_builder.set_index(1);
        let config = socks_builder.build();

        let options = SessionOptionsBuilder::default();
        let options = options.set_server_host("127.0.0.1").set_server_port(8194);
        let options = options.set_server_address_socks5config(config);
        let options = options.build();
        options.create();

        let _res = options.get_server_address_socks5config(0);
        println!("Res: {:?}", _res);
        Ok(())
    }

    #[test]
    fn test_remove_server_address() -> Result<(), Error> {
        // Create first server
        let host = "123.12.12.12";
        let port = 9999;
        let index = 1;
        let builder = SessionOptionsBuilder::default();
        let builder = builder.set_server_address(host, port, index);

        // Create additional server
        let new_host = "123.1.1.1";
        let new_port = 1111;
        let new_index = 2;
        let builder = builder.set_server_address(new_host, new_port, new_index);
        let mut options = builder.build();
        options.create();

        let no_server_address = options.num_server_addresses()?;
        println!("Server address: {:?}", no_server_address);

        let server_address = options.get_server_address(0);
        println!("Server address 0: {:?}", server_address);
        let server_address = options.get_server_address(1);
        println!("Server address 1: {:?}", server_address);
        let server_address = options.get_server_address(2);
        println!("Server address 2: {:?}", server_address);

        let res = options.remove_server_address(1);
        let no_server_address = options.num_server_addresses()?;
        println!("Server address after remove: {:?}", no_server_address);
        let server_address = options.get_server_address(0);
        println!("Server address 0: {:?}", server_address);
        let server_address = options.get_server_address(1);
        println!("Server address 1: {:?}", server_address);
        let server_address = options.get_server_address(2);
        println!("Server address 2: {:?}", server_address);

        match res {
            Ok(_r) => Ok(()),
            Err(e) => Err(e)
        }
    }

    #[test]
    fn test_session_options_num_server_adr() -> Result<(), Error> {
        // Create first server
        let host = "123.12.12.12";
        let port = 9999;
        let index = 1;
        let builder = SessionOptionsBuilder::default();
        let builder = builder.set_server_address(host, port, index);

        // Create additional server
        let new_host = "123.1.1.1";
        let new_port = 1111;
        let new_index = 2;
        let builder = builder.set_server_address(new_host, new_port, new_index);
        let options = builder.build();
        options.create();

        let no_server_address = options.num_server_addresses()?;
        assert_eq!(no_server_address, 3);
        Ok(())
    }

    #[test]
    fn test_session_option_connect_timeout() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let ms = option.connect_timeout();
        assert_eq!(ms?, BLPAPI_DEFAULT_TIMEOUT);
        Ok(())
    }

    #[test]
    fn test_session_option_default_service() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let services = option.default_services();
        let mut default_services_mkt = BLPAPI_DEFAULT_SERVICE_IDENTIFIER_MKTDATA.to_owned();
        let default_services_ref = BLPAPI_DEFAULT_SERVICE_IDENTIFIER_REFDATA.to_owned();
        default_services_mkt.push_str(";");
        default_services_mkt.push_str(&default_services_ref);
        assert_eq!(services?, default_services_mkt);
        Ok(())
    }

    #[test]
    fn test_session_option_default_subscription_service() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let service = option.default_subscription_service();
        let default_services_mkt = BLPAPI_DEFAULT_SERVICE_IDENTIFIER_REFDATA.to_owned();
        assert_eq!(service?, default_services_mkt);
        Ok(())
    }

    #[test]
    fn test_session_option_default_topic_prefix() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let service = option.default_topic_prefix();
        let default_services_mkt = BLPAPI_DEFAULT_TOPIC_PREFIX.to_owned();
        assert_eq!(service?, default_services_mkt);
        Ok(())
    }
    #[test]
    fn test_session_option_allow_multiple_corr() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let service = option.allow_multiple_correlators_per_msg();
        assert_eq!(service?, true);
        Ok(())
    }
    #[test]
    fn test_session_option_max_pending_req() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let service = option.max_pending_requests();
        assert_eq!(service?, BLPAPI_DEFAULT_MAX_PENDING_REQUEST);
        Ok(())
    }

    #[test]
    fn test_session_option_authentication_options() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let service = option.authentication_options();
        assert_eq!(service?, BLPAPI_AUTHENTICATION_OS_LOGON);
        Ok(())
    }

    #[test]
    fn test_session_option_auto_restart_on_disconnection() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let service = option.auto_restart_on_disconnection();
        assert_eq!(service?, true);
        Ok(())
    }

    #[test]
    fn test_session_option_num_of_start_attempts() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let no = option.num_start_attempts();
        assert_eq!(no?, BLPAPI_DEFAULT_MAX_START_ATTEMPTS);
        Ok(())
    }

    #[test]
    fn test_session_option_max_event_queue_size() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let no = option.max_event_queue_size();
        assert_eq!(no?, BLPAPI_DEFAULT_MAX_EVENT_QUEUE_SIZE);
        Ok(())
    }

    #[test]
    fn test_session_option_get_low_water_mark() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let no = option.slow_consumer_warning_lo_water_mark();
        assert_eq!(no?, BLPAPI_DEFAULT_LOW_WATER_MARK);
        Ok(())
    }

    #[test]
    fn test_session_option_get_high_water_mark() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let no = option.slow_consumer_warning_hi_water_mark();
        assert_eq!(no?, BLPAPI_DEFAULT_HIGH_WATER_MARK);
        Ok(())
    }

    #[test]
    fn test_session_option_default_keep_alive_inac_time() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let no = option.default_keep_alive_inactivity_time();
        assert_eq!(no?, BLPAPI_DEFAULT_KEEP_ALIVE_INACTIVITY_TIME);
        Ok(())
    }

    #[test]
    fn test_session_option_default_keep_alive_resp_to() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let no = option.default_keep_alive_response_timeout();
        assert_eq!(no?, BLPAPI_DEFAULT_KEEP_ALIVE_RESPONSE_TIMEOUT);
        Ok(())
    }

    #[test]
    fn test_session_option_keep_alive_enabled() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let service = option.keep_alive_enabled();
        assert_eq!(service?, true);
        Ok(())
    }

    #[test]
    fn test_session_option_status_record_data_receive_times() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let service = option.record_subscription_data_receive_times();
        assert_eq!(service?, true);
        Ok(())
    }

    #[test]
    fn test_session_option_service_check_timeout() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let service = option.service_check_timeout();
        assert_eq!(service?, BLPAPI_DEFAULT_SERVICE_CHECK_TIMEOUT);
        Ok(())
    }

    #[test]
    fn test_session_option_service_download_timeout() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let service = option.service_download_timeout();
        assert_eq!(service?, BLPAPI_DEFAULT_SERVICE_DOWNLOAD_TIMEOUT);
        Ok(())
    }

    #[test]
    fn test_session_option_flush_published_events_to() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let service = option.flush_published_events_timeout();
        assert_eq!(service?, BLPAPI_DEFAULT_FLUSH_PUBLISHED_EVENTS_TIMEOUT);
        Ok(())
    }

    #[test]
    fn test_session_option_band_width_disbaled_status() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let service = option.band_width_save_mode_disabled();
        assert_eq!(service?, BLPAPI_DEFAULT_BANDWIDTH_SAVE_MODE);
        Ok(())
    }

    #[test]
    fn test_session_option_application_id_key() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let service = option.application_identity_key();
        assert_eq!(service?, BLPAPI_DEFAULT_APPLICATION_IDENTIFICATION_KEY);
        Ok(())
    }

    #[test]
    fn test_session_option_session_name() -> Result<(), Error> {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let service = option.session_name();
        assert_eq!(service?, BLPAPI_DEFAULT_SESSION_NAME);
        Ok(())
    }
    #[test]
    fn test_session_option_print() {
        let builder = SessionOptionsBuilder::default();
        let option = builder.build();
        option.create();
        let mut output_buffer = Vec::new();
        let res = option.print(
            &mut output_buffer,
            2,
            4,
        );
        assert!(res.is_ok());
        let output_string = String::from_utf8(output_buffer).unwrap();
        println!("{}", output_string);
    }
}
