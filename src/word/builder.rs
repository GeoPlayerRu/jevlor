//
//    jevlor - extensible random word generator
//
//    Copyright (C) 2026  Fedor Veselov
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
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
                match &rule[0] {
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
