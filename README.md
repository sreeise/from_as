# from_as
Rust traits and derive macros for reading and writing files for types that implement serde.

Available on [crates.io](https://crates.io/crates/from_as)

```toml
from_as = "0.1.1"
```

This crate provides two traits: FromFile and AsFile. FromFile is used for getting
types from a file. AsFile is used for writing types to a file.

The traits can be used for writing json, yaml, and toml files. 

### Example

```rust
    #[macro_use]
    extern crate serde_derive;
    use std::io::{Read, Write};
    use std::convert::TryFrom;
    use from_as::*;

    #[derive(Debug, Deserialize, Serialize, AsFile, FromFile)]
    struct Attribute {
        name: String,
    }
    
    fn main() {
        let attr = Attribute { 
            name: "attr_name".into()
        };
        
        // Write to the example directory.
        attr.as_file("./examples/attr.json").unwrap();
        
        let attr = Attribute::from_file("./examples/attr.json").unwrap();
        println!("{:#?}", attr);
        
        // For writing a prettified version.
        attr.as_file_pretty("./examples/attr.json").uwnrap();
    }
```
