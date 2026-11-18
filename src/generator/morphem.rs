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

use crate::generator::syllable::SyllableGenerator;
use crate::language::Language;
use crate::{
    language::statistics::MorphemStatistics,
    morphem::{Morphem, MorphemType},
};
use rand::{RngExt, SeedableRng, rng, rngs::ThreadRng, seq::IndexedRandom};

pub struct StatisticalMorphemGenerator<'a, R: RngExt> {
    stats: MorphemStatistics,
    lang: &'a Language,
    rng: R,
    morphem: MorphemType,
}

impl<'a> StatisticalMorphemGenerator<'a, ThreadRng> {
    pub fn new(lang: &'a Language, morphem: MorphemType) -> Self {
        Self {
            stats: lang.stats.morphem(&morphem),
            lang,
            rng: rng(),
            morphem,
        }
    }
}

impl<'a, R: RngExt> StatisticalMorphemGenerator<'a, R> {
    pub fn create(lang: &'a Language, rng: R, morphem: MorphemType) -> Self {
        Self {
            stats: lang.stats.morphem(&morphem),
            lang,
            rng,
            morphem,
        }
    }
}

impl<'a, R: RngExt + SeedableRng> StatisticalMorphemGenerator<'a, R> {
    pub fn seed(&mut self, seed: u64) {
        self.rng = R::seed_from_u64(seed)
    }
}

impl<'a, R: RngExt> Iterator for StatisticalMorphemGenerator<'a, R> {
    type Item = Morphem;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result_string = String::new();
        let len: usize = self
            .rng
            .random_range(self.stats.lenghts.start..=self.stats.lenghts.end)
            as usize;

        result_string.reserve(len);
        match self.morphem {
            MorphemType::Prefix => {
                let choose_vec: Vec<(&char, &f64)> = self.stats.weights.iter().collect();
                for _ in 0..len {
                    result_string.push(
                        *choose_vec
                            .choose_weighted(&mut self.rng, |v| v.1)
                            .unwrap()
                            .0,
                    );
                } 
            },
            MorphemType::Root => {
                let mut sylgen = SyllableGenerator::new(&self.stats, &mut self.rng, self.lang);
                let mut count = 0;
                while count < len {
                    sylgen.restrict_len((len - count) as u8);
                    let syllable = sylgen.next().unwrap();
                    result_string.push_str(&syllable);
                    count += syllable.chars().count();
                }
            }
            MorphemType::Suffix => {
                let mut sylgen = SyllableGenerator::new(&self.stats, &mut self.rng, self.lang);
                sylgen.restrict_len(len as u8);
                result_string.push_str(&sylgen.next().unwrap());
            }
            MorphemType::Ending => {
                let choose_vec: Vec<(&char, &f64)> = self.stats.weights.iter().filter(|v| self.lang.alphabet.is_vowel(*v.0)).collect();
                for _ in 0..len {
                    result_string.push(
                        *choose_vec
                            .choose_weighted(&mut self.rng, |v| v.1)
                            .unwrap()
                            .0,
                    );
                }
            }
            MorphemType::Postfix => {}
            MorphemType::All => {
                return None; // You cannot iterate over all morphems at the same time.
            }
        }
        Some(Morphem::new(result_string, self.morphem))
    }
}
