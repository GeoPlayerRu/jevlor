use std::collections::HashMap;
use std::ops::Range;

use crate::morphem::MorphemType;
use crate::{language::Language, morphem::fill_with_all};

use serde::{Deserialize, Serialize};

use crate::serde::statistics::LanguageStatisticsPrototype;

/// Structure that contains statistics about language
#[derive(Serialize, Deserialize, Debug)]
#[serde(from = "LanguageStatisticsPrototype")]
pub struct LanguageStatistics {
    /// Grammatic and morphemic rules of the language.
    pub rules: Option<Language>,

    /// Hashmap of weights of letters per morphem.
    pub morphem_chars_probability: HashMap<MorphemType, HashMap<char, f64>>,

    /// Hashmap of size ranges of morphems.
    pub lenghts_per_morphem: HashMap<MorphemType, Range<u8>>,
}

/// Slice of [LanguageStatistics] for [MorphemType].
pub struct MorphemStatistics<'a> {
    /// Grammatic and morphemic rules of the language.
    pub lang: Option<&'a Language>,

    /// Hashmap of weights of letters
    pub weights: HashMap<char, f64>,

    /// Hashmap of size ranges of morphems.
    pub lenghts: Range<u8>,
}

impl LanguageStatistics {
    /// Get chances of vowels and consonants per morphem.
    pub fn chances(&self, morphem: &MorphemType) -> HashMap<char, f64> {
        self.morphem_chars_probability
            .get(morphem)
            .cloned()
            .unwrap()
    }

    /// Get [MorphemStatistics] for [MorhemType]
    pub fn morphem<'a>(&'a self, morphem: &MorphemType) -> MorphemStatistics<'a> {
        MorphemStatistics {
            lang: self.rules.as_ref(),
            weights: self.chances(morphem),
            lenghts: self.lenghts_per_morphem[morphem].clone(),
        }
    }
}

impl From<LanguageStatisticsPrototype> for LanguageStatistics {
    fn from(value: LanguageStatisticsPrototype) -> Self {
        Self {
            rules: value.rules,
            morphem_chars_probability: fill_with_all(
                value
                    .letters
                    .into_iter()
                    .map(|(mtype, stats)| (mtype, stats.hashmap_repr()))
                    .collect(),
            ),
            lenghts_per_morphem: fill_with_all(value.lenghts),
        }
    }
}
