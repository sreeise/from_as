#[macro_use]
extern crate serde_derive;
use from_as::*;

#[derive(Debug, Serialize, Deserialize, AsFile, FromFile)]
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
    // json
    let field = Field::new("name");
    field.as_file("./examples/field.json").unwrap();

    let field = Field::from_file("./examples/field.json").unwrap();
    println!("{:#?}", field);

    // yaml
    let field = Field::new("name");
    field.as_file("./examples/field.yaml").unwrap();

    let field = Field::from_file("./examples/field.yaml").unwrap();
    println!("{:#?}", field);

    // toml
    let field = Field::new("name");
    field.as_file("./examples/field.toml").unwrap();

    let field = Field::from_file("./examples/field.toml").unwrap();
    println!("{:#?}", field);
}
