use crate::name::{Name, NameBuilder};

/// Struct for Override
#[derive(Debug)]
pub struct Override {
    pub field_id: Name,
    pub value: String,
}

impl Override {
    pub fn new(id: impl Into<String>, val: impl Into<String>) -> Self {
        let field_id = id.into();
        let field_id = NameBuilder::default().name(field_id).build();
        let value = val.into();
        Self { field_id, value }
    }
}
