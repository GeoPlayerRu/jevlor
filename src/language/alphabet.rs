use crate::serde::alphabet::AlphabetPrototype;
use serde::{Deserialize, Serialize};

/// Structure that contains information about letters in language.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(from = "AlphabetPrototype")]
pub struct Alphabet {
    /// All leters that alphabet can provide.
    pub letters: Vec<char>,
    vowels: Vec<char>,
    consonants: Vec<char>,
}

impl Alphabet {
    pub fn new(letters: Vec<char>) -> Self {
        Self {
            letters,
            vowels: vec![],
            consonants: vec![],
        }
    }

    pub fn define_vowels(&mut self, vowels: Vec<char>) {
        self.vowels = vowels;
    }

    pub fn define_consonants(&mut self, consonants: Vec<char>) {
        self.consonants = consonants;
    }

    /// Get vowels as vec of [char].
    pub fn get_vowels(&self) -> Vec<char> {
        self.vowels.iter().copied().collect()
    }

    /// Get consonants as vec of [UlPair].
    pub fn get_consonants(&self) -> Vec<char> {
        self.consonants.iter().copied().collect()
    }

    /// Checks whether alphabet considers char a vowel.
    pub fn is_vowel(&self, ch: char) -> bool {
        self.vowels.contains(&ch)
    }
}

impl From<AlphabetPrototype> for Alphabet {
    fn from(value: AlphabetPrototype) -> Self {
        Self {
            letters: value.letters.chars().collect(),
            vowels: value.vowels.chars().collect(),
            consonants: value.consonants.chars().collect(),
        }
    }
}
