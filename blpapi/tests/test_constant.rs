use blpapi::{constant::ConstantList, Error};
#[test]
fn test_constant_list_default_is_null() -> Result<(), Error> {
    let _list = ConstantList::default();
    Ok(())
}
