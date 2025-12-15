use blpapi_sys::{
    blpapi_Char_t, blpapi_ConstantList_getConstantAt, blpapi_ConstantList_t,
    blpapi_Constant_datatype, blpapi_Constant_description, blpapi_Constant_getValueAsChar,
    blpapi_Constant_t, blpapi_DataType_t_BLPAPI_DATATYPE_BOOL,
    blpapi_DataType_t_BLPAPI_DATATYPE_BYTE, blpapi_DataType_t_BLPAPI_DATATYPE_BYTEARRAY,
    blpapi_DataType_t_BLPAPI_DATATYPE_CHAR, blpapi_DataType_t_BLPAPI_DATATYPE_CHOICE,
    blpapi_DataType_t_BLPAPI_DATATYPE_CORRELATION_ID, blpapi_DataType_t_BLPAPI_DATATYPE_DATE,
    blpapi_DataType_t_BLPAPI_DATATYPE_DATETIME, blpapi_DataType_t_BLPAPI_DATATYPE_DECIMAL,
    blpapi_DataType_t_BLPAPI_DATATYPE_ENUMERATION, blpapi_DataType_t_BLPAPI_DATATYPE_FLOAT32,
    blpapi_DataType_t_BLPAPI_DATATYPE_FLOAT64, blpapi_DataType_t_BLPAPI_DATATYPE_INT32,
    blpapi_DataType_t_BLPAPI_DATATYPE_INT64, blpapi_DataType_t_BLPAPI_DATATYPE_SEQUENCE,
    blpapi_DataType_t_BLPAPI_DATATYPE_STRING, blpapi_DataType_t_BLPAPI_DATATYPE_TIME,
};
use std::{
    ffi::{c_int, CString},
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ptr::{null, null_mut},
};

use crate::Error;

/// Enumeration of DataType
#[derive(Debug)]
pub enum DataType {
    BlpBool,
    BlpChar,
    BlpByte,
    BlpInt32,
    BlpInt64,
    BlpFloat32,
    BlpFloat64,
    BlpString,
    BlpByteArray,
    BlpDate,
    BlpDatetime,
    BlpTime,
    BlpDecimal,
    BlpEnumeration,
    BlpSequence,
    BlpChoice,
    BlpCorrelationId,
    Unknown,
}

#[allow(non_upper_case_globals)]
impl From<c_int> for DataType {
    fn from(arg: c_int) -> Self {
        match arg {
            blpapi_DataType_t_BLPAPI_DATATYPE_BOOL => DataType::BlpBool,
            blpapi_DataType_t_BLPAPI_DATATYPE_CHAR => DataType::BlpChar,
            blpapi_DataType_t_BLPAPI_DATATYPE_BYTE => DataType::BlpByte,
            blpapi_DataType_t_BLPAPI_DATATYPE_INT32 => DataType::BlpInt32,
            blpapi_DataType_t_BLPAPI_DATATYPE_INT64 => DataType::BlpInt64,
            blpapi_DataType_t_BLPAPI_DATATYPE_FLOAT32 => DataType::BlpFloat32,
            blpapi_DataType_t_BLPAPI_DATATYPE_FLOAT64 => DataType::BlpFloat64,
            blpapi_DataType_t_BLPAPI_DATATYPE_STRING => DataType::BlpString,
            blpapi_DataType_t_BLPAPI_DATATYPE_BYTEARRAY => DataType::BlpByteArray,
            blpapi_DataType_t_BLPAPI_DATATYPE_DATE => DataType::BlpDate,
            blpapi_DataType_t_BLPAPI_DATATYPE_DATETIME => DataType::BlpDatetime,
            blpapi_DataType_t_BLPAPI_DATATYPE_TIME => DataType::BlpTime,
            blpapi_DataType_t_BLPAPI_DATATYPE_DECIMAL => DataType::BlpDecimal,
            blpapi_DataType_t_BLPAPI_DATATYPE_ENUMERATION => DataType::BlpEnumeration,
            blpapi_DataType_t_BLPAPI_DATATYPE_SEQUENCE => DataType::BlpSequence,
            blpapi_DataType_t_BLPAPI_DATATYPE_CHOICE => DataType::BlpChoice,
            blpapi_DataType_t_BLPAPI_DATATYPE_CORRELATION_ID => DataType::BlpCorrelationId,
            _ => DataType::Unknown,
        }
    }
}
impl Display for DataType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            DataType::BlpBool => write!(f, "Bool"),
            DataType::BlpChar => write!(f, "Char"),
            DataType::BlpByte => write!(f, "Byte"),
            DataType::BlpInt32 => write!(f, "Int32"),
            DataType::BlpInt64 => write!(f, "Int64"),
            DataType::BlpFloat32 => write!(f, "Float32"),
            DataType::BlpFloat64 => write!(f, "Float64"),
            DataType::BlpString => write!(f, "String"),
            DataType::BlpByteArray => write!(f, "Byte Array"),
            DataType::BlpDate => write!(f, "Date"),
            DataType::BlpDatetime => write!(f, "Datetime"),
            DataType::BlpTime => write!(f, "Time"),
            DataType::BlpDecimal => write!(f, "Decimal"),
            DataType::BlpEnumeration => write!(f, "Enumeration"),
            DataType::BlpSequence => write!(f, "Sequence"),
            DataType::BlpChoice => write!(f, "Choice"),
            DataType::BlpCorrelationId => write!(f, "Correlation ID"),
            DataType::Unknown => write!(f, "Unknown"),
        }
    }
}

// ConstantList
#[derive(Debug)]
pub struct ConstantList {
    pub(crate) ptr: *const blpapi_ConstantList_t,
}

impl Default for ConstantList {
    fn default() -> Self {
        let ptr = null();
        Self { ptr }
    }
}

impl ConstantList {
    pub fn get_constant_at(&self, index: usize) -> Result<Constant, Error> {
        if self.ptr.is_null() {
            return Err(Error::ConstantList);
        }
        let const_ptr = unsafe { blpapi_ConstantList_getConstantAt(self.ptr, index) };
        if const_ptr.is_null() {
            return Err(Error::ConstantList);
        }
        Ok(Constant { ptr: const_ptr })
    }
}

/// Char Struct
pub struct Char {
    pub(crate) ptr: *mut blpapi_Char_t,
}

impl Default for Char {
    fn default() -> Self {
        let ptr: *mut blpapi_Char_t = null_mut();
        Self { ptr }
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
        Self { ptr }
    }
}

impl Constant {
    pub fn data_type(&self) -> Result<DataType, Error> {
        if self.ptr.is_null() {
            return Err(Error::ConstantList);
        }
        let constant: *const blpapi_Constant_t = self.ptr;
        let dt = unsafe { blpapi_Constant_datatype(constant) } as i32;
        let dt_type = DataType::from(dt);
        match dt_type {
            DataType::Unknown => Err(Error::Constant),
            _ => Ok(dt_type),
        }
    }

    pub fn description(self) {
        let _data_type = unsafe {
            let constant: *const blpapi_Constant_t = self.ptr;
            blpapi_Constant_description(constant)
        };
    }
    pub fn get_value_as_char(self) {
        let char = null_mut();
        let _char_value =
            unsafe { blpapi_Constant_getValueAsChar(self.ptr as *const blpapi_Constant_t, char) };
    }
}
