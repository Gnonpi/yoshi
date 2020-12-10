use crate::task_definition::{
    DefinitionArguments, 
    DefinitionArgumentType,
    DefinitionArgumentElement
};

#[test]
fn test_setting_value() {
    let mut da = DefinitionArguments::new();
    let key = String::from("my-key");
    let value = String::from("my-value");
    let da_type = DefinitionArgumentType::AString;

    da.set(&key, value.clone(), da_type);
    assert!(da.map.get(&key).is_some());
    let returned_value = da.get(&key).unwrap();
    assert_eq!(returned_value, (DefinitionArgumentElement::AString(value)));
}

#[test]
fn test_getting_unknown_key_is_none() {
    let da = DefinitionArguments::new();
    let key = String::from("my-key");
    let res = da.get(&key);
    assert!(res.is_none());
}

#[test]
fn test_getting_converts_to_type() {
    let mut da = DefinitionArguments::new();
    let key = String::from("my-key");
    let value = String::from("[\"one\", \"two\", \"three\"]");
    let expected = vec![
        String::from("one"),
        String::from("two"),
        String::from("three")
    ];
    let da_type = DefinitionArgumentType::VecString;

    da.set(&key, value.clone(), da_type);
    let res = da.get(&key);
    assert!(res.is_some());
    let res_value = res.unwrap();
    assert_eq!(res_value, DefinitionArgumentElement::VecString(expected));
}
