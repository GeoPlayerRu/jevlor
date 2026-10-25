use std::collections::HashMap;

use crate::morphem::{Morphem, MorphemType};

pub mod builder;

#[derive(Clone)]
pub struct Word {
    pub(crate) prefixes: Vec<Morphem>,
    pub(crate) cores: Vec<Morphem>,
    pub(crate) suffixes: Vec<Morphem>,
    pub(crate) ending: Morphem,
    pub(crate) postfix: Morphem,
}

impl Word {
    pub fn from_morphems(mut map: HashMap<MorphemType, Vec<Morphem>>) -> Option<Self> {
        Some(Self {
            prefixes: map.remove(&MorphemType::Prefix)?,
            cores: map.remove(&MorphemType::Core)?,
            suffixes: map.remove(&MorphemType::Suffix)?,
            ending: map.remove(&MorphemType::Ending)?.into_iter().next()?,
            postfix: map.remove(&MorphemType::Postfix)?.into_iter().next()?,
        })
    }

    pub fn split_into_morphems(self) -> HashMap<MorphemType, Vec<Morphem>> {
        [(MorphemType::Prefix, self.prefixes),
                (MorphemType::Core, self.cores),
                (MorphemType::Suffix, self.suffixes),
                (MorphemType::Ending, vec![self.ending]),
                (MorphemType::Postfix, vec![self.postfix]),
        ].iter().cloned().collect()
    }
}
