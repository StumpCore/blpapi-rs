use std::{
    collections::HashMap,
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
    blpapi_SchemaTypeDefinition_getElementDefinitionAt,
    blpapi_SchemaTypeDefinition_isComplexType,
    blpapi_SchemaTypeDefinition_isEnumerationType,
    blpapi_SchemaTypeDefinition_isSimpleType, blpapi_SchemaTypeDefinition_name,
    blpapi_SchemaTypeDefinition_numElementDefinitions, blpapi_SchemaTypeDefinition_print,
    blpapi_SchemaTypeDefinition_setUserData, blpapi_SchemaTypeDefinition_status,
    blpapi_SchemaTypeDefinition_t, blpapi_SchemaTypeDefinition_userData, BLPAPI_STATUS_ACTIVE,
    BLPAPI_STATUS_DEPRECATED, BLPAPI_STATUS_INACTIVE, BLPAPI_STATUS_PENDING_DEPRECATION,
};

use crate::{
    constant::{ConstantList, DataType},
    core::{write_to_stream_cb, OsInt, StreamWriterContext},
    name::{Name, NameBuilder},
    Error,
};

/// UserData Struct
#[derive(Debug, Default, Clone)]
pub struct UserData {
    pub ptr: *mut c_void,
}

/// Schema Status
#[derive(Clone, Debug, Default)]
pub enum SchemaStatus {
    Active,
    Deprecated,
    Inactive,
    PendingDeprecation,
    #[default]
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
    pub name: Name,
    pub alternate_names: HashMap<usize, Name>,
    pub schema_type: SchemaType,
    pub user_data: UserData,
}

impl Default for SchemaElements {
    fn default() -> Self {
        let ptr: *mut blpapi_SchemaElementDefinition_t = std::ptr::null_mut();
        let status = SchemaStatus::Inactive;
        let name = NameBuilder::default().build();
        let schema_type = SchemaType::default();
        let alternate_names = HashMap::new();
        let user_data = UserData::default();
        Self {
            ptr,
            status,
            name,
            alternate_names,
            schema_type,
            user_data,
        }
    }
}

impl SchemaElements {
    pub fn from_ptr(mut self, ptr: *mut blpapi_SchemaElementDefinition_t) -> Self {
        Self::default();
        self.ptr = ptr;
        self.name = self.schema_element_definition_name().unwrap_or_default();
        self.status = self.status().unwrap_or_default();
        self.schema_type = self.type_definition().unwrap_or_default();
        self.alternate_names = self.all_alternative_names().unwrap_or_default();
        self.user_data = self.user_data().unwrap_or_default();
        self
    }

