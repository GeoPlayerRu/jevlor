use std::collections::HashMap;
use std::ops::Range;

use crate::morphem::MorphemType;
pub use crate::rules::morphem::MorphemicRules;

// bullshit structures

/// Uppercase-lowercase pair
#[derive(Clone, PartialEq, Eq)]
pub struct ULPair(char,char);

impl ULPair {
    /// Creates new [ULPair] from `upper` and `lower` chars.
    pub fn new(upper: char, lower: char) -> Self {
        Self(upper, lower)
    }

    /// Returns right part of [ULPair] that corresponds to lowercase.
    #[inline(always)]
    pub fn lowercase(&self) -> char {
        self.1
    }

    /// Returns left part of [ULPair] that corresponds to uppercase.
    #[inline(always)]
    pub fn uppercase(&self) -> char {
        self.0
    }
}

/// Structure that contains information about letters in language.
pub struct Alphabet {
    /// All leters that alphabet can provide.
    pub letters: Vec<ULPair>,
    vowels: Vec<usize>,
    consonants: Vec<usize>,
}

impl Alphabet {
    pub fn new(letters: Vec<ULPair>) -> Self {
        Self {
            letters,
            vowels: vec![],
            consonants: vec![],
        }
    }

    pub fn define_vowels(&mut self, vowels: Vec<ULPair>) {
        self.vowels = self.letters.iter().enumerate().filter_map(|(index, v)| vowels.contains(v).then(|| index)).collect();
    }

    pub fn define_consonants(&mut self, consonants: Vec<ULPair>) {
        self.consonants = self.letters.iter().enumerate().filter_map(|(index, v)| consonants.contains(v).then(|| index)).collect();
    }

    /// Get vowels as vec of [ULPair].
    pub fn get_vowels(&self) -> Vec<ULPair> {
        self.vowels.iter()
            .map(|index| self.letters[*index].clone())
            .collect()
    }
    
    /// Get consonants as vec of [UlPair].
    pub fn get_consonants(&self) -> Vec<ULPair> {
        self.consonants.iter()
            .map(|index| self.letters[*index].clone())
            .collect()
    }
}

/// Structure that contains rules, alphabet of some language.
pub struct Language {
    /// Information about letters.
    pub alphabet: Alphabet,
    /// Information about morphems.
    pub morphem_rules: MorphemicRules,
}

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

        ((self.vowels as f64) / amount, (self.consonants as f64) / amount)        
    }
}
