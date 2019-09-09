use std::convert::TryFrom;
use std::ffi::OsStr;

pub enum Ext {
    Yaml,
    Json,
    Toml,
}

impl Ext {
    pub fn as_str(&self) -> &'static str {
        match self {
            Ext::Json => "json",
            Ext::Yaml => "yaml",
            Ext::Toml => "toml",
        }
    }
}

impl AsRef<str> for Ext {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&OsStr> for Ext {
    type Error = std::io::Error;

    fn try_from(value: &OsStr) -> Result<Self, Self::Error> {
        match value.to_str() {
            Some("json") => Ok(Ext::Json),
            Some("yaml") => Ok(Ext::Yaml),
            Some("toml") => Ok(Ext::Toml),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Invalid extension. Valid extensions are json, yaml, and toml.  Got: {:#?}",
                    value
                ),
            )),
        }
    }
}

impl From<Ext> for &OsStr {
    fn from(ext: Ext) -> Self {
        OsStr::new(ext.as_str())
    }
}
