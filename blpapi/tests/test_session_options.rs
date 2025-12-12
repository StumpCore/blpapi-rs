use blpapi::{
    auth_options::*, core::*, correlation_id::*, session_options::*, socks_5_config::*,
    tls_options::*, Error,
};
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
    let builder = builder.set_server_address(host, port, index);
    assert_eq!(builder.server_host.unwrap(), host);
    assert_eq!(builder.server_port.unwrap(), port);
    assert_eq!(builder.server_index.unwrap(), index);
    Ok(())
}

#[test]
fn test_session_options_builder_set_server_address_socks5config() -> Result<(), Error> {
    let socks_builder = Socks5ConfigBuilder::default();
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
    assert_eq!(
        _builder.slow_consumer_warning_high_water_mark.unwrap(),
        high
    );
    assert_eq!(_builder.slow_consumer_warning_low_water_mark.unwrap(), low);

    // Change oder to test default
    let _builder = _builder.set_both_slow_consumer_warning_marks(high, low);
    assert_eq!(
        _builder.slow_consumer_warning_high_water_mark.unwrap(),
        BLPAPI_DEFAULT_HIGH_WATER_MARK
    );
    assert_eq!(
        _builder.slow_consumer_warning_low_water_mark.unwrap(),
        BLPAPI_DEFAULT_LOW_WATER_MARK
    );
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
    assert_eq!(
        _builder.service_check_timeout.unwrap(),
        BLPAPI_DEFAULT_SERVICE_CHECK_TIMEOUT
    );
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
    assert_eq!(
        _builder.service_download_timeout.unwrap(),
        BLPAPI_DEFAULT_SERVICE_DOWNLOAD_TIMEOUT
    );
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
    assert_eq!(
        _builder.flush_published_events_timeout.unwrap(),
        BLPAPI_DEFAULT_FLUSH_PUBLISHED_EVENTS_TIMEOUT
    );
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
fn test_session_options_builder_set_correlation_id() -> Result<(), Error> {
    let cid = CorrelationIdBuilder::default();
    let cid = cid.build();
    let authb = AuthOptionsBuilder::default();
    let auth = authb.build();
    let builder = SessionOptionsBuilder::default();
    let builder = builder.set_correlation_id(cid).set_auth_options(auth);
    let options = builder.build();
    let opt = options.create();
    println!("{:?}", options);
    println!("{:?}", opt);
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
    let socks_builder = Socks5ConfigBuilder::default();
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
        Err(e) => Err(e),
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
    default_services_mkt.push(';');
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
    assert!(service?);
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
    assert!(service?);
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
    assert!(service?);
    Ok(())
}

#[test]
fn test_session_option_status_record_data_receive_times() -> Result<(), Error> {
    let builder = SessionOptionsBuilder::default();
    let option = builder.build();
    option.create();
    let service = option.record_subscription_data_receive_times();
    assert!(service?);
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
    let res = option.print(&mut output_buffer, 2, 4);
    assert!(res.is_ok());
    let output_string = String::from_utf8(output_buffer).unwrap();
    println!("{}", output_string);
}

#[test]
fn test_session_option_client_mode() -> Result<(), Error> {
    let builder = SessionOptionsBuilder::default();
    let option = builder.build();
    option.create();
    let mode = option.client_mode();
    assert_eq!(mode?, ClientMode::Auto);
    Ok(())
}