    pub fn schema_element_definition_name(&self) -> Result<Name, Error> {
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
        let mut schema = SchemaType::default().from_ptr(ptr);
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

    /// Get HashMap of all alternate Names
    fn all_alternative_names(&self) -> Result<HashMap<usize, Name>, Error> {
        let mut hm_names = HashMap::new();
        let no_alt_names = self.num_alternative_names();

        match no_alt_names {
            Ok(no) => {
                for index in 0..no {
                    let name = self.alternative_name_at_index(index)?;
                    hm_names.insert(index, name);
                }
                Ok(hm_names)
            }
            Err(_) => Err(Error::SchemaType),
        }
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
#[derive(Debug, Clone)]
pub struct SchemaType {
    pub(crate) ptr: *mut blpapi_SchemaTypeDefinition_t,
    pub name: Name,
    pub user_data: UserData,
    pub status: SchemaStatus,
    pub data_type: DataType,
    pub is_complex: bool,
    pub is_complex_type: bool,
    pub is_simple: bool,
    pub is_simple_type: bool,
    pub is_enum: bool,
    pub is_enum_type: bool,
    pub constant_list: ConstantList,
}

impl Default for SchemaType {
    fn default() -> Self {
        let ptr: *mut blpapi_SchemaTypeDefinition_t = std::ptr::null_mut();
        let status = SchemaStatus::Inactive;
        let name = NameBuilder::default().build();
        let user_data = UserData::default();
        let data_type = DataType::Unknown;
        let is_complex = false;
        let is_complex_type = false;
        let is_simple = false;
        let is_simple_type = false;
        let is_enum = false;
        let is_enum_type = false;
        let constant_list = ConstantList::default();
        Self {
            ptr,
            status,
            name,
            user_data,
            data_type,
            is_complex,
            is_complex_type,
            is_simple,
            is_simple_type,
            is_enum,
            is_enum_type,
            constant_list,
        }
    }
}

impl SchemaType {
    pub fn from_ptr(mut self, ptr: *mut blpapi_SchemaTypeDefinition_t) -> Self {
        Self::default();
        self.ptr = ptr;
        self.name = self.name();
        self.user_data = self.user_type().unwrap_or_default();
        self.status = self.status();
        self.data_type = self.data_type();
        self.is_complex = self.is_complex();
        self.is_complex_type = self.is_complex_type();
        self.is_simple = self.is_simple();
        self.is_simple_type = self.is_simple_type();
        self.is_enum = self.is_enum();
        self.is_enum_type = self.is_enum_type();
        self.constant_list = self.enumeration();
        self
    }

    /// Get name of the Schema Type
    pub fn name(&self) -> Name {
        let ptr = unsafe { blpapi_SchemaTypeDefinition_name(self.ptr) };
        NameBuilder::default().by_ptr(ptr).build()
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
        let ptr = unsafe { blpapi_SchemaTypeDefinition_datatype(self.ptr) } as OsInt;
        DataType::from(ptr)
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
    #[cfg(target_os = "windows")]
    pub fn is_simple(&self) -> bool {
        let ptr = unsafe { blpapi_SchemaTypeDefinition_isSimple(self.ptr) } as i64;
        match ptr == 0 {
            true => false,
            false => true,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn is_simple(&self) -> bool {
        true
    }

    /// Check if is complex
    #[cfg(target_os = "windows")]
    pub fn is_complex(&self) -> bool {
        let ptr = unsafe { blpapi_SchemaTypeDefinition_isComplex(self.ptr) } as i64;
        match ptr == 0 {
            true => false,
            false => true,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn is_complex(&self) -> bool {
        true
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
    #[cfg(target_os = "windows")]
    pub fn is_enum(&self) -> bool {
        let ptr = unsafe { blpapi_SchemaTypeDefinition_isEnumeration(self.ptr) } as i64;
        match ptr == 0 {
            true => false,
            false => true,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn is_enum(&self) -> bool {
        true
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
        let name_ptr = ptr::null_mut();
        let name = CString::new(name).unwrap_or_default();
        let ptr = unsafe {
            blpapi_SchemaTypeDefinition_getElementDefinition(self.ptr, name.as_ptr(), name_ptr)
        };
        let mut s_ele = SchemaElements::default().from_ptr(ptr);
        s_ele.status()?;
        Ok(s_ele)
    }

    /// Get the Element Definition by name
    pub fn element_def_name(&self, name: &Name) -> Result<SchemaElements, Error> {
        let name_ptr = ptr::null_mut();
        let ptr = unsafe {
            blpapi_SchemaTypeDefinition_getElementDefinition(self.ptr, name_ptr, name.ptr)
        };
        let mut s_ele = SchemaElements::default().from_ptr(ptr);
        s_ele.status()?;
        Ok(s_ele)
    }

    /// Get SchemaType Definition at index
    pub fn element_def_at(&self, index: usize) -> Result<SchemaElements, Error> {
        let ptr = unsafe { blpapi_SchemaTypeDefinition_getElementDefinitionAt(self.ptr, index) };
        let mut s_ele = SchemaElements::default().from_ptr(ptr);
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
        ConstantList::default().from_ptr(ptr)
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
