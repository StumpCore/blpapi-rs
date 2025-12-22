use std::{
    ffi::{c_void, CStr, CString},
    fmt::Display,
    io::Write,
    ptr,
};

use blpapi_sys::{
    blpapi_SchemaElementDefinition_description, blpapi_SchemaElementDefinition_getAlternateName,
    blpapi_SchemaElementDefinition_maxValues, blpapi_SchemaElementDefinition_minValues,
    blpapi_SchemaElementDefinition_name, blpapi_SchemaElementDefinition_numAlternateNames,
    blpapi_SchemaElementDefinition_setUserData, blpapi_SchemaElementDefinition_status,
    blpapi_SchemaElementDefinition_t, blpapi_SchemaElementDefinition_type,
    blpapi_SchemaElementDefinition_userData, blpapi_SchemaTypeDefinition_datatype,
    blpapi_SchemaTypeDefinition_description, blpapi_SchemaTypeDefinition_enumeration,
    blpapi_SchemaTypeDefinition_getElementDefinition,
    blpapi_SchemaTypeDefinition_getElementDefinitionAt, blpapi_SchemaTypeDefinition_isComplex,
    blpapi_SchemaTypeDefinition_isComplexType, blpapi_SchemaTypeDefinition_isEnumeration,
    blpapi_SchemaTypeDefinition_isEnumerationType, blpapi_SchemaTypeDefinition_isSimple,
    blpapi_SchemaTypeDefinition_isSimpleType, blpapi_SchemaTypeDefinition_name,
    blpapi_SchemaTypeDefinition_numElementDefinitions, blpapi_SchemaTypeDefinition_print,
    blpapi_SchemaTypeDefinition_setUserData, blpapi_SchemaTypeDefinition_status,
    blpapi_SchemaTypeDefinition_t, blpapi_SchemaTypeDefinition_userData, BLPAPI_STATUS_ACTIVE,
    BLPAPI_STATUS_DEPRECATED, BLPAPI_STATUS_INACTIVE, BLPAPI_STATUS_PENDING_DEPRECATION,
};

use crate::{
    constant::{ConstantList, DataType},
    core::{write_to_stream_cb, StreamWriterContext},
    name::{Name, NameBuilder},
    Error,
};

/// UserData Struct
pub struct UserData {
    pub ptr: *mut c_void,
}

/// Schema Status
#[derive(Clone, Debug)]
pub enum SchemaStatus {
    Active,
    Deprecated,
    Inactive,
    PendingDeprecation,
    Unkown,
}

impl From<u32> for SchemaStatus {
    fn from(status: u32) -> Self {
        match status {
            BLPAPI_STATUS_ACTIVE => SchemaStatus::Active,
            BLPAPI_STATUS_INACTIVE => SchemaStatus::Inactive,
            BLPAPI_STATUS_DEPRECATED => SchemaStatus::Deprecated,
            BLPAPI_STATUS_PENDING_DEPRECATION => SchemaStatus::PendingDeprecation,
            _ => SchemaStatus::Unkown,
        }
    }
}

/// Schema Elements Definition
#[derive(Debug, Clone)]
pub struct SchemaElements {
    pub(crate) ptr: *mut blpapi_SchemaElementDefinition_t,
    pub status: SchemaStatus,
}

impl Default for SchemaElements {
    fn default() -> Self {
        let ptr: *mut blpapi_SchemaElementDefinition_t = std::ptr::null_mut();
        let status = SchemaStatus::Inactive;
        Self { ptr, status }
    }
}

impl SchemaElements {
    pub fn schema_element_definition_name(self) -> Result<Name, Error> {
        if self.ptr.is_null() {
            return Err(Error::Schema);
        };
        let ptr = unsafe {
            blpapi_SchemaElementDefinition_name(self.ptr as *const blpapi_SchemaElementDefinition_t)
        };
        let name = NameBuilder::default().by_ptr(ptr).build();
        Ok(name)
    }

    /// Get the Depreciation Status
    pub fn status(&mut self) -> Result<SchemaStatus, Error> {
        if self.ptr.is_null() {
            return Err(Error::Schema);
        };
        let status = unsafe { blpapi_SchemaElementDefinition_status(self.ptr) as u32 };
        let st_schema = SchemaStatus::from(status);
        self.status = st_schema.clone();
        Ok(st_schema)
    }

