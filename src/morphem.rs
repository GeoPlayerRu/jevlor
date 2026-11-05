use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

/// Type of morphem for building words.
#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub enum MorphemType {
    /// Prefix is morphem that preceeds root. Influences words meaning.
    Prefix,
    /// Root is main morphem of the word. Responsible for its meaning.
    Root,
    /// Suffix is morphem that goes after root but before ending. Influences word type and meaning.
    Suffix,
    /// Ending is morphem that goes after suffix and before postfix. Influences some properties of
    /// the word. E.g. gender, singular and plural forms.
    Ending,
    /// Postfix is morphem that goes after ending. Does not have definitive inlfuence.
    Postfix,
}

/// Building block of words. Contains usually short string without special symbols and its type.
#[derive(Clone, PartialEq, Eq)]
pub struct Morphem(String, MorphemType);

impl Morphem {
    /// Creates morphem out of string and its type
    pub fn new(morphem: String, kind: MorphemType) -> Self {
        Self(morphem, kind)
    }
    /// Returns type of morphem.
    pub fn kind(&self) -> MorphemType {
        self.1
    }
    /// Returns inner string.
    pub fn content(&self) -> String {
        self.0.clone()
    }
}

impl DerefMut for Morphem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Morphem {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Morphem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Order of morphems for easier iteration.
pub const MORPHEM_ORDER: [MorphemType; 5] = [
    MorphemType::Prefix,
    MorphemType::Root,
    MorphemType::Suffix,
    MorphemType::Ending,
    MorphemType::Postfix,
];
