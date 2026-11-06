use crate::language::Language;
use crate::language::morphem::MorphemConcatRule;
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
        let morphems = self.word.into_morphems_vec();

        let capacity: usize = morphems.iter().map(|v| v.len()).sum();
        let mut result = String::with_capacity(capacity);

        result.push_str(&morphems[0]);

        for m in morphems.windows(2) {
            if let Some(rule) = self
                .rules
                .morphem_rules
                .concat_rules
                .get(&(m[0].kind(), m[1].kind()))
            {
                match rule {
                    MorphemConcatRule::Concat => result.push_str(&m[1]),
                    MorphemConcatRule::Bridge(bridge) => {
                        result.push_str(bridge);
                        result.push_str(&m[1]);
                    }
                    MorphemConcatRule::Eliminate => continue,
                    //MorphemConcatRule::ReverseEliminate => todo!(),
                }
            }
        }

        result
    }
}
