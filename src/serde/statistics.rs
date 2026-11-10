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
