use crate::dag_parsing::dag_config_parser::{SupportedFormat, DagConfigParser};
use crate::dag_parsing::yaml_parser::YamlDagConfigParser;
use crate::dag_parsing::DagParsingError;
use std::fs;


#[test]
fn it_can_return_format() {
    let ycp = YamlDagConfigParser {};
    let format = ycp.get_format();
    assert_eq!(format, SupportedFormat::Yaml);
}

#[test]
fn it_detect_invalid_yaml() {
    let invalid_yaml = "
foo:
  - list1
  - list2
error
bar:
  - 1
  - 2.0
";
    let ycp = YamlDagConfigParser {};
    assert!(! ycp.validate(&invalid_yaml.to_owned()));
}

#[test]
fn it_detect_invalid_schema() {
    let invalid_schema = "
foo: 
  - list1
  - list2
bar:
  - 1
  - 2.0
";
    let ycp = YamlDagConfigParser {};
    assert!(! ycp.validate(&invalid_schema.to_owned()));
}

// todo: parametrized tests?
#[test]
fn it_validate_valid_content() {
    let vec_example_path = vec![
      "src/dag_parsing/examples/example1.yaml",
      "src/dag_parsing/examples/example2.yaml"
    ];
    let ycp = YamlDagConfigParser {};
    for example_path in vec_example_path.iter() {
      let content = fs::read_to_string(example_path)
          .expect("failed to read example file");
      assert!(ycp.validate(&content));
    }
    assert!(false);
}

#[test]
fn it_can_parse_file() {
    assert!(false);
}
