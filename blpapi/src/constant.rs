use blpapi_sys::{
    blpapi_Char_t, blpapi_ConstantList_datatype, blpapi_ConstantList_description,
    blpapi_ConstantList_getConstantAt, blpapi_ConstantList_name, blpapi_ConstantList_numConstants,
    blpapi_ConstantList_setUserData, blpapi_ConstantList_status, blpapi_ConstantList_t,
    blpapi_ConstantList_userData, blpapi_Constant_datatype, blpapi_Constant_description,
    blpapi_Constant_getValueAsChar, blpapi_Constant_t, blpapi_DataType_t_BLPAPI_DATATYPE_BOOL,
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
    collections::HashMap,
    ffi::{c_int, CStr},
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ptr::{null, null_mut},
};

use crate::{
    core::OsInt,
    name::{Name, NameBuilder},
    schema::{SchemaStatus, UserData},
    Error,
};

/// Enumeration of DataType
#[derive(Debug, Clone, Default)]
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
    #[default]
    Unknown,
}

#[allow(non_upper_case_globals)]
impl From<OsInt> for DataType {
    fn from(arg: OsInt) -> Self {
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
#[derive(Debug, Clone)]
pub struct ConstantList {
    pub(crate) ptr: *const blpapi_ConstantList_t,
    pub name: Name,
    pub user_data: UserData,
    pub description: String,
    pub constants: HashMap<usize, Constant>,
    pub data_type: DataType,
    pub status: SchemaStatus,
}

impl Default for ConstantList {
    fn default() -> Self {
        let ptr = null();
        let name = NameBuilder::default().build();
        let user_data = UserData::default();
        let description = String::default();
        let constants = HashMap::new();
        let data_type = DataType::Unknown;
        let status = SchemaStatus::default();
        Self {
            ptr,
            name,
            user_data,
            description,
            constants,
            data_type,
            status,
        }
    }
}

impl ConstantList {
    // Creating constant list from pointer
    pub fn from_ptr(mut self, ptr: *const blpapi_ConstantList_t) -> Self {
        Self::default();
        self.ptr = ptr;
        if !self.ptr.is_null() {
            self.name = self.name();
            self.user_data = self.user_data();
            self.description = self.description();
            self.constants = self.all_constants().unwrap_or_default();
            self.data_type = self.data_type().unwrap_or_default();
            self.status = self.status();
        }
        self
    }

    // Get all available constants
    fn all_constants(&self) -> Result<HashMap<usize, Constant>, Error> {
        let mut hm_names = HashMap::new();
        let no_const = unsafe { blpapi_ConstantList_numConstants(self.ptr) as usize };

        match no_const > 0 {
            true => {
                for index in 0..no_const {
                    let constant = self.get_constant_at(index)?;
                    hm_names.insert(index, constant);
                }
                Ok(hm_names)
            }
            false => Err(Error::ConstantList),
        }
    }
    // status of the ConstantList
    pub fn status(&self) -> SchemaStatus {
        let res = unsafe { blpapi_ConstantList_status(self.ptr) as u32 };
        SchemaStatus::from(res)
    }

    // Get name of ConstantList
    pub fn name(&self) -> Name {
        let name = NameBuilder::default();
        if !self.ptr.is_null() {
            let name_ptr = unsafe { blpapi_ConstantList_name(self.ptr) };
            name.by_ptr(name_ptr).build()
        } else {
            name.build()
        }
    }

    // Get Datatype
    pub fn data_type(&self) -> Result<DataType, Error> {
        if self.ptr.is_null() {
            return Err(Error::ConstantList);
        }
        let dt = unsafe { blpapi_ConstantList_datatype(self.ptr) } as OsInt;
        let dt_type = DataType::from(dt);
        match dt_type {
            DataType::Unknown => Err(Error::ConstantList),
            _ => Ok(dt_type),
        }
    }

    // Get description of ConstantList
    pub fn description(&self) -> String {
        unsafe {
            let des = blpapi_ConstantList_description(self.ptr);
            CStr::from_ptr(des).to_string_lossy().into_owned()
        }
    }

    // User Data of ConstantList
    pub fn user_data(&self) -> UserData {
        let ptr = unsafe { blpapi_ConstantList_userData(self.ptr) };
        UserData { ptr }
    }

    // New User Data of ConstantList
    pub fn new_user_data(&mut self, user_data: UserData) -> Result<(), Error> {
        match !self.ptr.is_null() {
            true => {
                unsafe { blpapi_ConstantList_setUserData(self.ptr as *mut _, user_data.ptr) };
                Ok(())
            }
            false => Err(Error::ConstantList),
        }
    }

    // Get constant at index
    pub fn get_constant_at(&self, index: usize) -> Result<Constant, Error> {
        if self.ptr.is_null() {
            return Err(Error::ConstantList);
        }
        let const_ptr = unsafe { blpapi_ConstantList_getConstantAt(self.ptr, index) };
        if const_ptr.is_null() {
            return Err(Error::Constant);
        }
        Ok(Constant { ptr: const_ptr })
    }
}

impl Display for ConstantList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let des = self.description();
        write!(f, "ConstantList: {}", des)
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
#[derive(Debug, Clone)]
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
            return Err(Error::Constant);
        }
        let constant: *const blpapi_Constant_t = self.ptr;
        let dt = unsafe { blpapi_Constant_datatype(constant) } as OsInt;
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
