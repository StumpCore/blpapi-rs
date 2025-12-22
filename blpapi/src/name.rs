use blpapi_sys::*;
use once_cell::sync::Lazy;
use std::ffi::{CStr, CString};
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::string::ToString;
pub const DEFAULT_NAME: String = String::new();
pub static SECURITY_DATA: Lazy<Name> = Lazy::new(|| Name::new("securityData"));
pub static SECURITY_NAME: Lazy<Name> = Lazy::new(|| Name::new("security"));
pub static FIELD_DATA: Lazy<Name> = Lazy::new(|| Name::new("fieldData"));
pub static SECURITY_ERROR: Lazy<Name> = Lazy::new(|| Name::new("securityError"));
pub static SECURITIES: Lazy<Name> = Lazy::new(|| Name::new("securities"));
pub static FIELDS_NAME: Lazy<Name> = Lazy::new(|| Name::new("fields"));
pub static SESSION_TERMINATED: Lazy<Name> = Lazy::new(|| Name::new("SessionTerminated"));
pub static SESSION_STARTUP_FAILURE: Lazy<Name> = Lazy::new(|| Name::new("SessionStartupFailure"));

/// A 'Name' Builder
pub struct NameBuilder {
    pub name: Option<String>,
    pub length: usize,
}

/// Default Implementation of the Name struct
impl Default for NameBuilder {
    fn default() -> Self {
        Self {
            name: Some(DEFAULT_NAME.to_string()),
            length: 0,
        }
    }
}

impl NameBuilder {
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    pub fn by_ptr(mut self, ptr: *mut blpapi_Name_t) -> Self {
        unsafe {
            let len = blpapi_Name_length(ptr);
            let c_string = blpapi_Name_string(ptr);
            let slice = std::slice::from_raw_parts(c_string as *const u8, len + 1);
            let name_str = CStr::from_bytes_with_nul_unchecked(slice)
                .to_string_lossy()
                .to_string();

            self.name = Some(name_str);
            self.length = len;
        }
        self
    }

    pub fn build(self) -> Name {
        let n = self.name.expect("Expected Name. Provide Name first.");
        let name = CString::new(n.clone()).expect("CString failed.");
        let length = name.to_bytes_with_nul().len();
        unsafe {
            Name {
                ptr: blpapi_Name_create(name.as_ptr()),
                name: n,
                length,
            }
        }
    }
}

/// A `Name`
pub struct Name {
    pub(crate) ptr: *mut blpapi_Name_t,
    pub name: String,
    pub length: usize,
}

// As per bloomberg documentation:
// *Each Name instance refers to an entry in a global static table* thus Name is `Sync`
// https://bloomberg.github.io/blpapi-docs/dotnet/3.12/html/T_Bloomberglp_Blpapi_Name.htm
unsafe impl Sync for Name {}
unsafe impl Send for Name {}

impl Name {
    /// Create a new name
    pub fn new(s: &str) -> Self {
        let name_str = String::from(s);
        let name = CString::new(s).expect("CString failed.");
        let length = name.to_bytes_with_nul().len();
        unsafe {
            Name {
                ptr: blpapi_Name_create(name.as_ptr()),
                name: name_str,
                length,
            }
        }
    }

    /// Name length
    pub fn length(&mut self) -> usize {
        self.length = unsafe { blpapi_Name_length(self.ptr) };
        self.length
    }

    /// Find Name
    pub fn find_name(&self, name_string: &str) -> Self {
        let mut length = self.length;
        let name = CString::new(name_string).expect("CString failed.");
        let bbg_name = unsafe { blpapi_Name_findName(name.as_ptr() as *const _) };
        let final_ptr = if bbg_name.is_null() {
            self.ptr
        } else {
            length = unsafe { blpapi_Name_length(bbg_name) };
            bbg_name
        };
        Name {
            ptr: final_ptr,
            name: name_string.to_string(),
            length,
        }
    }

    /// Has Name
    pub fn has_name(name_string: &str) -> bool {
        let c_name = match CString::new(name_string) {
            Ok(s) => s,
            Err(_) => return false,
        };

        let fnd_ptr = unsafe { blpapi_Name_findName(c_name.as_ptr() as *const _) };
        !fnd_ptr.is_null()
    }
}

impl Deref for Name {
    type Target = CStr;
    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = blpapi_Name_string(self.ptr);
            let len = blpapi_Name_length(self.ptr);
            let slice = std::slice::from_raw_parts(ptr as *const u8, len + 1);
            CStr::from_bytes_with_nul_unchecked(slice)
        }
    }
}

/// Hash Implementation for Rust Hash Maps
impl Hash for Name {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.deref().hash(state);
    }
}

impl Drop for Name {
    fn drop(&mut self) {
        unsafe { blpapi_Name_destroy(self.ptr) }
    }
}

impl Clone for Name {
    fn clone(&self) -> Self {
        unsafe {
            let name_ptr = blpapi_Name_duplicate(self.ptr);
            let len = blpapi_Name_length(name_ptr);
            let slice = std::slice::from_raw_parts(name_ptr as *const u8, len + 1);
            let name_str = CStr::from_bytes_with_nul_unchecked(slice)
                .to_string_lossy()
                .to_string();
            Name {
                ptr: name_ptr,
                name: name_str,
                length: len,
            }
        }
    }
}

impl<S: AsRef<str>> PartialEq<S> for Name {
    fn eq(&self, other: &S) -> bool {
        let s = CString::new(other.as_ref()).expect("CString failed.");
        unsafe { blpapi_Name_equalsStr(self.ptr, s.as_ptr()) != 0 }
    }
}

impl PartialEq<Name> for Name {
    fn eq(&self, other: &Name) -> bool {
        self.ptr == other.ptr && self.length == other.length
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = unsafe {
            let ptr = blpapi_Name_string(self.ptr);
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        };
        write!(f, "{}", str)
    }
}

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Name: '{}'", self)
    }
}
