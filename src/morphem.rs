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
use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of morphem for building words.
#[derive(Hash, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MorphemType {
    /// Prefix is morphem that preceeds root. Influences words meaning.
    Prefix,
    /// Root is main morphem of the word. Responsible for its meaning.
    Root,
    /// Suffix is morphem that goes after root but before ending. Influences word type and meaning.
    Suffix,
    /// Ending is morphem that goes after suffix and before postfix. Influences some properties of
    /// the word. E.g. gender, singular and plural forms.
    Ending,
    /// Postfix is morphem that goes after ending. Does not have definitive inlfuence.
    Postfix,
    /// Type for signalizing all morphem types,
    All,
}

/// Fills provided dictionary with value at [MorphemType::All]
pub fn fill_with_all<T: Clone>(mut map: HashMap<MorphemType, T>) -> HashMap<MorphemType, T> {
    if map.contains_key(&MorphemType::All) {
        for morphem in MORPHEM_ORDER {
            let value = map[&MorphemType::All].clone();
            map.entry(morphem).or_insert(value);
        }
        return map;
    }
    map
}
/// Building block of words. Contains usually short string without special symbols and its type.
#[derive(Clone, PartialEq, Eq)]
pub struct Morphem(String, MorphemType);

impl Morphem {
    /// Creates morphem out of string and its type
    pub fn new(morphem: String, kind: MorphemType) -> Self {
        Self(morphem, kind)
    }
    /// Returns type of morphem.
    pub fn kind(&self) -> MorphemType {
        self.1
    }
    /// Returns inner string.
    pub fn content(&self) -> String {
        self.0.clone()
    }
}

impl DerefMut for Morphem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Morphem {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Morphem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Order of morphems for easier iteration.
pub const MORPHEM_ORDER: [MorphemType; 5] = [
    MorphemType::Prefix,
    MorphemType::Root,
    MorphemType::Suffix,
    MorphemType::Ending,
    MorphemType::Postfix,
];
