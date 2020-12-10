use crate::type_definition::FilePath;
use std::collections::HashMap;


// todo: move to new module
// todo: review visibility
#[derive(Debug, Clone, Copy)]
pub enum DefinitionArgumentType {
    AString,  // A-string to differentiate from type
    Filepath,
    Integer,
    Float,
    VecString
}

pub enum DefinitionArgumentElement {
    AString(String),
    Filepath(FilePath),
    Integer(i64),
    Float(f64),
    /// Vec of string as JSON array
    VecString(Vec<String>)
}

pub struct DefinitionArguments {
    map: HashMap<String, (String, DefinitionArgumentType)>
}

// todo: temporary, use a library or something more efficient and fail-proff
fn string_to_vec_of_string(mut s: String) -> Vec<String> {
    let mut res = Vec::new();
    s.remove(0);
    s.remove(s.len() - 1);
    if s.is_empty() {
        return res
    }
    // basic iterative algo
    let mut opened_quotes = false;
    let mut buffer = String::new();
    for c in s.chars() {
        if c == '\"' {
            if opened_quotes {
                // closing expression
                res.push(buffer);
                opened_quotes = false;
                buffer = String::new();
            } else {
                // opening expression
                opened_quotes = true;
            }
        } else {
            if opened_quotes {
                // add to current
                buffer.push(c);
            }
        }
    }
    res
}

impl DefinitionArguments {
    fn new() -> Self {
        DefinitionArguments {
            map: HashMap::new()
        }
    }

    fn set(&mut self, key: String, value: String, da_type: DefinitionArgumentType) {
        self.map.insert(key, (value, da_type));
    }

    pub fn get(&self, key: &String) -> Option<DefinitionArgumentElement> {
        match self.map.get(key) {
            Some((v, t)) => {
                let value = v.to_string();
                match t {
                    DefinitionArgumentType::AString => {
                        Some(DefinitionArgumentElement::AString(value))
                    },
                    DefinitionArgumentType::Filepath => {
                        let fp = FilePath::from(value);
                        Some(DefinitionArgumentElement::Filepath(fp))
                    },
                    DefinitionArgumentType::Integer => {
                        let as_int = value.parse::<i64>().unwrap();
                        Some(DefinitionArgumentElement::Integer(as_int))
                    },
                    DefinitionArgumentType::Float => {
                        let as_float = value.parse::<f64>().unwrap();
                        Some(DefinitionArgumentElement::Float(as_float))
                    },
                    DefinitionArgumentType::VecString => {
                        // from JSON format
                        // todo: use a library (serde?)
                        let result = string_to_vec_of_string(value);               
                        Some(DefinitionArgumentElement::VecString(result))
                    },
                }
            },
            None => None
        }
    }
}


#[cfg(test)]
#[path = "./definition_arguments_test.rs"]
mod definition_arguments_test;
