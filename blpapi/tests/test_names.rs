use blpapi::{
    name::{Name, NameBuilder},
    names::{SESSION_STARTED, SLOW_CONSUMER_WARNING},
};

#[test]
pub fn test_names() {
    let slow_consumer_warning = SESSION_STARTED;
    println!("{:#?}", slow_consumer_warning);
}
