//
//    jevlor - extensible random word generator
//
//    Copyright (C) 2026  Fedor Veselov
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
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
