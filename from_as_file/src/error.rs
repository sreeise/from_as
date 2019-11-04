use std::ffi::OsStr;
use std::io::ErrorKind;

#[derive(Debug)]
pub enum FromAsError {
    Io(std::io::Error),
    SerdeJsonError(serde_json::error::Error),
    SerdeYamlError(serde_yaml::Error),
    TomlDeError(toml::de::Error),
    TomlSerError(toml::ser::Error),
}

impl FromAsError {
    pub fn invalid_extension(ext: &OsStr) -> FromAsError {
        let s = format!("Expected json or yaml extension, got: {:#?}", ext);
        FromAsError::Io(std::io::Error::new(ErrorKind::InvalidData, s))
    }
}

impl std::error::Error for FromAsError {
    fn description(&self) -> &str {
        match *self {
            FromAsError::Io(ref err) => err.description(),
            FromAsError::SerdeJsonError(ref err) => err.description(),
            FromAsError::SerdeYamlError(ref err) => err.description(),
            FromAsError::TomlDeError(ref err) => err.description(),
            FromAsError::TomlSerError(ref err) => err.description(),
        }
    }

    fn source<'a>(&'a self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            FromAsError::Io(ref err) => Some(err),
            FromAsError::SerdeJsonError(ref err) => Some(err),
            FromAsError::SerdeYamlError(ref err) => Some(err),
            FromAsError::TomlDeError(ref err) => Some(err),
            FromAsError::TomlSerError(ref err) => Some(err),
        }
    }
}

impl std::fmt::Display for FromAsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nError Code: {:#?}", &self)
    }
}

impl From<std::io::Error> for FromAsError {
    fn from(err: std::io::Error) -> Self {
        FromAsError::Io(err)
    }
}

impl From<serde_json::error::Error> for FromAsError {
    fn from(err: serde_json::error::Error) -> Self {
        FromAsError::SerdeJsonError(err)
    }
}

impl From<serde_yaml::Error> for FromAsError {
    fn from(err: serde_yaml::Error) -> Self {
        FromAsError::SerdeYamlError(err)
    }
}

impl From<toml::de::Error> for FromAsError {
    fn from(err: toml::de::Error) -> Self {
        FromAsError::TomlDeError(err)
    }
}

impl From<toml::ser::Error> for FromAsError {
    fn from(err: toml::ser::Error) -> Self {
        FromAsError::TomlSerError(err)
    }
}
