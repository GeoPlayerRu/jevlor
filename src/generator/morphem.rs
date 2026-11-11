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
use crate::{
    language::statistics::MorphemStatistics,
    morphem::{Morphem, MorphemType},
};
use rand::{RngExt, SeedableRng, rng, rngs::ThreadRng};

pub struct StatisticalMorphemGenerator<'a, R: RngExt> {
    stats: MorphemStatistics<'a>,
    rng: R,
    morphem: MorphemType,
}

impl<'a> StatisticalMorphemGenerator<'a, ThreadRng> {
    pub fn new(stats: MorphemStatistics<'a>, morphem: MorphemType) -> Self {
        Self {
            stats,
            rng: rng(),
            morphem,
        }
    }
}

impl<'a, R: RngExt> StatisticalMorphemGenerator<'a, R> {
    pub fn create(stats: MorphemStatistics<'a>, rng: R, morphem: MorphemType) -> Self {
        Self {
            stats,
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
        let len: usize = self
            .rng
            .random_range(self.stats.lenghts.start..=self.stats.lenghts.end)
            as usize;

        let mut result_string = String::new();
        result_string.reserve(len as usize);

        let mut sylgen = SyllableGenerator::new(&self.stats, &mut self.rng);
        sylgen.restrict_len((len - result_string.len()) as u8);
        while result_string.len() < len {
            result_string.push_str(&sylgen.next().unwrap());
        }

        Some(Morphem::new(result_string, self.morphem))
    }
}
