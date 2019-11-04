#[macro_use]
extern crate serde_derive;

use from_as::*;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::hash::Hash;

#[derive(Debug, Serialize, Deserialize, FromFile, AsFile)]
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
        }
        Err(_) => {
            std::fs::remove_file(file).unwrap();
        }
    }
}

#[test]
pub fn test_derive_json() {
    let file = "./tests/field.json";
    let field = Field::new("field");
    field.as_file(file).unwrap();
    let field_from_json = Field::from_file(file).unwrap();
    assert_eq!(field_from_json.name, "field");
    clean_up(file);

    let field = Field::new("field");
    field.as_file_pretty(file).unwrap();
    let field_from_json = Field::from_file(file).unwrap();
    assert_eq!(field_from_json.name, "field");
    clean_up(file);
}

#[test]
pub fn test_derive_yaml() {
    let file = "./tests/field.yaml";
    let field = Field::new("field");
    field.as_file(file).unwrap();
    let field_from_json = Field::from_file(file).unwrap();
    assert_eq!(field_from_json.name, "field");
    clean_up(file);

    let field = Field::new("field");
    field.as_file_pretty(file).unwrap();
    let field_from_json = Field::from_file(file).unwrap();
    assert_eq!(field_from_json.name, "field");
    clean_up(file);
}

#[test]
pub fn test_derive_toml() {
    let file = "./tests/field.toml";
    let field = Field::new("field");
    field.as_file(file).unwrap();
    let field_from_json = Field::from_file(file).unwrap();
    assert_eq!(field_from_json.name, "field");
    clean_up(file);

    let field = Field::new("field");
    field.as_file_pretty(file).unwrap();
    let field_from_json = Field::from_file(file).unwrap();
    assert_eq!(field_from_json.name, "field");
    clean_up(file);
}

#[derive(Serialize, Deserialize, AsFile, FromFile)]
pub struct Collection<T> {
    vec: Vec<T>,
}

impl<T> Collection<T> {
    pub fn push(&mut self, value: T) {
        self.vec.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }
}

#[test]
fn test_generics() {
    let collection: Collection<Field> = Collection { vec: Vec::new() };
    collection.as_file("./tests/collection.toml").unwrap();
    let mut collection: Collection<Field> =
        Collection::from_file("./tests/collection.toml").unwrap();
    collection.push(Field::new("field"));
    collection.as_file("./tests/collection.json").unwrap();
    let mut collection: Collection<Field> =
        Collection::from_file("./tests/collection.json").unwrap();
    let field = collection.pop().unwrap();
    assert_eq!(field.name, "field".to_string());
    clean_up("./tests/collection.toml");
    clean_up("./tests/collection.json");

    let collection: Collection<Field> = Collection { vec: Vec::new() };
    collection
        .as_file_pretty("./tests/collection.toml")
        .unwrap();
    let mut collection: Collection<Field> =
        Collection::from_file("./tests/collection.toml").unwrap();
    collection.push(Field::new("field"));
    collection
        .as_file_pretty("./tests/collection.json")
        .unwrap();
    let mut collection: Collection<Field> =
        Collection::from_file("./tests/collection.json").unwrap();
    let field = collection.pop().unwrap();
    assert_eq!(field.name, "field".to_string());
    clean_up("./tests/collection.toml");
    clean_up("./tests/collection.json");
}

#[derive(Eq, PartialEq, Serialize, Deserialize, AsFile, FromFile)]
pub struct MultiGeneric<T, K, V>
where
    K: std::cmp::Eq + Hash,
{
    vec: Vec<T>,
    map: HashMap<K, V>,
}

impl<T, K, V> MultiGeneric<T, K, V>
where
    K: std::cmp::Eq + Hash,
{
    pub fn new(t: T, key: K, value: V) -> MultiGeneric<T, K, V> {
        let mut map = HashMap::new();
        map.insert(key, value);
        MultiGeneric { vec: vec![t], map }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }
}

#[derive(Serialize, Deserialize, AsFile, FromFile)]
pub struct Value<T> {
    v: T,
}

impl<T> Value<T> {
    pub fn new(v: T) -> Value<T> {
        Value { v }
    }
}

#[test]
fn test_multi_generic() {
    let multi = MultiGeneric::new(String::from("t"), String::from("key"), 3);
    multi.as_file("./tests/multi.toml").unwrap();
    let mut multi: MultiGeneric<String, String, usize> =
        MultiGeneric::from_file("./tests/multi.toml").unwrap();
    assert_eq!(multi.pop(), Some(String::from("t")));
    clean_up("./tests/multi.toml");

    let multi = MultiGeneric::new(String::from("t"), String::from("key"), 3);
    multi.as_file_pretty("./tests/multi.toml").unwrap();
    let mut multi: MultiGeneric<String, String, usize> =
        MultiGeneric::from_file("./tests/multi.toml").unwrap();
    assert_eq!(multi.pop(), Some(String::from("t")));
    clean_up("./tests/multi.toml");
}