    /// Get the Schema Type Definition
    pub fn type_definition(&self) -> Result<SchemaType, Error> {
        if self.ptr.is_null() {
            return Err(Error::Schema);
        };
        let ptr = unsafe { blpapi_SchemaElementDefinition_type(self.ptr) };
        let status = SchemaStatus::Inactive;
        let mut schema = SchemaType { ptr, status };
        schema.status();
        Ok(schema)
    }

    /// Get the number of Alternate Names
    pub fn num_alternative_names(&self) -> Result<usize, Error> {
        if self.ptr.is_null() {
            return Err(Error::Schema);
        };
        Ok(unsafe { blpapi_SchemaElementDefinition_numAlternateNames(self.ptr) })
    }

    /// Get the Alternate Name of specific index
    pub fn alternative_name_at_index(&self, index: usize) -> Result<Name, Error> {
        if self.ptr.is_null() {
            return Err(Error::Schema);
        };
        let ptr = unsafe { blpapi_SchemaElementDefinition_getAlternateName(self.ptr, index) };
        let name = NameBuilder::default().by_ptr(ptr).build();
        Ok(name)
    }

    /// Get Minimum
    pub fn min(&self) -> Result<usize, Error> {
        if self.ptr.is_null() {
            return Err(Error::Schema);
        };
        Ok(unsafe { blpapi_SchemaElementDefinition_minValues(self.ptr) })
    }

    /// Get Maximum
    pub fn max(&self) -> Result<usize, Error> {
        if self.ptr.is_null() {
            return Err(Error::Schema);
        };
        Ok(unsafe { blpapi_SchemaElementDefinition_maxValues(self.ptr) })
    }

    /// Get User Data
    pub fn user_data(&self) -> Result<UserData, Error> {
        if self.ptr.is_null() {
            return Err(Error::Schema);
        };
        let ptr = unsafe { blpapi_SchemaElementDefinition_userData(self.ptr) };
        Ok(UserData { ptr })
    }

    /// Set new User Data
    pub fn set_user_data(&self, user_data: UserData) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(Error::Schema);
        };
        unsafe { blpapi_SchemaElementDefinition_setUserData(self.ptr, user_data.ptr) };
        Ok(())
    }
}

/// Impleenting Display Trait for Schema Elements
impl Display for SchemaElements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let des = unsafe {
            let new_des = blpapi_SchemaElementDefinition_description(self.ptr);
            CStr::from_ptr(new_des)
        };
        let des = des.to_str().expect("Invalid UTF-8");
        write!(f, "SchemaElements: {}", des)
    }
}

/// Schema Type Definition
pub struct SchemaType {
    pub(crate) ptr: *mut blpapi_SchemaTypeDefinition_t,
    pub status: SchemaStatus,
}

impl SchemaType {
    /// Get name of the Schema Type
    pub fn name(&self) -> Name {
        let ptr = unsafe { blpapi_SchemaTypeDefinition_name(self.ptr) };
        let name = NameBuilder::default().by_ptr(ptr).build();
        name
    }

    /// Get the Depreciation Status
    pub fn status(&mut self) -> SchemaStatus {
        let status = unsafe { blpapi_SchemaTypeDefinition_status(self.ptr) } as u32;
        let st_schema = SchemaStatus::from(status);
        self.status = st_schema.clone();
        st_schema
    }

    /// Geth the data type of the Schema Type
    pub fn data_type(&self) -> DataType {
        let ptr = unsafe { blpapi_SchemaTypeDefinition_datatype(self.ptr) };
        let dt_type = DataType::from(ptr);
        dt_type
    }

    /// Check if is complex
    pub fn is_complex(&self) -> bool {
        let ptr = unsafe { blpapi_SchemaTypeDefinition_isComplex(self.ptr) } as i64;
        match ptr == 0 {
            true => false,
            false => true,
        }
    }

    /// Check if is complex type
    pub fn is_complex_type(&self) -> bool {
        let ptr = unsafe { blpapi_SchemaTypeDefinition_isComplexType(self.ptr) } as i64;
        match ptr == 0 {
            true => false,
            false => true,
        }
    }

    /// Check if is simple
    pub fn is_simple(&self) -> bool {
        let ptr = unsafe { blpapi_SchemaTypeDefinition_isSimple(self.ptr) } as i64;
        match ptr == 0 {
            true => false,
            false => true,
        }
    }

