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
use std::path::Path;

use crate::language::{Alphabet, Language, statistics::Statistics};
use loader::FromTomlFile;

/// Structs for serializing and deserializing alphabet.
pub mod alphabet;
/// Structs for serializing and deserializing language structures.
pub mod language;
/// Traits with blanket implementations for RW
pub mod loader;
/// Structs for serializing and deserializing statistic structures.
pub mod statistics;

const ALPHABET_LOOKUP_FILE: &str = "alphabet.toml";
const LANGUAGE_LOOKUP_FILE: &str = "rules.toml";
const STATISTICS_LOOKUP_FILE: &str = "statistics.toml";

/// Loads language from desired directory
pub fn load_language<P: AsRef<Path>>(path: P) -> Result<Language, anyhow::Error> {
    let path = path.as_ref();
    let mut lang_prot =
        language::LanguagePrototype::from_toml_file(path.join(LANGUAGE_LOOKUP_FILE))?;

    if lang_prot.alphabet.is_none() {
        let alphabet = Alphabet::from_toml_file(path.join(ALPHABET_LOOKUP_FILE))?;
        lang_prot.alphabet = Some(alphabet);
    }

    if lang_prot.stats.is_none() {
        let stats = Statistics::from_toml_file(path.join(STATISTICS_LOOKUP_FILE))?;
        lang_prot.stats = Some(stats);
    }

    Ok(lang_prot.into())
}
