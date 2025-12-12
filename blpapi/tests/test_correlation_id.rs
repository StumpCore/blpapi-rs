use blpapi::core::{
    BLPAPI_DEFAULT_CORRELATION_CLASS_ID, BLPAPI_DEFAULT_CORRELATION_ID,
    BLPAPI_DEFAULT_CORRELATION_INT_VALUE,
};
use blpapi::correlation_id::{CorrelationIdBuilder, ValueType};
use std::ffi::c_void;

// A simple struct to use for pointer and smart pointer tests.
#[derive(Debug, PartialEq, Eq)]
struct MyRequestData {
    id: u32,
    message: String,
}

#[test]
fn test_correlation_id_builder() {
    let builder = CorrelationIdBuilder::new();
    assert_eq!(builder.class_id, None);
    assert_eq!(builder.reserved, None);
}

#[test]
fn test_correlation_id_builder_settings() {
    let int_val: u64 = 1;
    let builder = CorrelationIdBuilder::new();
    let builder = builder.set_value_type(ValueType::IntValue(int_val));
    let builder = builder.set_reserved(int_val);
    let builder = builder.set_class_id(35);

    assert_eq!(builder.reserved.unwrap(), 1);
    assert_eq!(builder.class_id.unwrap(), 35);
}
#[test]
fn test_correlation_id_builder_default() {
    let builder = CorrelationIdBuilder::default();

    assert_eq!(
        builder.reserved.unwrap(),
        BLPAPI_DEFAULT_CORRELATION_INT_VALUE
    );
    assert_eq!(
        builder.class_id.unwrap(),
        BLPAPI_DEFAULT_CORRELATION_CLASS_ID
    );
}

#[test]
fn test_correlation_id_builder_default_build() {
    let builder = CorrelationIdBuilder::default();
    let cor_id = builder.build();
    assert_eq!(cor_id.value, BLPAPI_DEFAULT_CORRELATION_ID);
    assert_eq!(cor_id.class_id, BLPAPI_DEFAULT_CORRELATION_CLASS_ID);
    assert_eq!(cor_id.reserved, BLPAPI_DEFAULT_CORRELATION_INT_VALUE);
}

#[test]
fn test_correlation_id_builder_new_int_id() {
    let value: u64 = 12_354_789;
    let builder = CorrelationIdBuilder::default();
    let builder = builder.set_value_type(ValueType::IntValue(value));
    let cor_id = builder.build();
    assert_eq!(cor_id.value, value);
    assert_eq!(cor_id.class_id, BLPAPI_DEFAULT_CORRELATION_CLASS_ID);
    assert_eq!(cor_id.reserved, BLPAPI_DEFAULT_CORRELATION_INT_VALUE);
}

#[test]
fn test_correlation_id_builder_new_pointer() {
    let data = MyRequestData {
        id: 101,
        message: "Raw pointer test".to_string(),
    };
    let ptr = &data as *const MyRequestData as *mut c_void;

    let builder = CorrelationIdBuilder::default();
    let builder = builder.set_value_type(ValueType::PointerValue(ptr));
    let _cor_id = builder.build();

    let _value = ptr as u64;
}

#[test]
fn test_correlation_id_builder_new_smart_pointer() {
    let original_data = Box::new(MyRequestData {
        id: 202,
        message: "Smart pointer test".to_string(),
    });
    let original_data_ptr = &*original_data as *const MyRequestData;
    let value_type_res = ValueType::SmartPointerValue(original_data);

    let builder = CorrelationIdBuilder::default();
    let builder = builder.set_value_type(value_type_res);
    let _cor_id = builder.build();

    let _value = original_data_ptr as u64;
}

#[test]
fn test_correlation_id_get_class_id() {
    let value: u64 = 12_354_789;
    let builder = CorrelationIdBuilder::default();
    let builder = builder.set_value_type(ValueType::IntValue(value));
    let cor_id = builder.build();

    assert_eq!(cor_id.class_id(), 0);
}
