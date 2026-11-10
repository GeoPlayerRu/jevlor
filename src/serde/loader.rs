use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::de::DeserializeOwned;

pub trait FromTomlFile: Sized {
    /// Load [Self] from toml file at path
    fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self, anyhow::Error>;
}

impl<T: DeserializeOwned> FromTomlFile for T {
    fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self, anyhow::Error> {
        let mut toml = String::new();
        File::options()
            .read(true)
            .open(path)?
            .read_to_string(&mut toml)?;

        Ok(toml::from_str(&toml)?)
    }
}
