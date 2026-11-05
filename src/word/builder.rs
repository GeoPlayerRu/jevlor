use crate::morphem::MORPHEM_ORDER;
use crate::rules::Language;
use crate::rules::morphem::MorphemConcatRule;
use crate::word::Word;

/// Constructs words using language's rules.
///
/// If you want to build word ignoring all rules, try [[Word::to_string()]]
pub struct WordBuilder<'a, 'b> {
    word: &'a Word,
    rules: &'b Language,
}

impl<'a, 'b> WordBuilder<'a, 'b> {
    // Create new builder using word and rules.
    pub fn new(word: &'a Word, rules: &'b Language) -> Self {
        Self { word, rules }
    }
    // Build word according to all rules.
    pub fn build(self) -> String {
        let morphems = self.word.split_into_morphems();

        let capacity: usize = morphems.values().flatten().map(|m| m.len()).sum();
        let mut result = String::with_capacity(capacity);

        for kind in &MORPHEM_ORDER {
            if let Some(list) = morphems.get(kind) {
                let mut first = true;
                for pairs in list.windows(2) {
                    // Concatenate using rules. If is first then need to add two morphems at once.
                    if first {
                        match &self.rules.concat_rules[kind] {
                            MorphemConcatRule::Concat => {
                                result.push_str(&pairs[0].to_string());
                                result.push_str(&pairs[1].to_string());
                            }
                            MorphemConcatRule::Bridge(bridge) => {
                                result.push_str(&pairs[0].to_string());
                                result.push_str(&bridge);
                                result.push_str(&pairs[1].to_string());
                            }
                            MorphemConcatRule::Eliminate => {
                                result.push_str(&pairs[0].to_string());
                            }
                            MorphemConcatRule::ReverseEliminate => continue,
                        }
                        first = false;
                    } else {
                        match &self.rules.concat_rules[kind] {
                            MorphemConcatRule::Concat => {
                                result.push_str(&pairs[1].to_string());
                            }
                            MorphemConcatRule::Bridge(bridge) => {
                                result.push_str(&bridge);
                                result.push_str(&pairs[1].to_string());
                            }
                            MorphemConcatRule::Eliminate => continue,
                            MorphemConcatRule::ReverseEliminate => continue,
                        }
                    }

                    if let MorphemConcatRule::ReverseEliminate = &self.rules.concat_rules[kind] {
                        result.push_str(
                            &list.last().map(|v| v.to_string()).unwrap_or("".to_string()),
                        );
                    }
                }
            }
        }

        result
    }
}
