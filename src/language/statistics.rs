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
use std::collections::BTreeMap;
use std::ops::Range;

use crate::morphem::MorphemType;
use crate::morphem::fill_with_all;

use serde::{Deserialize, Serialize};

use crate::serde::statistics::LanguageStatisticsPrototype;

/// Structure that contains statistics about language
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(from = "LanguageStatisticsPrototype")]
pub struct Statistics {
    /// Hashmap of weights of letters per morphem.
    pub morphem_chars_probability: BTreeMap<MorphemType, BTreeMap<char, f64>>,

    /// Hashmap of size ranges of morphems.
    pub lenghts_per_morphem: BTreeMap<MorphemType, Range<u8>>,

    pub morhem_data: MorphemData,
}

/// Slice of [LanguageStatistics] for [MorphemType].
#[derive(Debug)]
pub struct MorphemStatistics {
    /// Hashmap of weights of letters
    pub weights: BTreeMap<char, f64>,

    /// Hashmap of size ranges of morphems.
    pub lenghts: Range<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MorphemData {
    pub probabilities: BTreeMap<MorphemType, f64>,
}

impl Statistics {
    /// Get chances of vowels and consonants per morphem.
    pub fn chances(&self, morphem: &MorphemType) -> BTreeMap<char, f64> {
        self.morphem_chars_probability
            .get(morphem)
            .cloned()
            .unwrap()
    }

    /// Get [MorphemStatistics] for [MorhemType]
    pub fn morphem(&self, morphem: &MorphemType) -> MorphemStatistics {
        MorphemStatistics {
            weights: self.chances(morphem),
            lenghts: self.lenghts_per_morphem[morphem].clone(),
        }
    }
}

impl From<LanguageStatisticsPrototype> for Statistics {
    fn from(value: LanguageStatisticsPrototype) -> Self {
        Self {
            morphem_chars_probability: fill_with_all(
                value
                    .letters
                    .into_iter()
                    .map(|(mtype, stats)| (mtype, stats.hashmap_repr()))
                    .collect(),
            ),
            lenghts_per_morphem: fill_with_all(value.lenghts),
            morhem_data: value.morphems,
        }
    }
}
