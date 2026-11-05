use crate::{morphem::Morphem, rules::Language};
use rand::{RngExt, SeedableRng, rng, rngs::ThreadRng};

pub struct StatisticalMorphemGenerator<'a, R: RngExt> {
    rules: &'a Language,
    vowels_weight: u8,
    consonants_weight: u8,
    rng: R,
}

impl<'a> StatisticalMorphemGenerator<'a, ThreadRng> {
    pub fn new(rules: &'a Language) -> Self {
        Self {
            rules,
            vowels_weight: 1,
            consonants_weight: 3,
            rng: rng(),
        }
    }
}

impl<'a, R: RngExt> StatisticalMorphemGenerator<'a, R> {
    pub fn create(rules: &'a Language, vowels_weight: u8, consonants_weight: u8, rng: R) -> Self {
        Self {
            rules,
            vowels_weight,
            consonants_weight,
            rng,
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
        todo!()
    }

}
