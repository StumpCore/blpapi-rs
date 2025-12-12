use blpapi::tls_options::*;
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
fn test_tlsoptions_filebuilder() {
    let new_cert = "New Cert";
    let new_pw = "New Pw";
    let new_name = "New Name";

    let builder = TlsOptionsFileBuilder::default();
    let tls = builder.cert_name(new_cert).name(new_name).password(new_pw);

    assert_eq!(tls.cert_name.unwrap(), new_cert);
    assert_eq!(tls.cc_name.unwrap(), new_name);
    assert_eq!(tls.cc_password.unwrap(), new_pw);

    let builder = TlsOptionsFileBuilder::default();
    let tls = builder.cert_name(new_cert).name(new_name).password(new_pw);

    let _tls = tls.build();
}

#[test]
fn test_tlsoptions_from_files() {
    let tls = TlsOptionsFile::new("test", "testpw", "testcert");
    let _options = TlsOptions::create_from_files(tls);
}

#[test]
fn test_tlsoptions_blobs() {
    let blobs = TlsOptionsBlobs::default();
    let _options = TlsOptions::create_from_blobs(blobs);
}

#[test]
fn test_tlsoptions_blobsbuilder() {
    let new_raw_data = "New Raw Data";
    let new_raw_len = 10isize;
    let new_pw = "New Pw";
    let new_raw_cert = "New Cert Data";
    let new_raw_cert_len = 15isize;

    let builder = TlsOptionsBlobsBuilder::default();
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

    let builder = TlsOptionsBlobsBuilder::default();
    let tls = builder
        .cert_raw_data(new_raw_cert)
        .cert_raw_data_length(new_raw_cert_len)
        .cc_raw_data(new_raw_data)
        .cc_raw_data_length(new_raw_len)
        .cc_password(new_pw);

    let _options = tls.build();
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
