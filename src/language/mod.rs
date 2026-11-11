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
