use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

/// Building block of words. Contains usually short string without special symbols.
#[derive(Clone, PartialEq, Eq)]
pub struct Morphem(String);

impl Morphem {
    pub fn new(morphem: String) -> Self {
        Self(morphem)
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

/// Type of morphem for building words.
#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub enum MorphemType {
    Prefix,
    Root,
    Suffix,
    Ending,
    Postfix,
}

/// Order of morphems for easier iteration.
pub const MORPHEM_ORDER: [MorphemType; 5] = [
    MorphemType::Prefix,
    MorphemType::Root,
    MorphemType::Suffix,
    MorphemType::Ending,
    MorphemType::Postfix,
];
