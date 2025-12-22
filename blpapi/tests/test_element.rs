use blpapi::element::Element;
use blpapi_sys::{blpapi_Element_t, blpapi_Name_t};
use std::ptr;

thread_local! {
    static MOCK_REGISTRY: std::cell::RefCell<
        std::collections::HashMap<*mut blpapi_Element_t, MockElementData>
    >=std::cell::RefCell::new(std::collections::HashMap::new());
}
pub struct MockElementData {
    pub name_ptr: *const blpapi_Name_t,
    pub num_values: usize,
    pub i64_values: i64,
    pub f64_values: f64,
    pub bool_values: bool,
}

// Mock function blpapi_Element_name
#[allow(non_snake_case)]
pub extern "C" fn mock_blpapi_Element_name(element: *mut blpapi_Element_t) -> *mut blpapi_Name_t {
    MOCK_REGISTRY.with(|reg| {
        reg.borrow()
            .get(&element)
            .map(|data| data.name_ptr)
            .unwrap_or(std::ptr::null_mut())
    }) as *mut blpapi_Name_t
}

/// Builder struct for testing
pub struct ElementBuilder {
    data: MockElementData,
}

impl ElementBuilder {
    pub fn new() -> Self {
        Self {
            data: MockElementData {
                name_ptr: ptr::null(),
                num_values: 1,
                i64_values: 0,
                bool_values: true,
                f64_values: 0.0,
            },
        }
    }

    pub fn with_i64_value(mut self, value: i64) -> Self {
        self.data.i64_values = value;
        self
    }

    pub fn with_num_values(mut self, value: usize) -> Self {
        self.data.num_values = value;
        self
    }

    pub fn with_bool_values(mut self, value: bool) -> Self {
        self.data.bool_values = value;
        self
    }

    pub fn with_f64_values(mut self, value: f64) -> Self {
        self.data.f64_values = value;
        self
    }

    pub fn build(self) -> Element {
        let sent_ptr = Box::into_raw(Box::new(1)) as *mut blpapi_Element_t;
        MOCK_REGISTRY.with(|reg| {
            reg.borrow_mut().insert(sent_ptr, self.data);
        });
        let mut ele = Element::default();
        ele.ptr = sent_ptr;
        ele
    }
}

#[test]
pub fn test_blpapi_element_name() {
    let element = ElementBuilder::new().build();
    let _name = mock_blpapi_Element_name(element.ptr);
}
