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

/// Return result of from* functions. Exists to not waste result in case you need invalid word and
/// and not to create too many functions.
pub enum WordWithLabel {
    Valid(Word),
    Invalid(Word),
}

impl WordWithLabel {
    /// Consumes [WordWithLabel] to get [Word]
    pub fn extract(self) -> Word {
        match self {
            WordWithLabel::Valid(word) => word,
            WordWithLabel::Invalid(word) => word,
        }
    }
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

    /// Tries to construct word out of morphems mapped to their type. Return [WordWithLabel::Invalid] if there are no
    /// roots.
    pub fn from_morphem_hashmap(map: HashMap<MorphemType, Vec<String>>) -> WordWithLabel {
        let mut morphem_map: HashMap<MorphemType, Vec<Morphem>> = map
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    v.into_iter()
                        .map(|ms| Morphem::new(ms, k.clone()))
                        .collect(),
                )
            })
            .collect();

        let result = Self {
            prefixes: morphem_map
                .remove(&MorphemType::Prefix)
                .unwrap_or(Vec::new()),
            roots: morphem_map.remove(&MorphemType::Root).unwrap_or(Vec::new()),
            suffixes: morphem_map
                .remove(&MorphemType::Suffix)
                .unwrap_or(Vec::new()),
            ending: morphem_map
                .remove(&MorphemType::Ending)
                .map(|v| v.into_iter().next())
                .flatten(),
            postfix: morphem_map
                .remove(&MorphemType::Postfix)
                .map(|v| v.into_iter().next())
                .flatten(),
        };

        if result.roots.len() < 1 {
            return WordWithLabel::Invalid(result);
        }

        WordWithLabel::Valid(result)
    }

    /// Tries to parse vector of morphems into word. Returns [WordWithLabel::Invalid] if there are no roots in resulting
    /// word.
    pub fn from_morphem_vec(vec: Vec<Morphem>) -> WordWithLabel {
        let mut result: Self = Self {
            prefixes: vec![],
            roots: vec![],
            suffixes: vec![],
            ending: None,
            postfix: None,
        };

        for m in vec {
            match m.kind() {
                MorphemType::Prefix => result.prefixes.push(m),
                MorphemType::Root => result.roots.push(m),
                MorphemType::Suffix => result.suffixes.push(m),
                MorphemType::Ending if let None = result.ending => result.ending = Some(m),
                MorphemType::Postfix if let None = result.postfix => result.postfix = Some(m),
                _ => continue,
            }
        }

        if result.roots.len() < 1 {
            return WordWithLabel::Invalid(result);
        }

        WordWithLabel::Valid(result)
    }

    /// Splits word into hashmap of morphem types and morphems. Endings and postfixes are returned as
    /// vector with zero or one element.
    pub fn into_morphems_hashmap(&self) -> HashMap<MorphemType, Vec<String>> {
        [
            (
                MorphemType::Prefix,
                self.prefixes.iter().map(|v| v.content()).collect(),
            ),
            (
                MorphemType::Root,
                self.roots.iter().map(|v| v.content()).collect(),
            ),
            (
                MorphemType::Suffix,
                self.suffixes.iter().map(|v| v.content()).collect(),
            ),
            (
                MorphemType::Ending,
                self.ending
                    .clone()
                    .map(|v| vec![v.content()])
                    .unwrap_or(vec![]),
            ),
            (
                MorphemType::Postfix,
                self.postfix
                    .clone()
                    .map(|v| vec![v.content()])
                    .unwrap_or(vec![]),
            ),
        ]
        .into_iter()
        .collect()
    }

    /// Splits word into vector of morphems.
    pub fn into_morphems_vec(&self) -> Vec<Morphem> {
        let mut result = vec![];

        result.append(&mut self.prefixes.clone());
        result.append(&mut self.roots.clone());
        result.append(&mut self.suffixes.clone());

        if let Some(ending) = self.ending.clone() {
            result.push(ending);
        }

        if let Some(postfix) = self.postfix.clone() {
            result.push(postfix);
        }

        result
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
