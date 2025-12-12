use crate::core::{
    BLPAPI_DEFAULT_CORRELATION_CLASS_ID, BLPAPI_DEFAULT_CORRELATION_ID,
    BLPAPI_DEFAULT_CORRELATION_INT_VALUE,
};
use blpapi_sys::*;
use std::ffi::c_void;
use std::os::raw::c_uint;

/// ValueType
#[derive(Debug)]
pub enum ValueType {
    UnsetValue,
    IntValue(u64),
    PointerValue(*mut c_void),
    SmartPointerValue(Box<dyn std::fmt::Debug>),
    AutogenValue(u64),
}
/// Correlation ID builder
#[derive(Debug)]
pub struct CorrelationIdBuilder {
    pub value: Option<u64>,
    pub reserved: Option<u64>,
    pub value_type: Option<u32>,
    pub class_id: Option<u32>,
}

impl CorrelationIdBuilder {
    pub fn new() -> Self {
        Self {
            value: None,
            reserved: None,
            value_type: None,
            class_id: None,
        }
    }

    /// setting value type
    pub fn set_value_type(mut self, value_type: ValueType) -> Self {
        self.value_type = Some(Self::get_value_type(&value_type));

        match value_type {
            ValueType::IntValue(value) => {
                self.value = Some(value);
            }
            ValueType::AutogenValue(value) => {
                self.value = Some(value);
            }
            ValueType::PointerValue(value) => {
                self.value = Some(value as u64);
            }
            ValueType::SmartPointerValue(value) => {
                let raw_p = Box::into_raw(value) as *mut c_void;
                self.value = Some(raw_p as u64);
            }
            ValueType::UnsetValue => {
                self.value = Some(0);
            }
        }
        self
    }

    /// setting class id
    pub fn set_class_id(mut self, class_id: u32) -> Self {
        match class_id {
            0..=BLPAPI_CORRELATION_MAX_CLASS_ID => {
                self.class_id = Some(class_id);
            }
            _ => {
                self.class_id = None;
            }
        }
        self
    }

    /// setting int value
    pub fn set_reserved(mut self, reserved: u64) -> Self {
        self.reserved = Some(reserved);
        self
    }

    /// Get the corresponding valuetype in integer form
    fn get_value_type(value_type: &ValueType) -> u32 {
        match value_type {
            ValueType::UnsetValue => BLPAPI_CORRELATION_TYPE_UNSET,
            ValueType::IntValue(_) => BLPAPI_CORRELATION_TYPE_INT,
            ValueType::PointerValue(_) => BLPAPI_CORRELATION_TYPE_POINTER,
            ValueType::AutogenValue(_) => BLPAPI_CORRELATION_TYPE_AUTOGEN,
            ValueType::SmartPointerValue(_) => BLPAPI_CORRELATION_TYPE_POINTER,
        }
    }

    /// builder of the CorrelationIdBuilder
    pub fn build(self) -> CorrelationId {
        let value = self.value.expect("Expected value (u64 or pointer)");
        let class_id = self.class_id.expect("Expected class ID");
        let reserved = self.reserved.expect("Expected int value");
        let value_type = self.value_type.expect("Expected value type");

        let mut id = unsafe {
            let size = std::mem::size_of::<blpapi_CorrelationId_t>() as c_uint;
            let mut id = core::mem::zeroed::<blpapi_CorrelationId_t_>();
            id.set_size(size);
            id.set_valueType(value_type as c_uint);
            id.set_classId(class_id as c_uint);
            id.set_reserved(reserved as c_uint);
            id.value.intValue = value;
            id
        };

        CorrelationId {
            id: &mut id,
            value,
            class_id,
            value_type,
            reserved,
        }
    }
}

/// Default of CorrelationIdBuilder
impl Default for CorrelationIdBuilder {
    fn default() -> Self {
        CorrelationIdBuilder {
            value: Some(BLPAPI_DEFAULT_CORRELATION_ID),
            value_type: Some(BLPAPI_CORRELATION_TYPE_UNSET),
            class_id: Some(BLPAPI_DEFAULT_CORRELATION_CLASS_ID),
            reserved: Some(BLPAPI_DEFAULT_CORRELATION_INT_VALUE),
        }
    }
}

/// A Correlation Id
#[derive(Copy, Clone, Debug)]
pub struct CorrelationId {
    pub id: *mut blpapi_CorrelationId_t,
    pub value: u64,
    pub value_type: u32,
    pub class_id: u32,
    pub reserved: u64,
}

impl CorrelationId {
    pub fn new_u64(value: u64) -> Self {
        let size = std::mem::size_of::<blpapi_CorrelationId_t>() as c_uint;
        let value_type = BLPAPI_CORRELATION_TYPE_INT;
        let new_value_typ = BLPAPI_CORRELATION_TYPE_INT;
        let class_id = BLPAPI_DEFAULT_CORRELATION_CLASS_ID;
        let reserved = BLPAPI_DEFAULT_CORRELATION_INT_VALUE;
        let reserved_c = reserved as c_uint;
        let _bitfield_1 = blpapi_CorrelationId_t_::new_bitfield_1(
            size,
            value_type,
            class_id as c_uint,
            reserved_c,
        );
        let new_value = blpapi_CorrelationId_t___bindgen_ty_1 { intValue: value };

        let mut id = blpapi_CorrelationId_t_ {
            _bitfield_align_1: [],
            value: new_value,
            _bitfield_1,
        };

        CorrelationId {
            id: &mut id,
            value,
            value_type: new_value_typ,
            class_id,
            reserved,
        }
    }
    /// get the current size of the correlation id
    pub fn size(&self) -> u32 {
        unsafe {
            let id = *self.id;
            id.size()
        }
    }

    /// get the user defined classId
    pub fn class_id(&self) -> u32 {
        unsafe {
            let id = *self.id;
            id.classId()
        }
    }

    /// get the value type u32 value
    pub fn value_type(&self) -> u32 {
        unsafe {
            let id = *self.id;
            id.valueType()
        }
    }

    /// get the reserved value
    pub fn reserved(&self) -> u32 {
        unsafe {
            let id = *self.id;
            id.reserved()
        }
    }
}
