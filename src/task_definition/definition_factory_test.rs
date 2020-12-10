use crate::task_definition::{
    TaskDefinitionType,
    DefinitionArguments,
    DefinitionArgumentType,
    create_new_definition
};
use std::collections::HashMap;

#[test]
fn it_can_create_bash_def() {
    let mut da = DefinitionArguments::new();
    let command = "[\"echo\", \"'1'\"]".to_string();
    da.set(
        &"command".to_string(), 
        command.clone(), 
        DefinitionArgumentType::VecString
    );
    
    let b_def = create_new_definition(&TaskDefinitionType::Bash, da);
    assert_eq!(b_def.task_type(), TaskDefinitionType::Bash);
    
    let mut expected_params = HashMap::new();
    // in BashTaskDefinition.get_params, we concatenate in one
    expected_params.insert("command".to_string(), "echo \'1\'".to_string());
    assert_eq!(b_def.get_params(), expected_params);
}

#[test]
fn it_can_create_python_def() {
    let mut da = DefinitionArguments::new();
    let script_path = String::from("/tmp/script.py");
    let args = String::from("[\"one\"]");
    da.set(
        &"script_path".to_string(), 
        script_path.clone(), 
        DefinitionArgumentType::Filepath
    );
    da.set(
        &"args".to_string(),
        args.clone(), 
        DefinitionArgumentType::VecString
    );
    
    let p_def = create_new_definition(&TaskDefinitionType::Python, da);
    assert_eq!(p_def.task_type(), TaskDefinitionType::Python);
    
    let mut expected_params = HashMap::new();
    expected_params.insert("script_path".to_string(), script_path.to_string());
    // in get_params, we remove the doublequotes
    expected_params.insert("args".to_string(), "[one]".to_string());
    assert_eq!(p_def.get_params(), expected_params);
}

#[test]
fn it_can_create_dummy_def() {
    let da = DefinitionArguments::new();
    let d_def = create_new_definition(&TaskDefinitionType::Dummy, da);
    assert_eq!(d_def.task_type(), TaskDefinitionType::Dummy);
}
