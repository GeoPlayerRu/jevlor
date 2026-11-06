use crate::language::alphabet::Alphabet;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename = "Alphabet", into = "Alphabet")]
pub struct AlphabetPrototype {
    pub letters: String,
    pub vowels: String,
    pub consonants: String,
}
