use blpapi_sys::{blpapi_SchemaElementDefinition_name, blpapi_SchemaElementDefinition_t, blpapi_SchemaTypeDefinition_t};

/// Schema Status
pub enum SchemaStatus {
    Active,
    Deprecated,
    Inactive,
    PendingDeprecation,
}
/// Schema Elements Definition
pub struct SchemaElements {
    pub(crate) ptr: *mut blpapi_SchemaElementDefinition_t,
}

impl Default for SchemaElements {
    fn default() -> Self {
        let ptr: *mut blpapi_SchemaElementDefinition_t = std::ptr::null_mut();
        Self {
            ptr
        }
    }
}

impl SchemaElements {
    pub fn schema_element_definition_name(self) {
        let res = unsafe {
            let mut ptr = self.ptr;
            blpapi_SchemaElementDefinition_name(
                ptr as *const blpapi_SchemaElementDefinition_t,
            )
        };
        println!("res: {:?}", res);
    }
}

/// Schema Type Definition
pub struct SchemaType {
    pub(crate) ptr: *mut blpapi_SchemaTypeDefinition_t,
}

impl Default for SchemaType {
    fn default() -> Self {
        let ptr: *mut blpapi_SchemaTypeDefinition_t = std::ptr::null_mut();
        Self {
            ptr
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::{SchemaElements, SchemaType};

    #[test]
    pub fn test_schema_ele() {
        let ele = SchemaElements::default();
    }

    #[test]
    pub fn test_schema_ele_name() {
        let ele = SchemaElements::default();
        ele.schema_element_definition_name();
    }

    #[test]
    pub fn test_schema_type() {
        let schema_type = SchemaType::default();
    }
}
