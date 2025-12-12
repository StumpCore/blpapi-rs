use blpapi::socks_5_config::*;
#[test]
fn test_socks5_builder() {
    let socks_builder = Socks5ConfigBuilder::default();
    let socks_builder = socks_builder.set_host_name("localhost").unwrap();
    let socks_builder = socks_builder.set_host_name_size(20).unwrap();
    let socks_builder = socks_builder.set_port(8888);
    let socks_builder = socks_builder.set_index(1);

    assert_eq!(socks_builder.clone().host_name.unwrap(), "localhost");
    assert_eq!(socks_builder.clone().host_name_size.unwrap(), 20);
    assert_eq!(socks_builder.clone().port.unwrap(), 8888);
    assert_eq!(socks_builder.clone().index.unwrap(), 1);

    let socks_config = socks_builder.build();

    println!("{:?}", socks_config);
}

#[test]
fn test_socks5_default() {
    let socks_config = Socks5Config::default();
    println!("{:?}", socks_config);
}
#[test]
fn test_socks5_clone() {
    let socks_config = Socks5Config::default();
    let socks_config_copy = socks_config.clone();
    println!("{:?}", socks_config);
    println!("{:?}", socks_config_copy);
}

#[test]
fn test_socks5_drop() {
    let socks_config = Socks5Config::default();
    drop(socks_config);
}

#[test]
fn test_socks5_config_print() {
    let config = Socks5Config::default();
    println!("{:?}", config);
    let mut output_buffer = Vec::new();
    let res = config.print(&mut output_buffer, 2, 4);
    assert!(res.is_ok());
    let output_string = String::from_utf8(output_buffer).unwrap();
    println!("{}", output_string);
}