    /// Check if is simple type
    pub fn is_simple_type(&self) -> bool {
        let ptr = unsafe { blpapi_SchemaTypeDefinition_isSimpleType(self.ptr) } as i64;
        match ptr == 0 {
            true => false,
            false => true,
        }
    }

    /// Check if is enumeration  
    pub fn is_enum(&self) -> bool {
        let ptr = unsafe { blpapi_SchemaTypeDefinition_isEnumeration(self.ptr) } as i64;
        match ptr == 0 {
            true => false,
            false => true,
        }
    }

    /// Check if is enumeration type
    pub fn is_enum_type(&self) -> bool {
        let ptr = unsafe { blpapi_SchemaTypeDefinition_isEnumerationType(self.ptr) } as i64;
        match ptr == 0 {
            true => false,
            false => true,
        }
    }

    /// Get the number of Element Definitions
    pub fn num_eleent_definitions(&self) -> usize {
        unsafe { blpapi_SchemaTypeDefinition_numElementDefinitions(self.ptr) }
    }

    /// Get the Element Definition
    pub fn element_def_str(&self, name: &str) -> Result<SchemaElements, Error> {
        let status = SchemaStatus::Inactive;
        let name_ptr = ptr::null_mut();
        let name = CString::new(name).unwrap_or(CString::default());
        let ptr = unsafe {
            blpapi_SchemaTypeDefinition_getElementDefinition(self.ptr, name.as_ptr(), name_ptr)
        };
        let mut s_ele = SchemaElements { ptr, status };
        s_ele.status()?;
        Ok(s_ele)
    }

    /// Get the Element Definition by name
    pub fn element_def_name(&self, name: &Name) -> Result<SchemaElements, Error> {
        let status = SchemaStatus::Inactive;
        let name_ptr = ptr::null_mut();
        let ptr = unsafe {
            blpapi_SchemaTypeDefinition_getElementDefinition(self.ptr, name_ptr, name.ptr)
        };
        let mut s_ele = SchemaElements { ptr, status };
        s_ele.status()?;
        Ok(s_ele)
    }

    /// Get SchemaType Definition at index
    pub fn element_def_at(&self, index: usize) -> Result<SchemaElements, Error> {
        let status = SchemaStatus::Inactive;
        let ptr = unsafe { blpapi_SchemaTypeDefinition_getElementDefinitionAt(self.ptr, index) };
        let mut s_ele = SchemaElements { ptr, status };
        s_ele.status()?;
        Ok(s_ele)
    }

    /// Print the values
    pub fn print<T: Write>(&self, writer: &mut T, indent: i32, spaces: i32) -> Result<(), Error> {
        let mut context = StreamWriterContext { writer };
        unsafe {
            let res = blpapi_SchemaTypeDefinition_print(
                self.ptr,
                Some(write_to_stream_cb),
                &mut context as *mut _ as *mut c_void,
                indent as std::ffi::c_int,
                spaces as std::ffi::c_int,
            );
            if res != 0 {
                return Err(Error::struct_error(
                    "SchemaType",
                    "print",
                    "Error when trying to write to stream writer",
                ));
            };
        };
        Ok(())
    }

    /// Get the user data associated with this Schema Element
    pub fn user_type(&self) -> Result<UserData, Error> {
        let ptr = unsafe { blpapi_SchemaTypeDefinition_userData(self.ptr) };
        Ok(UserData { ptr })
    }

    /// Set the user data from another UserData
    pub fn set_user_type(&self, user_data: UserData) -> Result<(), Error> {
        unsafe { blpapi_SchemaTypeDefinition_setUserData(self.ptr, user_data.ptr) };
        Ok(())
    }

    /// Get Enumeration of Schema Type Definition
    pub fn enumeration(&self) -> ConstantList {
        let ptr = unsafe { blpapi_SchemaTypeDefinition_enumeration(self.ptr) };
        ConstantList { ptr }
    }
}

impl Display for SchemaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let des = unsafe {
            let des = blpapi_SchemaTypeDefinition_description(self.ptr);
            CStr::from_ptr(des)
        };
        let des = des.to_str().expect("Invalid UTF-8");
        write!(f, "SchemaTypeDefinition: {}", des)
    }
}

impl Default for SchemaType {
    fn default() -> Self {
        let ptr: *mut blpapi_SchemaTypeDefinition_t = std::ptr::null_mut();
        let status = SchemaStatus::Inactive;
        Self { ptr, status }
    }
}
