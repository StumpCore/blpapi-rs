use std::ffi::{c_int, CStr};

use blpapi_sys::{blpapi_getVersionIdentifier, blpapi_getVersionInfo};

/// Version Info struct
#[derive(Debug)]
pub struct VersionInfo {
    pub major_version: isize,
    pub minor_version: isize,
    pub patch_version: isize,
    pub build_version: isize,
    pub version_identifier: String,
}

impl Default for VersionInfo {
    fn default() -> Self {
        let mut major_version: c_int = 0;
        let mut minor_version: c_int = 0;
        let mut patch_version: c_int = 0;
        let mut build_version: c_int = 0;

        unsafe {
            blpapi_getVersionInfo(
                &mut major_version,
                &mut minor_version,
                &mut patch_version,
                &mut build_version,
            );
        }

        let version_identifier = unsafe {
            let v_id = blpapi_getVersionIdentifier();
            let mut v_str = CStr::from_ptr(v_id).to_string_lossy().into_owned();
            if v_str.is_empty() {
                v_str = String::from("unknown");
            }
            v_str
        };

        VersionInfo {
            major_version: major_version as isize,
            minor_version: minor_version as isize,
            patch_version: patch_version as isize,
            build_version: build_version as isize,
            version_identifier,
        }
    }
}
