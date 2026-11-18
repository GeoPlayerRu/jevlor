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

use rand::rngs::ThreadRng;
use rand::{RngExt, rng};

use crate::generator::morphem::{CustomMorphemGenerator, MorphemGenerator};
use crate::language::Language;
use crate::morphem::{Morphem, MorphemType};
use crate::word::Word;

/// Generator that uses different types of morphem generators to create words.
pub struct HeterogenousWordGenerator<'a, A, B, C, D, E, R>
where
    A: Iterator<Item = Morphem>,
    B: Iterator<Item = Morphem>,
    C: Iterator<Item = Morphem>,
    D: Iterator<Item = Morphem>,
    E: Iterator<Item = Morphem>,
    R: RngExt + Clone,
{
    lang: &'a Language,
    prefix_generator: A,
    root_generator: B,
    suffix_generator: C,
    ending_generator: D,
    postfix_generator: E,
    rng: R,
}

impl<'a, A, B, C, D, E> HeterogenousWordGenerator<'a, A, B, C, D, E, ThreadRng>
where
    A: MorphemGenerator<'a>,
    B: MorphemGenerator<'a>,
    C: MorphemGenerator<'a>,
    D: MorphemGenerator<'a>,
    E: MorphemGenerator<'a>,
{
    /// Create new generator based on language
    pub fn new(lang: &'a Language) -> Self {
        Self {
            lang,
            prefix_generator: A::new_gen(lang, MorphemType::Prefix),
            root_generator: B::new_gen(lang, MorphemType::Root),
            suffix_generator: C::new_gen(lang, MorphemType::Suffix),
            ending_generator: D::new_gen(lang, MorphemType::Ending),
            postfix_generator: E::new_gen(lang, MorphemType::Postfix),
            rng: rng(),
        }
    }
}

impl<'a, A, B, C, D, E, R> HeterogenousWordGenerator<'a, A, B, C, D, E, R>
where
    A: CustomMorphemGenerator<'a, R>,
    B: CustomMorphemGenerator<'a, R>,
    C: CustomMorphemGenerator<'a, R>,
    D: CustomMorphemGenerator<'a, R>,
    E: CustomMorphemGenerator<'a, R>,
    R: RngExt + Clone,
{
    /// Create new generator based on language with specified random number generator
    pub fn create(lang: &'a Language, rng: R) -> Self {
        Self {
            lang,
            prefix_generator: A::create_gen(lang, rng.clone(), MorphemType::Prefix),
            root_generator: B::create_gen(lang, rng.clone(), MorphemType::Root),
            suffix_generator: C::create_gen(lang, rng.clone(), MorphemType::Suffix),
            ending_generator: D::create_gen(lang, rng.clone(), MorphemType::Ending),
            postfix_generator: E::create_gen(lang, rng.clone(), MorphemType::Postfix),
            rng,
        }
    }
}

impl<'a, A, B, C, D, E, R> Iterator for HeterogenousWordGenerator<'a, A, B, C, D, E, R>
where
    A: Iterator<Item = Morphem>,
    B: Iterator<Item = Morphem>,
    C: Iterator<Item = Morphem>,
    D: Iterator<Item = Morphem>,
    E: Iterator<Item = Morphem>,
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
