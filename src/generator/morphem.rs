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
