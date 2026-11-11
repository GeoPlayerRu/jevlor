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
use crate::language::Language;
use crate::language::statistics::LanguageStatistics;
use crate::morphem::MorphemType;

use std::{collections::HashMap, ops::Range};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(into = "LanguageStatistics")]
pub(crate) struct LanguageStatisticsPrototype {
    pub rules: Option<Language>,
    pub letters: HashMap<MorphemType, LetterStatistics>,
    pub lenghts: HashMap<MorphemType, Range<u8>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct CharWeightPair(pub char, pub f64);

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub(crate) enum LetterStatistics {
    Ranked { value: String },
    Weighted { value: Vec<CharWeightPair> },
}

impl LetterStatistics {
    pub fn hashmap_repr(self) -> HashMap<char, f64> {
        match self {
            LetterStatistics::Ranked { value: ranks } => {
                let amount = ranks.len();
                let probabilities_sum = ((amount + 1) * amount / 2) as f64;
                let weights: HashMap<char, f64> = ranks
                    .chars()
                    .enumerate()
                    .map(|(i, v)| (v, 1.0 - ((i + 1) as f64) / probabilities_sum))
                    .collect();
                weights
            }
            LetterStatistics::Weighted { value: weights } => weights
                .into_iter()
                .map(|kvpair| (kvpair.0, kvpair.1))
                .collect(),
        }
    }
}
