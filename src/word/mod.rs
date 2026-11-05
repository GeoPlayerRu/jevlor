use std::collections::HashMap;

use crate::morphem::{Morphem, MorphemType};

pub mod builder;

#[derive(Clone)]
pub struct Word {
    pub(crate) prefixes: Vec<Morphem>,
    pub(crate) cores: Vec<Morphem>,
    pub(crate) suffixes: Vec<Morphem>,
    pub(crate) ending: Option<Morphem>,
    pub(crate) postfix: Option<Morphem>,
}

impl Word {
    pub fn from_morphems(mut map: HashMap<MorphemType, Vec<Morphem>>) -> Option<Self> {
        let Some(cores) = map.remove(&MorphemType::Core) else {
            return None;
        };

        if cores.len() < 1 {
            return None;
        }

        Some(Self {
            prefixes: map.remove(&MorphemType::Prefix).unwrap_or(Vec::new()),
            cores: cores,
            suffixes: map.remove(&MorphemType::Suffix).unwrap_or(Vec::new()),
            ending: map.remove(&MorphemType::Ending)?.into_iter().next(),
            postfix: map.remove(&MorphemType::Postfix)?.into_iter().next(),
        })
    }

    pub fn split_into_morphems(self) -> HashMap<MorphemType, Vec<Morphem>> {
        [(MorphemType::Prefix, self.prefixes),
                (MorphemType::Core, self.cores),
                (MorphemType::Suffix, self.suffixes),
                (MorphemType::Ending, self.ending.map(|v| vec![v]).unwrap_or(vec![])),
                (MorphemType::Postfix, self.postfix.map(|v| vec![v]).unwrap_or(vec![])),
        ].iter().cloned().collect()
    }
}
