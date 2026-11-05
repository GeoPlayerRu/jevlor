use crate::morphem::{MORPHEM_ORDER};
use crate::word::Word;
use crate::rules::Language;

/// Constructs words using language's rules.
///
/// If you want to build word ignoring all rules, try [[Word::to_string()]]
pub struct WordBuilder<'a, 'b>{
    word: &'a Word,
    rules: &'b Language,
}

impl<'a, 'b> WordBuilder<'a, 'b> {
    // Create new builder using word and rules.
    pub fn new(word: &'a Word, rules: &'b Language) -> Self {
        Self {
            word,
            rules,
        }
    }
    // Build word according to all rules.
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
