use crate::element::{Element, GetValue};
use std::collections::HashMap;

/// Trait Implementation for bulk Elements
pub type BulkElement = Vec<HashMap<String, String>>;

pub trait RefDataField {
    fn set_from_element(&mut self, element: &Element);
}

/// RefDataField for String Implementation
impl RefDataField for String {
    fn set_from_element(&mut self, element: &Element) {
        if let Some(v) = element.get_at(0) {
            *self = v;
        }
    }
}

/// RefDataField for String Implementation
impl<T: GetValue> RefDataField for Option<T> {
    fn set_from_element(&mut self, element: &Element) {
        if let Some(v) = element.get_at(0) {
            *self = Some(v);
        }
    }
}

/// RefDataField for BulkElement
impl RefDataField for BulkElement {
    fn set_from_element(&mut self, element: &Element) {
        let num_rows = element.num_values();
        let mut new_hm: Vec<HashMap<String, String>> = vec![];

        for i in 0..num_rows {
            let row_element: Option<Element> = element.get_at(i);
            let mut new_hash: HashMap<String, String> = HashMap::with_capacity(num_rows);
            if let Some(row_el) = row_element {
                for sub_field in row_el.elements() {
                    let name = sub_field.name().to_string();
                    let val: Option<String> = sub_field.get_at(0);
                    let val = val.unwrap_or_default();
                    new_hash.insert(name, val);
                }
                new_hm.push(new_hash);
            }
        }
        *self = new_hm;
    }
}

/// RefDataField for String Implementation
impl RefDataField for f64 {
    fn set_from_element(&mut self, element: &Element) {
        if let Some(v) = element.get_at(0) {
            *self = v;
        }
    }
}

/// A trait to convert reference data element fields into a struct
pub trait RefData: Default {
    const FIELDS: &'static [&'static str];
    fn on_field(&mut self, field: &str, element: &Element);
}
