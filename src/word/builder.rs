use std::collections::HashMap;
use std::fmt::Write;

use crate::morphem::{MorphemType, MORPHEM_ORDER};
use crate::word::Word;
use crate::rules::ConcatRules;

pub struct WordBuilder{
    word: Word,
    rules: ConcatRules,
}

impl WordBuilder {
    pub fn build(self) -> String {
        let morphems = self.word.split_into_morphems();

        let capacity: usize = morphems.values().flatten().map(|m| m.len()).sum();
        let mut result = String::with_capacity(capacity);

        for kind in &MORPHEM_ORDER {
            if let Some(list) = morphems.get(kind) {
                for m in list {
                    result.push_str(m);
                }
            }
        }

        result
    }
}
