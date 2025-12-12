use blpapi::session_options::SessionOptions;
use blpapi::tls_options::TlsOptions;
use blpapi::zfp_util::{Remote, ZfpUtilBuilder};

#[test]
pub fn test_zfputil_builder() {
    let builder = ZfpUtilBuilder::default();
    assert_eq!(builder.remote, Remote::Remote8194);

    let builder = ZfpUtilBuilder::default();
    let builder = builder.set_remote(Remote::Remote8196);
    assert_eq!(builder.remote, Remote::Remote8196);

    let builder = ZfpUtilBuilder::default();
    let builder = builder.set_tls_options(TlsOptions::default());
    let comp_tls = TlsOptions::default();
    assert_eq!(builder.tls_options.crl_timeout, comp_tls.crl_timeout);

    let builder = ZfpUtilBuilder::default();
    let builder = builder.set_sessions_options(SessionOptions::default());
    let comp_sessions = SessionOptions::default();
    assert_eq!(
        builder.session_options.service_download_timeout,
        comp_sessions.service_download_timeout
    );
}

#[test]
pub fn test_zfp_util_builder_build() {
    let builder = ZfpUtilBuilder::default();
    let zfp_u = builder.build();
    assert_eq!(zfp_u.remote, 8194);
}

#[test]
pub fn test_zfp_util_get_lease() {
    let builder = ZfpUtilBuilder::default();
    let builder = builder.set_sessions_options(SessionOptions::default());
    let zfp_u = builder.build();
    let so = zfp_u.get_zfp_options_for_leased_lines();
    match so {
        Ok(_) => {
            println!("success to fix connection")
        }
        Err(e) => {
            println!("fail to fix connection: {}", e)
        }
    }
}
