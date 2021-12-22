# Derive macros for the from_as crate

# This crate is meant to be used as part of the [from_as](https://crates.io/crates/from_as) crate and may work as intended by itself.
# Please use https://crates.io/crates/from_as

# from_as
Rust derive macros for reading and writing files for types that implement serde. The derive
macros implement the from_as_file traits.

The from_as_file crate provides two traits: FromFile and AsFile. FromFile is used for getting
types from a file. AsFile is used for writing a types to a file.

The derive_from_as crate provides derive macros for these traits with the same names.

Currently, the only files types that can be used are json, yaml, and toml.

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
