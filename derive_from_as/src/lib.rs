#![recursion_limit = "256"]
extern crate proc_macro;
extern crate quote;
extern crate serde_derive;

use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, GenericParam, Generics};

fn add_ser_bound(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(serde::Serialize));
        }
    }
    generics
}

#[proc_macro_derive(FromFile)]
pub fn derive_from_file(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics from_as_file::FromFile for #name #ty_generics #where_clause {
            type Error = from_as_file::FromAsError;

            fn from_file<P: AsRef<std::path::Path>>(path: P) -> std::result::Result<Self, Self::Error>
                where
                    for<'de> Self: serde::Deserialize<'de>
            {
                if let Some(ext) = path.as_ref().to_path_buf().extension() {
                    let ext = from_as_file::Ext::try_from(ext)?;
                    let mut f = std::fs::File::open(path)?;

                    match ext {
                        from_as_file::Ext::Yaml => {
                            let t: Self = serde_yaml::from_reader(f)?;
                            return Ok(t);
                        }
                        from_as_file::Ext::Json => {
                            let t: Self = serde_json::from_reader(f)?;
                            return Ok(t);
                        }
                        from_as_file::Ext::Toml => {
                            let mut buffer = String::new();
                            f.read_to_string(&mut buffer)?;
                            let t: Self = toml::from_str(buffer.as_str())?;
                            Ok(t)
                        }
                    }
                } else {
                    Err(from_as_file::FromAsError::Io(
                        std::io::Error::new(std::io::ErrorKind::InvalidData,
                        "Could not determine file type - either the given path is a directory or does not have a valid extension"
                    )))
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(AsFile)]
pub fn derive_as_file(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = add_ser_bound(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics from_as_file::AsFile for #name #ty_generics #where_clause {
            type Error = from_as_file::FromAsError;

            fn as_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Self::Error>
            {
                if let Some(ext) = path.as_ref().to_path_buf().extension() {
                    let ext = from_as_file::Ext::try_from(ext)?;
                    let mut file = std::fs::OpenOptions::new()
                        .create(true)
                        .write(true)
                        .open(&path)?;

                    match ext {
                        from_as_file::Ext::Yaml => {
                            let serialized = serde_yaml::to_string(&self)?;
                            file.write_all(serialized.as_bytes())?;
                        }
                        from_as_file::Ext::Json => {
                            let serialized = serde_json::to_string(&self)?;
                            file.write_all(serialized.as_bytes())?;
                        }
                        from_as_file::Ext::Toml => {
                            let serialized = toml::to_string(&self)?;
                            file.write_all(serialized.as_bytes())?;
                        }
                    };

                    file.sync_all()?;
                    Ok(())
                } else {
                    Err(from_as_file::FromAsError::Io(
                        std::io::Error::new(std::io::ErrorKind::InvalidData,
                        "Could not determine file type - either the given path is a directory or does not have a valid extension"
                    )))
                }
            }

             fn as_file_pretty<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Self::Error> {
                if let Some(ext) = path.as_ref().to_path_buf().extension() {
                        let ext = from_as_file::Ext::try_from(ext)?;
                        let mut file = std::fs::OpenOptions::new()
                            .create(true)
                            .write(true)
                            .open(&path)?;

                        match ext {
                            from_as_file::Ext::Yaml => {
                                let serialized = serde_yaml::to_string(&self)?;
                                file.write_all(serialized.as_bytes())?;
                            }
                            from_as_file::Ext::Json => {
                                let serialized = serde_json::to_string_pretty(&self)?;
                                file.write_all(serialized.as_bytes())?;
                            }
                            from_as_file::Ext::Toml => {
                                let serialized = toml::to_string_pretty(&self)?;
                                file.write_all(serialized.as_bytes())?;
                            }
                        };

                    file.sync_all()?;
                    Ok(())
                } else {
                    Err(from_as_file::FromAsError::Io(
                        std::io::Error::new(std::io::ErrorKind::InvalidData,
                        "Could not determine file type - either the given path is a directory or does not have a valid extension"
                    )))
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
