#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_from_to;

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

fn main() {
    let field = Field::new("name");
    field.as_json_file("./examples/field.json").unwrap();

    let field2 = Field::from_json_file("./examples/field.json").unwrap();
    println!("{:#?}", field2);
}
