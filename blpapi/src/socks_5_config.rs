use crate::core::{
    write_to_stream_cb, StreamWriterContext, BLPAPI_DEFAULT_HOST, BLPAPI_DEFAULT_INDEX,
    BLPAPI_DEFAULT_PORT,
};
use crate::Error;
use blpapi_sys::{
    blpapi_Socks5Config_copy, blpapi_Socks5Config_create, blpapi_Socks5Config_destroy,
    blpapi_Socks5Config_print, blpapi_Socks5Config_t,
};
use std::ffi::{c_int, c_ushort, c_void, CString};
use std::io::Write;

/// Socks 5 Config Builder
#[derive(Debug, Clone, Default)]
pub struct Socks5ConfigBuilder {
    pub host_name: Option<String>,
    pub host_name_size: Option<usize>,
    pub port: Option<u16>,
    pub index: Option<usize>,
}

/// Socks 5 Config
#[derive(Debug)]
pub struct Socks5Config {
    pub ptr: *mut blpapi_Socks5Config_t,
    pub host_name: String,
    pub host_name_size: usize,
    pub port: u16,
    pub index: usize,
}

impl Socks5ConfigBuilder {
    pub fn set_host_name<T: Into<String>>(mut self, host: T) -> Result<Self, Error> {
        let binding = host.into();
        let chost = CString::new(&*binding);
        if chost.unwrap().is_empty() {
            return {
                Err(Error::struct_error(
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
                Err(Error::struct_error(
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
                Err(Error::struct_error(
                    "Socks5ConfigBuilder",
                    "set_host_name_size",
                    "Consider setting a host_name first",
                ))
            };
        };

        self.host_name_size = if host_name_size > cur_host_name_size {
            Some(host_name_size)
        } else {
            Some(cur_host_name_size)
        };

        Ok(self)
    }

    /// Setting new port
    pub fn set_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Setting new index
    pub fn set_index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    pub fn build(self) -> Socks5Config {
        let host_name = self
            .host_name
            .unwrap_or_else(|| BLPAPI_DEFAULT_HOST.to_string());
        let c_host_name = CString::new(&*host_name).expect("CString::new failed");
        let host_name_size = host_name.len();
        let port = self.port.unwrap_or(BLPAPI_DEFAULT_PORT);
        let index = self.index.unwrap_or(BLPAPI_DEFAULT_INDEX);
        let ptr = unsafe {
            blpapi_Socks5Config_create(c_host_name.as_ptr(), host_name_size, port as c_ushort)
        };

        Socks5Config {
            ptr,
            host_name,
            host_name_size,
            port,
            index,
        }
    }
}

impl Socks5Config {
    pub fn print<T: Write>(&self, writer: &mut T, indent: i32, spaces: i32) -> Result<(), Error> {
        let mut context = StreamWriterContext { writer };
        let res = unsafe {
            blpapi_Socks5Config_print(
                self.ptr,
                Some(write_to_stream_cb),
                &mut context as *mut _ as *mut c_void,
                indent as c_int,
                spaces as c_int,
            )
        };
        Error::check(res)?;
        Ok(())
    }
}

/// Implementing the Default trait
impl Default for Socks5Config {
    fn default() -> Self {
        let ptr = unsafe {
            blpapi_Socks5Config_create(
                BLPAPI_DEFAULT_HOST.as_ptr() as *const _,
                BLPAPI_DEFAULT_HOST.len(),
                BLPAPI_DEFAULT_PORT as c_ushort,
            )
        };
        Self {
            ptr,
            host_name: BLPAPI_DEFAULT_HOST.into(),
            host_name_size: BLPAPI_DEFAULT_HOST.len(),
            port: BLPAPI_DEFAULT_PORT,
            index: BLPAPI_DEFAULT_INDEX,
        }
    }
}

/// Implementing the Clone Trait
impl Clone for Socks5Config {
    fn clone(&self) -> Self {
        let clone = Socks5Config::default();
        unsafe { blpapi_Socks5Config_copy(clone.ptr as *mut _, self.ptr) };
        clone
    }
}

/// Implementing Drop Trait
impl Drop for Socks5Config {
    fn drop(&mut self) {
        unsafe { blpapi_Socks5Config_destroy(self.ptr as *mut _) };
    }
}
