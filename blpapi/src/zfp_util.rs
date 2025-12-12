use crate::errors::Error;
use crate::session_options::SessionOptions;
use crate::tls_options::TlsOptions;
use blpapi_sys::{
    blpapi_ZfpUtil_getOptionsForLeasedLines, BLPAPI_ZFPUTIL_REMOTE_8194, BLPAPI_ZFPUTIL_REMOTE_8196,
};
use std::ffi::c_int;

/// Implementing enum for the port setting
#[derive(Debug, PartialEq)]
pub enum Remote {
    Remote8194,
    Remote8196,
}

/// Zfp Util Builder struct
pub struct ZfpUtilBuilder {
    pub session_options: SessionOptions,
    pub remote: Remote,
    pub tls_options: TlsOptions,
}

/// Implementing default for the builder
impl Default for ZfpUtilBuilder {
    fn default() -> Self {
        let session_options: SessionOptions = Default::default();
        let tls_options = TlsOptions::default();
        let remote = Remote::Remote8194;
        Self {
            session_options,
            remote,
            tls_options,
        }
    }
}

/// Implementing setting for the Builder
impl ZfpUtilBuilder {
    /// Setting remote value
    pub fn set_remote(mut self, remote: Remote) -> Self {
        self.remote = remote;
        self
    }

    /// Setting tls_options value
    pub fn set_tls_options(mut self, tls_options: TlsOptions) -> Self {
        self.tls_options = tls_options;
        self
    }

    /// Setting Session Options
    pub fn set_sessions_options(mut self, session_options: SessionOptions) -> Self {
        self.session_options = session_options;
        self
    }

    /// Build ZfpUtil
    pub fn build(self) -> ZfpUtil {
        let remote = match self.remote {
            Remote::Remote8194 => BLPAPI_ZFPUTIL_REMOTE_8194,
            Remote::Remote8196 => BLPAPI_ZFPUTIL_REMOTE_8196,
        };

        ZfpUtil {
            session_options: self.session_options,
            remote,
            tls_options: self.tls_options,
        }
    }
}

/// ZfpUtil struct
#[derive(Debug)]
pub struct ZfpUtil {
    pub session_options: SessionOptions,
    pub remote: u32,
    pub tls_options: TlsOptions,
}

/// Default for ZfpUtil
impl Default for ZfpUtil {
    fn default() -> Self {
        let tls_options = TlsOptions::default();
        let remote = BLPAPI_ZFPUTIL_REMOTE_8194;
        let session_options: SessionOptions = Default::default();
        Self {
            session_options,
            remote,
            tls_options,
        }
    }
}

impl ZfpUtil {
    /// get the zfpoptions for lease lines
    pub fn get_zfp_options_for_leased_lines(self) -> Result<SessionOptions, Error> {
        let rem = self.remote;
        let tls_options = self.tls_options.ptr;
        let session_options = self.session_options.ptr;

        unsafe {
            let i = blpapi_ZfpUtil_getOptionsForLeasedLines(
                session_options,
                tls_options as *const _,
                rem as c_int,
            );

            if i != 0 {
                return Err(Error::struct_error(
                    "ZfpUtil",
                    "get_zfp_options_for_leased_lines() returned non-zero int",
                    "Invalid call of bloomberg private lease line. This can be due to \
                    missing permission rights to FIX",
                ));
            }
        }
        Ok(self.session_options)
    }
}

