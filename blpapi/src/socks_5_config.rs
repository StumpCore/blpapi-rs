use crate::core::{BLPAPI_DEFAULT_HOST, BLPAPI_DEFAULT_PORT};
use crate::Error;
use blpapi_sys::{blpapi_Socks5Config_create, blpapi_Socks5Config_t};
use std::ffi::{c_ushort, CString};

/// Socks 5 Config Builder
#[derive(Debug, Clone)]
pub struct Socks5ConfigBuilder {
    pub ptr: Option<*mut blpapi_Socks5Config_t>,
    pub host_name: Option<String>,
    pub host_name_size: Option<usize>,
    pub port: Option<u16>,
}

/// Socks 5 Config
#[derive(Debug, Clone)]
pub struct Socks5Config {
    pub ptr: *mut blpapi_Socks5Config_t,
    pub host_name: String,
    pub host_name_size: usize,
    pub port: u16,
}

impl Socks5ConfigBuilder {
    pub fn new() -> Self {
        Self {
            ptr: None,
            host_name: None,
            host_name_size: None,
            port: None,
        }
    }
    pub fn set_host_name<T: Into<String>>(mut self, host: T) -> Result<Self, Error> {
        let binding = host.into();
        let chost = CString::new(&*binding);
        if chost.unwrap().is_empty() {
            return {
                Err(Error::session_options(
                    "Socks5ConfigBuilder",
                    "set_host_name",
                    "Invalid host name call",
                ))
            };
        };
        self.host_name = Some(binding);
        Ok(self)
    }

    /// Setting the new host_name_size
    /// In case the new host_name_size is smaller than the existing host_name
    /// the size is changed to fit the current host_name
    pub fn set_host_name_size(mut self, host_name_size: usize) -> Result<Self, Error> {
        if self.host_name.is_none() {
            return {
                Err(Error::session_options(
                    "Socks5ConfigBuilder",
                    "set_host_name_size",
                    "Consider setting a host_name first",
                ))
            };
        }
        let cur_host_name_size = if let Some(x) = &self.host_name {
            x.len()
        } else {
            return {
                Err(Error::session_options(
                    "Socks5ConfigBuilder",
                    "set_host_name_size",
                    "Consider setting a host_name first",
                ))
            };
        };

        self.host_name_size = if host_name_size > cur_host_name_size {
            Some(host_name_size)
        } else {
            Some(host_name_size)
        };

        Ok(self)
    }

    /// Setting new port
    pub fn set_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn build(self) -> Socks5Config {
        let host_name = self.host_name.unwrap_or_else(|| BLPAPI_DEFAULT_HOST.to_string());
        let c_host_name = CString::new(&*host_name).expect("CString::new failed");
        let host_name_size = host_name.len();
        let port = self.port.unwrap_or_else(|| BLPAPI_DEFAULT_PORT);
        let ptr = unsafe {
            blpapi_Socks5Config_create(
                c_host_name.as_ptr(),
                host_name_size,
                port as c_ushort,
            )
        };

        Socks5Config {
            ptr,
            host_name,
            host_name_size,
            port,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socks5_builder() {
        let socks_builder = Socks5ConfigBuilder::new();
        let socks_builder = socks_builder.set_host_name("localhost").unwrap();
        let socks_builder = socks_builder.set_host_name_size(20).unwrap();
        let socks_builder = socks_builder.set_port(8888);
        let socks_config = socks_builder.build();

        println!("{:?}", socks_config);
    }
}



