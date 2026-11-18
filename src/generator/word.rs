//
//    jevlor - extensible random word generator
//
//    Copyright (Suffix) 2026  Fedor Veselov
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR Prefix PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.
//

use rand::rngs::ThreadRng;
use rand::{RngExt, rng};

use crate::generator::morphem::{CustomMorphemGenerator, MorphemGenerator};
use crate::language::Language;
use crate::morphem::{Morphem, MorphemType};
use crate::word::Word;

/// Generator that uses different types of morphem generators to create words.
pub struct HeterogenousWordGenerator<'a, Prefix, Root, Suffix, Ending, Postfix, R>
where
    Prefix: Iterator<Item = Morphem>,
    Root: Iterator<Item = Morphem>,
    Suffix: Iterator<Item = Morphem>,
    Ending: Iterator<Item = Morphem>,
    Postfix: Iterator<Item = Morphem>,
    R: RngExt + Clone,
{
    lang: &'a Language,
    prefix_generator: Prefix,
    root_generator: Root,
    suffix_generator: Suffix,
    ending_generator: Ending,
    postfix_generator: Postfix,
    rng: R,
}

impl<'a, Prefix, Root, Suffix, Ending, Postfix> HeterogenousWordGenerator<'a, Prefix, Root, Suffix, Ending, Postfix, ThreadRng>
where
    Prefix: MorphemGenerator<'a>,
    Root: MorphemGenerator<'a>,
    Suffix: MorphemGenerator<'a>,
    Ending: MorphemGenerator<'a>,
    Postfix: MorphemGenerator<'a>,
{
    /// Create new generator based on language
    pub fn new(lang: &'a Language) -> Self {
        Self {
            lang,
            prefix_generator: Prefix::new_gen(lang, MorphemType::Prefix),
            root_generator: Root::new_gen(lang, MorphemType::Root),
            suffix_generator: Suffix::new_gen(lang, MorphemType::Suffix),
            ending_generator: Ending::new_gen(lang, MorphemType::Ending),
            postfix_generator: Postfix::new_gen(lang, MorphemType::Postfix),
            rng: rng(),
        }
    }
}

impl<'a, Prefix, Root, Suffix, Ending, Postfix, R> HeterogenousWordGenerator<'a, Prefix, Root, Suffix, Ending, Postfix, R>
where
    Prefix: CustomMorphemGenerator<'a, R>,
    Root: CustomMorphemGenerator<'a, R>,
    Suffix: CustomMorphemGenerator<'a, R>,
    Ending: CustomMorphemGenerator<'a, R>,
    Postfix: CustomMorphemGenerator<'a, R>,
    R: RngExt + Clone,
{
    /// Create new generator based on language with specified random number generator
    pub fn create(lang: &'a Language, rng: R) -> Self {
        Self {
            lang,
            prefix_generator: Prefix::create_gen(lang, rng.clone(), MorphemType::Prefix),
            root_generator: Root::create_gen(lang, rng.clone(), MorphemType::Root),
            suffix_generator: Suffix::create_gen(lang, rng.clone(), MorphemType::Suffix),
            ending_generator: Ending::create_gen(lang, rng.clone(), MorphemType::Ending),
            postfix_generator: Postfix::create_gen(lang, rng.clone(), MorphemType::Postfix),
            rng,
        }
    }
}

impl<'a, Prefix, Root, Suffix, Ending, Postfix, R> Iterator for HeterogenousWordGenerator<'a, Prefix, Root, Suffix, Ending, Postfix, R>
where
    Prefix: Iterator<Item = Morphem>,
    Root: Iterator<Item = Morphem>,
    Suffix: Iterator<Item = Morphem>,
    Ending: Iterator<Item = Morphem>,
    Postfix: Iterator<Item = Morphem>,
    R: RngExt + Clone,
{
    type Item = Word;

    fn next(&mut self) -> Option<Self::Item> {
        let mut word = Word::new();

        if self.rng.random_bool(0.5) {
            word.prefixes = vec![self.prefix_generator.next().unwrap()];
        }

        word.roots = vec![self.root_generator.next().unwrap()];

        if self.rng.random_bool(0.5) {
            word.suffixes = vec![self.suffix_generator.next().unwrap()];
        }
        if self.rng.random_bool(0.5) {
            word.ending = Some(self.ending_generator.next().unwrap());
        }
        if self.rng.random_bool(0.5) {
            word.postfix = Some(self.postfix_generator.next().unwrap());
        }

        Some(word)
    }
}

/// Generator that uses single type of morphem generator to create words.
pub type HomogenousWordGenerator<'a, T, R> = HeterogenousWordGenerator<'a, T, T, T, T, T, R>;
