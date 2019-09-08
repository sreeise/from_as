#[macro_use]
extern crate serde_derive;

use std::env;
use from_to::*;

#[derive(Debug, Serialize, Deserialize, FromToFile)]
struct Field {
    name: String,
}

impl Field {
    pub fn new(n: &str) -> Field {
        Field {
            name: String::from(n),
        }
    }
}

fn clean_up(file: &str) {
    match env::var("TRAVIS") {
        Ok(value) => {
            if value.ne("TRUE") {
                std::fs::remove_file(file).unwrap();
            }
        },
        Err(_) => {
            std::fs::remove_file(file).unwrap();
        }
    }
}

#[test]
pub fn test_derive_json() {
    let file = "./tests/field.json";
    let field = Field::new("field");
    field.as_json_file(file).unwrap();
    let field_from_json = Field::from_json_file(file).unwrap();
    assert_eq!(field_from_json.name, "field");
    clean_up(file);
}

#[test]
pub fn test_derive_yaml() {
    let file = "./tests/field.yaml";
    let field = Field::new("field");
    field.as_yaml_file(file).unwrap();
    let field_from_json = Field::from_yaml_file(file).unwrap();
    assert_eq!(field_from_json.name, "field");
    clean_up(file);
}