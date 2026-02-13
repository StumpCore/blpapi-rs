use crate::auth_options::AuthOptions;
use crate::core::*;
use crate::correlation_id::CorrelationId;
use crate::socks_5_config::Socks5Config;
use crate::tls_options::TlsOptions;
use crate::Error;
use blpapi_sys::*;
use core::ffi::{c_char, c_int, c_uint, c_ushort, c_void, CStr};
use regex::Regex;
use std::ffi::CString;
use std::io::Write;
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
    pub auth_options: Option<AuthOptions>,
    pub correlation_id: Option<CorrelationId>,
}

/// A SessionOptions
#[derive(Debug, Clone)]
pub struct SessionOptionsData {
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
    pub auth_options: Option<AuthOptions>,
    pub correlation_id: Option<CorrelationId>,
}

/// A SessionOptions
#[derive(Debug)]
pub struct SessionOptions {
    pub(crate) ptr: *mut blpapi_SessionOptions_t,
    pub data: SessionOptionsData,
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
            auth_options: None,
            correlation_id: None,
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
        let host_string: String = host.into();
        let new_address_host: String = host_string.clone();
        if self.server_host.is_none() && self.server_port.is_none() && self.server_index.is_none() {
            self = self.set_server_host(host_string);
            self = self.set_server_port(port);
            self = self.set_index(index);
        }
        let new_address = ServerAddress {
            host: new_address_host,
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
        let id: String = service_id.into();
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
            true => {
                self.topic_prefix = Some(id);
            }
            false => {
                println!(
                    "Invalid topic prefix or format. Setting to default: {}",
                    BLPAPI_DEFAULT_TOPIC_PREFIX
                );
                self.topic_prefix = Some(BLPAPI_DEFAULT_TOPIC_PREFIX.into());
            }
        };
        self
    }

    /// Setting allowance of multiple correlation IDs with a message
    pub fn set_allow_multiple_correlators_per_msg(mut self, allow: bool) -> Self {
        match allow {
            true => self.multiple_corr_per_msg = Some(0usize),
            false => self.multiple_corr_per_msg = Some(1usize),
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
        match option {
            true => self.auto_restart = Some(0usize),
            false => self.auto_restart = Some(1usize),
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
        let low = match (0.0..=1.0).contains(&low) {
            true => low,
            false => BLPAPI_DEFAULT_LOW_WATER_MARK,
        };
        let high = match (0.0..=1.0).contains(&high) {
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
            false => {
                self.keep_alive_inactivity_time = Some(BLPAPI_DEFAULT_KEEP_ALIVE_INACTIVITY_TIME)
            }
        };
        self
    }

    /// Setting default keep alive response timeout
    pub fn set_default_keep_alive_response_timeout(mut self, ms: isize) -> Self {
        match ms >= 0 {
            true => self.keep_alive_response_timeout = Some(ms),
            false => {
                self.keep_alive_response_timeout = Some(BLPAPI_DEFAULT_KEEP_ALIVE_RESPONSE_TIMEOUT)
            }
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
            false => {
                self.flush_published_events_timeout =
                    Some(BLPAPI_DEFAULT_FLUSH_PUBLISHED_EVENTS_TIMEOUT)
            }
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

    /// Setting AuthOptions
    pub fn set_auth_options(mut self, auth_options: AuthOptions) -> Self {
        self.auth_options = Some(auth_options);
        self
    }

    /// Setting CorrelationId
    pub fn set_correlation_id(mut self, cid: CorrelationId) -> Self {
        self.correlation_id = Some(cid);
        self
    }

    /// Builder function
    pub fn build(self) -> SessionOptions {
        let data = SessionOptionsData {
            server_host: self.server_host.expect("Expected server host"),
            server_port: self.server_port.expect("Expected server port"),
            server_index: self.server_index.expect("Expected server index"),
            server_addresses: self.server_addresses.expect("Expected server addresses"),
            timeout: self.timeout.expect("Expected timeout"),
            services: self.services.expect("Expected subscription services"),
            topic_prefix: self.topic_prefix.expect("Expected topic prefix"),
            multiple_corr_per_msg: self
                .multiple_corr_per_msg
                .expect("Expect multiple_corr_per_msg"),
            client_mode: self.client_mode.expect("Expected client mode"),
            authentication: self.authentication.expect("Expected authentication option"),
            auto_restart: self.auto_restart.expect("Expected auto restart"),
            max_pending_request: self
                .max_pending_request
                .expect("Expected max_request_tries"),
            max_start_attempts: self
                .max_start_attempts
                .expect("Expected max_start_attempts"),
            max_queue_size: self.max_queue_size.expect("Expected max queue size"),
            slow_consumer_warning_low_water_mark: self
                .slow_consumer_warning_low_water_mark
                .expect("Expected low water mark"),
            slow_consumer_warning_high_water_mark: self
                .slow_consumer_warning_high_water_mark
                .expect("Expected high water mark"),
            keep_alive: self.keep_alive.expect("Expected keep alive"),
            keep_alive_inactivity_time: self
                .keep_alive_inactivity_time
                .expect("Expected keep alive inactivity time"),
            keep_alive_response_timeout: self
                .keep_alive_response_timeout
                .expect("Expect keep alive response timeout"),
            record_subscription: self
                .record_subscription
                .expect("Expected record subscription"),
            service_check_timeout: self
                .service_check_timeout
                .expect("Expected service check timeout"),
            service_download_timeout: self
                .service_download_timeout
                .expect("Expected service download timeout"),
            flush_published_events_timeout: self
                .flush_published_events_timeout
                .expect("Expected flush published events timeout"),
            session_name: self.session_name.expect("Expected session name"),
            tls_options: self.tls_options.expect("Expected TLS options"),
            bandwidth_save_mode: self
                .bandwidth_save_mode
                .expect("Expected bandwidth save mode"),
            application_identifier: self
                .application_identifier
                .expect("Expected application identifier"),
            socks_5_config: self.socks_5_config,
            auth_options: self.auth_options,
            correlation_id: self.correlation_id,
        };
        SessionOptions {
            ptr: self.ptr.expect("Expected pointer"),
            data,
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
                auth_options: None,
                correlation_id: None,
            }
        }
    }
}

impl SessionOptions {
    pub fn create(&self) {
        // Creating a new instance based on the provided parameter
        let server_host_con =
            CString::new(&*self.data.server_host).expect("Failed to generated host");
        let server_port_con = self.data.server_port;
        unsafe {
            blpapi_SessionOptions_setServerHost(self.ptr, server_host_con.as_ptr());
            blpapi_SessionOptions_setServerPort(self.ptr, server_port_con as c_ushort);
        }
        for adr in self.data.server_addresses.iter() {
            let server_host = adr.host.clone();
            let server_port = adr.port;
            let server_index = adr.index;
            let host: CString;
            match &self.data.socks_5_config {
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
                None => unsafe {
                    host = CString::new(server_host).expect("Failed to generated host");
                    let res = blpapi_SessionOptions_setServerAddress(
                        self.ptr,
                        host.as_ptr(),
                        server_port as c_ushort,
                        server_index,
                    );
                    if res != 0 {
                        panic!("Failed to set server address");
                    }
                },
            };
        }

        // Setting die services
        let default_service = CString::new(BLPAPI_DEFAULT_SERVICE_IDENTIFIER_REFDATA)
            .expect("Failed to generate service identifier");
        unsafe {
            blpapi_SessionOptions_setDefaultSubscriptionService(self.ptr, default_service.as_ptr());
        }
        let topic_prefix =
            CString::new(&*self.data.topic_prefix).expect("Failed to generated topic prefix");
        let session_name =
            CString::new(&*self.data.session_name).expect("Failed to generated session name");
        let session_name_len = self.data.session_name.len();
        let aik = CString::new(&*self.data.application_identifier)
            .expect("Failed to generate application identifier");
        let aik_len = self.data.application_identifier.len();
        let auth = match self.data.authentication {
            Authentication::OsLogon => BLPAPI_AUTHENTICATION_OS_LOGON,
            Authentication::DirectoryService => BLPAPI_AUTHENTICATION_DIRECTORY_SERVICE,
            Authentication::ApplicationOnly => BLPAPI_AUTHENTICATION_APPLICATION_ONLY,
            Authentication::AppnameAndKey => BLPAPI_AUTHENTICATION_APPNAME_AND_KEY,
        };
        let c_auth = CString::new(auth).expect("Failed to generate authentication");

        let keep_alive = match self.data.keep_alive {
            true => 0,
            false => 1,
        };
        let bandwidth = match self.data.bandwidth_save_mode {
            true => 0,
            false => 1,
        };
        let restart = self.data.auto_restart as c_int;

        unsafe {
            blpapi_SessionOptions_setAutoRestartOnDisconnection(self.ptr, restart);
            blpapi_SessionOptions_setConnectTimeout(self.ptr, self.data.timeout as c_uint);
            blpapi_SessionOptions_setDefaultTopicPrefix(self.ptr, topic_prefix.as_ptr());

            blpapi_SessionOptions_setMaxPendingRequests(
                self.ptr,
                self.data.max_pending_request as c_int,
            );
            blpapi_SessionOptions_setNumStartAttempts(
                self.ptr,
                self.data.max_start_attempts as c_int,
            );
            blpapi_SessionOptions_setMaxEventQueueSize(self.ptr, self.data.max_queue_size);
            blpapi_SessionOptions_setSlowConsumerWarningLoWaterMark(
                self.ptr,
                self.data.slow_consumer_warning_low_water_mark,
            );
            blpapi_SessionOptions_setSlowConsumerWarningHiWaterMark(
                self.ptr,
                self.data.slow_consumer_warning_high_water_mark,
            );
            blpapi_SessionOptions_setDefaultKeepAliveInactivityTime(
                self.ptr,
                self.data.keep_alive_inactivity_time as c_int,
            );
            blpapi_SessionOptions_setDefaultKeepAliveResponseTimeout(
                self.ptr,
                self.data.keep_alive_response_timeout as c_int,
            );
            blpapi_SessionOptions_setKeepAliveEnabled(self.ptr, keep_alive as c_int);
            blpapi_SessionOptions_setServiceCheckTimeout(
                self.ptr,
                self.data.service_check_timeout as c_int,
            );
            blpapi_SessionOptions_setServiceDownloadTimeout(
                self.ptr,
                self.data.service_download_timeout as c_int,
            );
            blpapi_SessionOptions_setFlushPublishedEventsTimeout(
                self.ptr,
                self.data.flush_published_events_timeout as c_int,
            );
            blpapi_SessionOptions_setSessionName(self.ptr, session_name.as_ptr(), session_name_len);
            blpapi_SessionOptions_setBandwidthSaveModeDisabled(self.ptr, bandwidth as c_int);
            blpapi_SessionOptions_setApplicationIdentityKey(self.ptr, aik.as_ptr(), aik_len);
            blpapi_SessionOptions_setAuthenticationOptions(self.ptr, c_auth.as_ptr());
            match (&self.data.auth_options, self.data.correlation_id) {
                (Some(auth), Some(correlation_id)) => {
                    let auth_ptr = auth.ptr;
                    let mut cid_ptr = correlation_id.id;

                    let i = blpapi_SessionOptions_setSessionIdentityOptions(
                        self.ptr,
                        auth_ptr as *const blpapi_AuthOptions_t,
                        &mut cid_ptr,
                    );
                    if i != 0 {
                        panic!("Failed to set session identity");
                    };
                }
                _ => (),
            };
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
            blpapi_SessionOptions_getServerAddress(self.ptr, &mut host_ptr, &mut port, index)
        };
        if res != 0 {
            return Err(Error::struct_error(
                "SessionOptions",
                "get_server_address",
                "Error when trying to receive Server Address",
            ));
        };

        if host_ptr.is_null() {
            return Err(Error::NotFound(
                "Server address not found for index".to_string(),
            ));
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
        let server_host_ptr: *const c_char = ptr::null();
        let server_port: c_ushort = 0;
        let socks5_host_ptr: *const c_char = ptr::null();
        let socks_config = self
            .data
            .socks_5_config
            .clone()
            .expect("Expect Socks5 Config");
        let port: c_ushort = socks_config.port;

        unsafe {
            let res = blpapi_SessionOptions_getServerAddressWithProxy(
                self.ptr,
                server_host_ptr as *mut *const c_char,
                server_port as *mut c_ushort,
                socks5_host_ptr as *mut *const c_char,
                port as *mut _,
                index,
            ) as i32;

            if res != 0 {
                return Err(Error::struct_error(
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
            let res = blpapi_SessionOptions_removeServerAddress(self.ptr, index);
            if res != 0 {
                return Err(Error::struct_error(
                    "SessionOptions",
                    "remove_server_address",
                    "Error when trying to remove Server Address",
                ));
            };
        }
        let current_addresses = &mut self.data.server_addresses;
        current_addresses.remove(index);
        for adr in current_addresses.iter_mut() {
            if adr.index >= index {
                adr.index -= 1;
            }
        }

        Ok(())
    }

    /// Get the number of serveraddresses
    pub fn num_server_addresses(&self) -> Result<i16, Error> {
        let adr = unsafe { blpapi_SessionOptions_numServerAddresses(self.ptr) };

        match adr >= 0 {
            true => Ok(adr as i16),
            false => Err(Error::NotFound(format!(
                "Invalid amount of server addresses"
            ))),
        }
    }

    /// Get the time (milliseconds) of connection timeout
    pub fn connect_timeout(&self) -> Result<u32, Error> {
        let to = unsafe { blpapi_SessionOptions_connectTimeout(self.ptr) as u32 };
        match to > 0 {
            true => Ok(to),
            false => Err(Error::struct_error(
                "SessionOptions",
                "connect_timeout",
                "Error when trying to receive connect timeout",
            )),
        }
    }

    /// Get value of the allow multiple correlators per message
    pub fn allow_multiple_correlators_per_msg(&self) -> Result<bool, Error> {
        let res = unsafe { blpapi_SessionOptions_allowMultipleCorrelatorsPerMsg(self.ptr) };

        match res == 0 {
            true => Ok(true),
            false => Err(Error::struct_error(
                "SessionOptions",
                "allow_multiple_correlators_per_msg",
                "Error when trying to receive status of allow multiple correlators per msg",
            )),
        }
    }

    pub fn max_pending_requests(&self) -> Result<u16, Error> {
        let max_req = unsafe { blpapi_SessionOptions_maxPendingRequests(self.ptr) } as u16;
        match max_req > 0 {
            true => Ok(max_req),
            false => Err(Error::struct_error(
                "SessionOptions",
                "max_pending_requests",
                "Error when trying to receive status of max pending requests",
            )),
        }
    }

    /// Get the default service
    pub fn default_services(&self) -> Result<String, Error> {
        let res = unsafe { blpapi_SessionOptions_defaultServices(self.ptr) };

        let c_services = unsafe { CStr::from_ptr(res).to_owned() };
        let c_str = c_services.to_string_lossy().into_owned();
        match c_str.len() > 0 {
            true => Ok(c_str),
            false => Err(Error::struct_error(
                "SessionOptions",
                "default_services",
                "Error when trying to receive default services string",
            )),
        }
    }

    ///Get current default subscription service
    pub fn default_subscription_service(&self) -> Result<String, Error> {
        let service = unsafe { blpapi_SessionOptions_defaultSubscriptionService(self.ptr) };
        let c_services = unsafe { CStr::from_ptr(service).to_owned() };
        let c_str = c_services.to_string_lossy().into_owned();
        match c_str.len() > 0 {
            true => Ok(c_str),
            false => Err(Error::struct_error(
                "SessionOptions",
                "default_subscription_service",
                "Error when trying to receive default subscription service string",
            )),
        }
    }

    /// Get the defaul Topic prefix
    pub fn default_topic_prefix(&self) -> Result<String, Error> {
        let service = unsafe { blpapi_SessionOptions_defaultTopicPrefix(self.ptr) };
        let c_services = unsafe { CStr::from_ptr(service).to_owned() };
        let c_str = c_services.to_string_lossy().into_owned();
        match c_str.len() > 0 {
            true => Ok(c_str),
            false => Err(Error::struct_error(
                "SessionOptions",
                "default_topic_prefix",
                "Error when trying to receive default topic prefix string",
            )),
        }
    }

    /// Getting authentication options string
    pub fn authentication_options(&self) -> Result<String, Error> {
        let res = unsafe { blpapi_SessionOptions_authenticationOptions(self.ptr) };
        let c_services = unsafe { CStr::from_ptr(res).to_owned() };
        let c_str = c_services.to_string_lossy().into_owned();
        match c_str.len() > 0 {
            true => Ok(c_str),
            false => Err(Error::struct_error(
                "SessionOptions",
                "authentication_options",
                "Error when trying to receive authentication options",
            )),
        }
    }

    /// Get the auto restart on disconnection status
    pub fn auto_restart_on_disconnection(&self) -> Result<bool, Error> {
        let res = unsafe { blpapi_SessionOptions_autoRestartOnDisconnection(self.ptr) };

        match res == 0 {
            true => Ok(true),
            false => Err(Error::struct_error(
                "SessionOptions",
                "auto_restart_on_disconnection",
                "Error when trying to receive status of auto restart on disconnection",
            )),
        }
    }

    /// Get the number of start attempts in milliseconds
    pub fn num_start_attempts(&self) -> Result<u16, Error> {
        let res = unsafe { blpapi_SessionOptions_numStartAttempts(self.ptr) };
        match res > 0 {
            true => Ok(res as u16),
            false => Err(Error::struct_error(
                "SessionOptions",
                "num_start_attempts",
                "Error when trying to receive number of start attempts",
            )),
        }
    }

    /// Get the number of max event queue size
    pub fn max_event_queue_size(&self) -> Result<usize, Error> {
        let res = unsafe { blpapi_SessionOptions_maxEventQueueSize(self.ptr) };
        match res > 0 {
            true => Ok(res as usize),
            false => Err(Error::struct_error(
                "SessionOptions",
                "max_event_queue_size",
                "Error when trying to receive number of max event queue size",
            )),
        }
    }

    /// Get the value for the slow consumer warning low water mark
    pub fn slow_consumer_warning_lo_water_mark(&self) -> Result<f32, Error> {
        let res = unsafe { blpapi_SessionOptions_slowConsumerWarningLoWaterMark(self.ptr) };
        match res > 0.0 {
            true => Ok(res),
            false => Err(Error::struct_error(
                "SessionOptions",
                "slow_consumer_warning_lo_water_mark",
                "Error when trying to receive value of slow consumer warning lo water mark",
            )),
        }
    }

    /// Get the value for the slow consumer warning high water mark
    pub fn slow_consumer_warning_hi_water_mark(&self) -> Result<f32, Error> {
        let res = unsafe { blpapi_SessionOptions_slowConsumerWarningHiWaterMark(self.ptr) };
        match res > 0.0 {
            true => Ok(res),
            false => Err(Error::struct_error(
                "SessionOptions",
                "slow_consumer_warning_hi_water_mark",
                "Error when trying to receive value of slow consumer warning high water mark",
            )),
        }
    }

    /// Get the value of the keep alive inactivety time
    pub fn default_keep_alive_inactivity_time(&self) -> Result<isize, Error> {
        let res = unsafe { blpapi_SessionOptions_defaultKeepAliveInactivityTime(self.ptr) };
        match res > 0 {
            true => Ok(res as isize),
            false => Err(Error::struct_error(
                "SessionOptions",
                "default_keep_alive_inactivity_time",
                "Error when trying to receive value of default keep-alive inactivity time",
            )),
        }
    }

    /// Get the value of the keep response timeout
    pub fn default_keep_alive_response_timeout(&self) -> Result<isize, Error> {
        let res = unsafe { blpapi_SessionOptions_defaultKeepAliveResponseTimeout(self.ptr) };
        match res > 0 {
            true => Ok(res as isize),
            false => Err(Error::struct_error(
                "SessionOptions",
                "default_keep_alive_response_timeout",
                "Error when trying to receive value of default keep-alive response timeout",
            )),
        }
    }

    /// Get the value of the keep alive status
    pub fn keep_alive_enabled(&self) -> Result<bool, Error> {
        let res = unsafe { blpapi_SessionOptions_keepAliveEnabled(self.ptr) };
        match res == 0 {
            true => Ok(true),
            false => Err(Error::struct_error(
                "SessionOptions",
                "default_keep_alive_enabled",
                "Error when trying to receive value of default keep-alive status",
            )),
        }
    }

    /// Get the value of record subscription data receive times
    pub fn record_subscription_data_receive_times(&self) -> Result<bool, Error> {
        let res = unsafe { blpapi_SessionOptions_recordSubscriptionDataReceiveTimes(self.ptr) };
        match res == 0 {
            true => Ok(true),
            false => Err(Error::struct_error(
                "SessionOptions",
                "record_subscription_data_receive_times",
                "Error when trying to receive status of record subscription data receive times",
            )),
        }
    }

    /// Get the value of service check timeout
    pub fn service_check_timeout(&self) -> Result<isize, Error> {
        let res = unsafe { blpapi_SessionOptions_serviceCheckTimeout(self.ptr) };
        match res >= 0 {
            true => Ok(res as isize),
            false => Err(Error::struct_error(
                "SessionOptions",
                "service_check_timeout",
                "Error when trying to receive status of record subscription data receive times",
            )),
        }
    }

    /// Get the value of service download timeout
    pub fn service_download_timeout(&self) -> Result<isize, Error> {
        let res = unsafe { blpapi_SessionOptions_serviceDownloadTimeout(self.ptr) };
        match res >= 0 {
            true => Ok(res as isize),
            false => Err(Error::struct_error(
                "SessionOptions",
                "service_download_timeout",
                "Error when trying to receive status of download timeout",
            )),
        }
    }

    /// Get the value of flush published events timeout
    pub fn flush_published_events_timeout(&self) -> Result<isize, Error> {
        let res = unsafe { blpapi_SessionOptions_flushPublishedEventsTimeout(self.ptr) };
        match res >= 0 {
            true => Ok(res as isize),
            false => Err(Error::struct_error(
                "SessionOptions",
                "flush_published_events_timeout",
                "Error when trying to receive value of the flush published events timeout",
            )),
        }
    }

    /// Get the value of band width save mode
    pub fn band_width_save_mode_disabled(&self) -> Result<bool, Error> {
        let res = unsafe { blpapi_SessionOptions_bandwidthSaveModeDisabled(self.ptr) };
        match res == 0 {
            true => Ok(true),
            false => Err(Error::struct_error(
                "SessionOptions",
                "band_width_save_mode_disabled",
                "Error when trying to receive status band width save mode disabled",
            )),
        }
    }

    /// Get the application identity key (AIK)
    pub fn application_identity_key(&self) -> Result<String, Error> {
        let mut app_id: *const c_char = ptr::null();
        let mut app_size: usize = self.data.application_identifier.len();
        unsafe {
            let res = blpapi_SessionOptions_applicationIdentityKey(
                &mut app_id as *mut _,
                &mut app_size as *mut _,
                self.ptr,
            );
            if res != 0 {
                return Err(Error::struct_error(
                    "SessionOptions",
                    "application_identity_key",
                    "Error when trying to receive application key",
                ));
            };
        };

        let app_key = unsafe { CStr::from_ptr(app_id).to_string_lossy().into_owned() };
        Ok(app_key)
    }
    /// Get the session name
    pub fn session_name(&self) -> Result<String, Error> {
        let mut session_name: *const c_char = ptr::null();
        let mut session_name_size: usize = self.data.application_identifier.len();
        unsafe {
            let res = blpapi_SessionOptions_sessionName(
                &mut session_name as *mut _,
                &mut session_name_size as *mut _,
                self.ptr,
            );
            if res != 0 {
                return Err(Error::struct_error(
                    "SessionOptions",
                    "session_name",
                    "Error when trying to receive session name",
                ));
            };
        };

        let ses_name = unsafe { CStr::from_ptr(session_name).to_string_lossy().into_owned() };
        Ok(ses_name)
    }

    /// Implementing the writer function to return the details about the SessionOptions
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
                return Err(Error::struct_error(
                    "SessionOptions",
                    "print",
                    "Error when trying to write to stream writer",
                ));
            };
        };
        Ok(())
    }

    /// Get client mode
    pub fn client_mode(&self) -> Result<ClientMode, Error> {
        let mode = unsafe { blpapi_SessionOptions_clientMode(self.ptr) };
        match mode as u32 {
            BLPAPI_CLIENTMODE_AUTO => Ok(ClientMode::Auto),
            BLPAPI_CLIENTMODE_DAPI => Ok(ClientMode::DApi),
            BLPAPI_CLIENTMODE_SAPI => Ok(ClientMode::SApi),
            BLPAPI_CLIENTMODE_COMPAT_33X => Ok(ClientMode::Compat33X),
            _ => {
                return Err(Error::struct_error(
                    "SessionOptions",
                    "client_mode",
                    "Error when trying to write to stream writer",
                ))
            }
        }
    }
}

impl Drop for SessionOptions {
    fn drop(&mut self) {
        unsafe { blpapi_SessionOptions_destroy(self.ptr) }
    }
}

impl Clone for SessionOptions {
    fn clone(&self) -> Self {
        unsafe {
            let new_ptr = blpapi_SessionOptions_create();
            blpapi_SessionOptions_copy(new_ptr, self.ptr);
            let data = self.data.clone();
            Self { ptr: new_ptr, data }
        }
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
        let data = SessionOptionsData {
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
            auth_options: None,
            correlation_id: None,
        };
        unsafe {
            SessionOptions {
                ptr: blpapi_SessionOptions_create(),
                data,
            }
        }
    }
}
