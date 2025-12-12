use blpapi::{
    schema::{SchemaElements, SchemaType},
    Error,
};

#[test]
pub fn test_schema_ele() {
    let _ele = SchemaElements::default();
}

#[test]
pub fn test_schema_ele_name() -> Result<(), Error> {
    let ele = SchemaElements::default();
    let _res = ele.schema_element_definition_name();
    Ok(())
}

#[test]
pub fn test_schema_type() {
    let _schema_type = SchemaType::default();
}
