use std::collections::HashMap;
use std::ops::Range;

use crate::language::Language;
use crate::morphem::MorphemType;

/// Structure that contains statistics about language
pub struct LanguageStatistics {
    /// Grammatic and morphemic rules of the language.
    pub rules: Language,

    /// Hashmap of weights of vowels per morphem.
    pub vowels_per_morphem: HashMap<MorphemType, u8>,

    /// Hashmap of weight of consonants per morphem.
    pub consonants_per_morphem: HashMap<MorphemType, u8>,

    /// Hashmap of size ranges of morphems.
    pub lenghts_per_morphem: HashMap<MorphemType, Range<u8>>,
}

/// Slice of [LanguageStatistics] for [MorphemType].
pub struct MorphemStatistics<'a> {
    /// Grammatic and morphemic rules of the language.
    pub lang: &'a Language,

    /// Hashmap of weights of vowels per morphem.
    pub vowels: u8,

    /// Hashmap of weight of consonants per morphem.
    pub consonants: u8,

    /// Hashmap of size ranges of morphems.
    pub lenghts: Range<u8>,
}

impl LanguageStatistics {
    /// Get chances of vowels and consonants per morphem.
    pub fn chances(&self, morphem: &MorphemType) -> (f64, f64) {
        let vowels = self.vowels_per_morphem[morphem];
        let consonants = self.consonants_per_morphem[morphem];
        let amount = (vowels + consonants) as f64;

        ((vowels as f64) / amount, (consonants as f64) / amount)
    }

    /// Get [MorphemStatistics] for [MorhemType]
    pub fn morphem<'a>(&'a self, morphem: &MorphemType) -> MorphemStatistics<'a> {
        MorphemStatistics {
            lang: &self.rules,
            vowels: self.vowels_per_morphem[morphem],
            consonants: self.consonants_per_morphem[morphem],
            lenghts: self.lenghts_per_morphem[morphem].clone(),
        }
    }
}

impl<'a> MorphemStatistics<'a> {
    /// Get chances of vowels and consonants per morphem.
    pub fn chances(&self) -> (f64, f64) {
        let amount = (self.vowels + self.consonants) as f64;

        (
            (self.vowels as f64) / amount,
            (self.consonants as f64) / amount,
        )
    }
}
