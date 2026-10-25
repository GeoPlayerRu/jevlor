use std::collections::HashMap;
use std::fmt::Write;

use crate::morphem::MorphemType;
use crate::word::Word;
use crate::rules::ConcatRules;

pub struct WordBuilder{
    word: Word,
    rules: ConcatRules,
}

impl WordBuilder {
    pub fn build(self) -> String {
        let morphems_map = self.word.split_into_morphems();

        let built_morphems: HashMap<MorphemType, String> = morphems_map.into_iter().map(|(kind, morphems)| {
            let capacity = morphems.iter().map(|m| m.len()).sum();
            let mut result = String::with_capacity(capacity);
            for morphem in morphems {
                result.push_str(&(*morphem));
            }
            (kind, result)
        }).collect();

        let morphem_order = [
            MorphemType::Prefix, MorphemType::Core, MorphemType::Suffix,
            MorphemType::Ending, MorphemType::Postfix
        ];

        let mut result = String::with_capacity(built_morphems.values().map(String::len).sum());
        morphem_order.iter().filter_map(|t| built_morphems.get(t)).for_each(|morphems| result.push_str(morphems));

        result
    }
}
