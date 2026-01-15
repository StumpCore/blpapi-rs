use crate::core::{
    BLPAPI_DEFAULT_CORRELATION_CLASS_ID, BLPAPI_DEFAULT_CORRELATION_ID,
    BLPAPI_DEFAULT_CORRELATION_INT_VALUE,
};
use blpapi_sys::*;
use core::fmt;
use std::ffi::c_void;
use std::os::raw::c_uint;

/// ValueType
#[derive(Debug)]
pub enum OwnValueType {
    UnsetValue,
    IntValue(u64),
    PointerValue(*mut c_void),
    SmartPointerValue(Box<dyn std::fmt::Debug>),
    AutogenValue(u64),
}

#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    UnsetValue,
    IntValue,
    PointerValue,
    AutogenValue,
}

impl From<u64> for ValueType {
    fn from(v: u64) -> Self {
        match v as u32 {
            BLPAPI_CORRELATION_TYPE_UNSET => ValueType::UnsetValue,
            BLPAPI_CORRELATION_TYPE_INT => ValueType::IntValue,
            BLPAPI_CORRELATION_TYPE_POINTER => ValueType::PointerValue,
            BLPAPI_CORRELATION_TYPE_AUTOGEN => ValueType::AutogenValue,
            _ => ValueType::UnsetValue,
        }
    }
}

/// Correlation ID builder
#[derive(Debug)]
pub struct CorrelationIdBuilder {
    pub value: Option<u64>,
    pub reserved: Option<u64>,
    pub value_type: Option<u64>,
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

    /// setting correlation id from pointer
    pub fn from_pointer(self, correlation_id: blpapi_CorrelationId_t) -> CorrelationId {
        let id = correlation_id;
        let size = correlation_id.size();
        let value = correlation_id.valueType() as u64;
        let value_type = value.into();
        let class_id = correlation_id.classId();
        #[cfg(target_os = "windows")]
        let reserved = correlation_id.reserved() as u64;
        #[cfg(target_os = "linux")]
        let reserved = correlation_id.internalClassId() as u64;

        CorrelationId {
            id,
            size,
            value,
            class_id,
            value_type,
            reserved,
        }
    }

    /// setting value type
    pub fn set_value_type(mut self, value_type: OwnValueType) -> Self {
        self.value_type = Some(Self::get_value_type(&value_type));

        match value_type {
            OwnValueType::IntValue(value) => {
                self.value = Some(value);
            }
            OwnValueType::AutogenValue(value) => {
                self.value = Some(value);
            }
            OwnValueType::PointerValue(value) => {
                self.value = Some(value as u64);
            }
            OwnValueType::SmartPointerValue(value) => {
                let raw_p = Box::into_raw(value) as *mut c_void;
                self.value = Some(raw_p as u64);
            }
            OwnValueType::UnsetValue => {
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
    fn get_value_type(value_type: &OwnValueType) -> u64 {
        let v_type = match value_type {
            OwnValueType::UnsetValue => BLPAPI_CORRELATION_TYPE_UNSET,
            OwnValueType::IntValue(_) => BLPAPI_CORRELATION_TYPE_INT,
            OwnValueType::PointerValue(_) => BLPAPI_CORRELATION_TYPE_POINTER,
            OwnValueType::AutogenValue(_) => BLPAPI_CORRELATION_TYPE_AUTOGEN,
            OwnValueType::SmartPointerValue(_) => BLPAPI_CORRELATION_TYPE_POINTER,
        };
        v_type as u64
    }

    /// builder of the CorrelationIdBuilder
    pub fn build(self) -> CorrelationId {
        let value = self.value.expect("Expected u64");
        let class_id = self.class_id.expect("Expected class ID");
        let reserved = self.reserved.expect("Expected int value");
        let value_type = self.value_type.expect("Expected value type");
        let size = std::mem::size_of::<blpapi_CorrelationId_t>() as c_uint;

        let id = unsafe {
            let mut id = core::mem::zeroed::<blpapi_CorrelationId_t_>();
            id.set_size(size);
            id.set_valueType(value_type as c_uint);
            id.set_classId(class_id as c_uint);
            #[cfg(target_os = "windows")]
            id.set_reserved(reserved as c_uint);
            #[cfg(target_os = "linux")]
            id.set_internalClassId(reserved as c_uint);
            id.value.intValue = value;
            id
        };
        let value_type = value_type.into();

        CorrelationId {
            id,
            size,
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
            value_type: Some(BLPAPI_CORRELATION_TYPE_UNSET as u64),
            class_id: Some(BLPAPI_DEFAULT_CORRELATION_CLASS_ID),
            reserved: Some(BLPAPI_DEFAULT_CORRELATION_INT_VALUE),
        }
    }
}

/// A Correlation Id
#[derive(Clone, Copy)]
pub struct CorrelationId {
    pub id: blpapi_CorrelationId_t,
    pub size: u32,
    pub value: u64,
    pub value_type: ValueType,
    pub class_id: u32,
    pub reserved: u64,
}

impl fmt::Debug for CorrelationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CorrelationId")
            .field("value", &self.value)
            .field("value_type", &self.value_type)
            .field("class_id", &self.class_id)
            // We can also peek into the raw bitfield values from the C struct
            // if you want to verify they match the Rust fields
            .field("c_size", &self.id.size())
            .field("c_type", &self.id.valueType())
            .finish()
    }
}

impl CorrelationId {
    pub fn new_u64(value: u64) -> Self {
        let size = std::mem::size_of::<blpapi_CorrelationId_t>() as c_uint;
        let value_type = BLPAPI_CORRELATION_TYPE_INT;
        let new_value_typ = BLPAPI_CORRELATION_TYPE_INT as u64;
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

        let id = blpapi_CorrelationId_t_ {
            _bitfield_align_1: [],
            value: new_value,
            _bitfield_1,
        };

        CorrelationId {
            id,
            size,
            value,
            value_type: new_value_typ.into(),
            class_id,
            reserved,
        }
    }
    /// get the current size of the correlation id
    pub fn size(&self) -> u32 {
        let id = self.id;
        id.size()
    }

    /// get the user defined classId
    pub fn class_id(&self) -> u32 {
        let id = self.id;
        id.classId()
    }

    /// get the value type u32 value
    pub fn value_type(&self) -> u32 {
        let id = self.id;
        id.valueType()
    }

    /// get the reserved value
    pub fn reserved(&self) -> u32 {
        let id = self.id;
        #[cfg(target_os = "windows")]
        let res = id.reserved();
        #[cfg(target_os = "linux")]
        let res = id.internalClassId();
        res
    }
}
