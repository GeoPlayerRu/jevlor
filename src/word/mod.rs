use std::{collections::HashMap, fmt::Display};

use crate::morphem::{Morphem, MorphemType};

/// Contains struct that helps to construct word correctly according to language rules.
pub mod builder;

/// Describes word constructed out of morphems.
///
/// Word is considered valid if it contains at least one root.
#[derive(Clone)]
pub struct Word {
    pub(crate) prefixes: Vec<Morphem>,
    pub(crate) roots: Vec<Morphem>,
    pub(crate) suffixes: Vec<Morphem>,
    pub(crate) ending: Option<Morphem>,
    pub(crate) postfix: Option<Morphem>,
}

impl Word {
    /// Constructs new and invalid word.
    pub fn new() -> Self {
        Self {
            prefixes: vec![],
            roots: vec![],
            suffixes: vec![],
            ending: None,
            postfix: None,
        }
    }

    /// Tries to construct word out of morphems mapped to their type. Return None if there are no
    /// roots.
    pub fn try_from_morphems(mut map: HashMap<MorphemType, Vec<Morphem>>) -> Option<Self> {
        let Some(cores) = map.remove(&MorphemType::Root) else {
            return None;
        };

        if cores.len() < 1 {
            return None;
        }

        Some(Self {
            prefixes: map.remove(&MorphemType::Prefix).unwrap_or(Vec::new()),
            roots: cores,
            suffixes: map.remove(&MorphemType::Suffix).unwrap_or(Vec::new()),
            ending: map
                .remove(&MorphemType::Ending)
                .map(|v| v.into_iter().next())
                .flatten(),
            postfix: map
                .remove(&MorphemType::Postfix)
                .map(|v| v.into_iter().next())
                .flatten(),
        })
    }

    /// Splits word into hasmap of morphem types and morphems. Endings and postfixes are returned as
    /// vector with zero or one element.
    pub fn split_into_morphems(&self) -> HashMap<MorphemType, Vec<Morphem>> {
        [
            (MorphemType::Prefix, self.prefixes.clone()),
            (MorphemType::Root, self.roots.clone()),
            (MorphemType::Suffix, self.suffixes.clone()),
            (
                MorphemType::Ending,
                self.ending.clone().map(|v| vec![v]).unwrap_or(vec![]),
            ),
            (
                MorphemType::Postfix,
                self.postfix.clone().map(|v| vec![v]).unwrap_or(vec![]),
            ),
        ]
        .into_iter()
        .collect()
    }
}

impl Display for Word {
    /// Formats word as plain concatenated morphems.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for prefix in &self.prefixes {
            prefix.fmt(f)?
        }
        for root in &self.roots {
            root.fmt(f)?
        }
        for suffix in &self.suffixes {
            suffix.fmt(f)?
        }
        if let Some(ending) = &self.ending {
            ending.fmt(f)?
        }
        if let Some(postfix) = &self.postfix {
            postfix.fmt(f)?
        }
        Ok(())
    }
}
