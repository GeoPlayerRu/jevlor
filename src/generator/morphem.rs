use crate::{
    language::statistics::MorphemStatistics,
    morphem::{Morphem, MorphemType},
};
use rand::{RngExt, SeedableRng, rng, rngs::ThreadRng, seq::IndexedRandom};

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
        let len: u8 = self
            .rng
            .random_range(self.stats.lenghts.start..=self.stats.lenghts.end);

        let mut result_string = String::new();
        result_string.reserve(len as usize);

        let weights: Vec<(char, f64)> = self.stats.weights.iter().map(|(k, v)| (*k, *v)).collect();

        for _ in 0..len {
            let Ok(chosen_char) = weights.choose_weighted(&mut self.rng, |val| val.1) else {
                break;
            };

            result_string.push(chosen_char.0);
        }

        Some(Morphem::new(result_string, self.morphem))
    }
}
