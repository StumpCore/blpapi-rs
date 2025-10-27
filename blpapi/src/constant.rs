use blpapi_sys::{blpapi_Char_t, blpapi_ConstantList_getConstantAt, blpapi_ConstantList_t, blpapi_Constant_datatype, blpapi_Constant_description, blpapi_Constant_getValueAsChar, blpapi_Constant_t};
use std::ffi::CString;
// Implement Schema first before continousing with constant due to dependency

// ConstantList
#[derive(Debug)]
pub struct ConstantList {
    pub(crate) ptr: *const blpapi_ConstantList_t,
}

impl Default for ConstantList {
    fn default() -> Self {
        let ptr = std::ptr::null();
        Self {
            ptr
        }
    }
}

impl ConstantList {
    pub fn get_constant_at(&self, index: usize) -> Result<Constant, &'static str> {
        if self.ptr.is_null() {
            return Err("ConstantList pointer is null.");
        }
        unsafe {
            let constant_ptr = blpapi_ConstantList_getConstantAt(self.ptr, index);

            if constant_ptr.is_null() {
                return Err("Failed to retrieve constant at index, pointer is null.");
            }
            Ok(Constant { ptr: constant_ptr })
        }
    }
}
/// Char Struct
pub struct Char {
    pub(crate) ptr: *mut blpapi_Char_t,
}

impl Default for Char {
    fn default() -> Self {
        let ptr: *mut blpapi_Char_t = CString::default().into_raw();
        Self {
            ptr
        }
    }
}

///Constant Data Type
#[derive(Debug)]
pub struct Constant {
    pub(crate) ptr: *const blpapi_Constant_t,
}

/// Default for constant
impl Default for Constant {
    fn default() -> Self {
        let ptr: *const blpapi_Constant_t = std::ptr::null();
        Self {
            ptr,
        }
    }
}

impl Constant {
    pub fn datatype(self) {
        let _data_type = unsafe {
            let constant: *const blpapi_Constant_t = self.ptr;
            blpapi_Constant_datatype(constant)
        };
    }
    pub fn description(self) {
        let _data_type = unsafe {
            let constant: *const blpapi_Constant_t = self.ptr;
            blpapi_Constant_description(constant)
        };
    }
    pub fn get_value_as_char(self, buffer: Char) {
        let _char_value = unsafe {
            let buffer_c = buffer.ptr;
            blpapi_Constant_getValueAsChar(
                self.ptr as *const blpapi_Constant_t,
                buffer_c,
            )
        };
    }
}


#[cfg(test)]
mod tests {
    use crate::constant::{Char, Constant, ConstantList};
    #[test]
    fn test_constant_list_default_is_null() {
        let list = ConstantList::default();
        assert!(list.ptr.is_null(), "Default ConstantList pointer must be null.");
    }
    #[test]
    fn test_get_constant_at_with_null_pointer() {
        let list = ConstantList::default();
        let constant = list.get_constant_at(0);
        println!("{:?}", constant);
    }


    #[test]
    pub fn test_datatype() {
        let constant = Constant::default();
        let _con = constant.datatype();
    }

    #[test]
    pub fn test_get_value_char() {
        let constant = Constant::default();
        let char_val = Char::default();
        let _con = constant.get_value_as_char(char_val);
    }
}