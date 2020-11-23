use crate::dag_parsing::dag_config_parser::{SupportedFormat, DagConfigParser};
use crate::dag_parsing::yaml_parser::YamlDagConfigParser;
use crate::dag_parsing::DagParsingError;
use std::fs;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}


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
fn it_can_parse_example_1() {
    let example_path = "src/dag_parsing/examples/example1.yaml";
    let content = fs::read_to_string(example_path).expect("failed to read example file");
    let ycp = YamlDagConfigParser {};
    let dag_config = ycp.parse_file(content).unwrap();

    assert_eq!(dag_config.nodes.len(), 4);
    assert_eq!(dag_config.definitions.len(), 3);
    assert_eq!(dag_config.runners.len(), 1);
    assert_eq!(dag_config.dag_edges.len(), 3);
}

#[test]
fn it_can_parse_example_2() {
    init_logger();
    let example_path = "src/dag_parsing/examples/example2.yaml";
    let content = fs::read_to_string(example_path).expect("failed to read example file");
    let ycp = YamlDagConfigParser {};
    let dag_config = ycp.parse_file(content).unwrap();

    // println!("{:#?}", dag_config);
    
    assert_eq!(dag_config.nodes.len(), 4);
    // 1 def & 1 runner per node
    assert_eq!(dag_config.definitions.len(), 4);
    assert_eq!(dag_config.runners.len(), 4);
    assert_eq!(dag_config.dag_edges.len(), 3);
}
