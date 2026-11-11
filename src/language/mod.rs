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
pub use crate::language::{alphabet::Alphabet, morphem::MorphemicRules};
use crate::serde::language::LanguagePrototype;
use serde::{Deserialize, Serialize};

/// Contains structures that describe letters and information about them.
pub mod alphabet;
/// Contains structures that descibe additional information about morphems in context of language.
pub mod morphem;
/// Contains structures that give additional information about language's statistics.
pub mod statistics;

/// Structure that contains rules, alphabet of some language.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(from = "LanguagePrototype")]
pub struct Language {
    /// Information about letters.
    pub alphabet: Alphabet,
    /// Information about morphems.
    pub morphem_rules: MorphemicRules,
}

impl From<LanguagePrototype> for Language {
    fn from(value: LanguagePrototype) -> Self {
        Self {
            alphabet: value.alphabet.unwrap_or(Alphabet::new(vec![' '])),
            morphem_rules: value.concat_rules.into(),
        }
    }
}
