use blpapi::name::{Name, NameBuilder};

#[test]
pub fn test_name_builder_default() {
    let name = NameBuilder::default();
    let name = name.build();
    let name = name.to_string();
    println!("{}", name);
}

#[test]
pub fn test_name_builder_set_name() {
    let name = NameBuilder::default();
    let name = name.set_name("JohnsCon");
    let name = name.build();
    let name = name.to_string();
    println!("{}", name);
}

#[test]
pub fn test_name_builder_compare_names_strings() {
    let n_one = String::from("JohnsCon");
    let n_two = String::from("JohnsCon");
    let name = NameBuilder::default();
    let name = name.set_name(n_one);
    let name = name.build();

    assert_eq!(name, n_two);
}

#[test]
pub fn test_name_builder_compare_names() {
    let n_one = String::from("JohnsCon");
    let n_two = String::from("JohnsCon");

    // Creating the first name
    let name = NameBuilder::default();
    let name = name.set_name(n_one);
    let name = name.build();

    // Creating the second name
    let name_t = NameBuilder::default().set_name(n_two).build();

    assert_eq!(name, name_t);
}
#[test]
#[should_panic]
pub fn test_name_builder_compare_names_panic() {
    let n_one = String::from("JohnsCon");
    let n_two = String::from("NotJohnsCon");
    let name = NameBuilder::default();
    let name = name.set_name(n_one);
    let name = name.build();
    assert_eq!(name, n_two);
}

#[test]
pub fn test_name_display() {
    let name = NameBuilder::default().set_name("JohnsCon").build();
    println!("{}", name);
}

#[test]
pub fn test_name_to_string() {
    let name = NameBuilder::default().set_name("JohnsCon").build();
    let name_string = name.to_string();
    println!("{}", name_string);
    assert_eq!(name_string, "JohnsCon");
}

#[test]
pub fn test_name_find_name() {
    let another_name = "JohnsConNot";
    let name = NameBuilder::default().set_name("JohnsCon").build();
    let res = name.find_name(another_name);
    println!("{}", res);
}

#[test]
pub fn test_name_has_name() {
    let another_name = "JohnsCon";
    let _name = NameBuilder::default().set_name("JohnsCon").build();
    let res = Name::has_name(another_name);
    assert!(res);
    let another_name_false = "JohnsConFalse";
    let res = Name::has_name(another_name_false);
    assert!(!res);
}
