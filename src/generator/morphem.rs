use crate::{
    language::statistics::MorphemStatistics,
    morphem::{Morphem, MorphemType},
};
use rand::{RngExt, SeedableRng, rng, rngs::ThreadRng, seq::IndexedRandom};

/// Iterator that returns random syllables
struct SyllableGenerator<'a, 'b, R: RngExt> {
    stats: &'a MorphemStatistics<'b>,
    rng: &'a mut R,
}

impl<'a, 'b, R: RngExt> SyllableGenerator<'a, 'b, R> {
    /// Create new syllable generator out of references to stats and random number generator
    pub fn new(stats: &'a MorphemStatistics<'b>, rng: &'a mut R) -> Self {
        Self { stats, rng }
    }
}

impl<'a, 'b, R: RngExt> Iterator for SyllableGenerator<'a, 'b, R> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut result = String::new();
        result.reserve_exact(3);

        let weights: Vec<(char, f64)> = self.stats.weights.iter().map(|(k, v)| (*k, *v)).collect();
        let vowel_weights: Vec<(char, f64)> = weights
            .iter()
            .filter_map(|(ch, v)| {
                self.stats
                    .lang
                    .unwrap()
                    .alphabet
                    .is_vowel(*ch)
                    .then(|| (*ch, *v))
            })
            .collect();

        for i in 0..3 {
            let ch = if i % 2 == 0 {
                weights.choose_weighted(&mut self.rng, |v| v.1)
            } else {
                vowel_weights.choose_weighted(&mut self.rng, |v| v.1)
            }.unwrap();

            result.push(ch.0);

            if self.stats.lang.unwrap().alphabet.is_vowel(ch.0) || self.rng.random_bool(0.5) {
                break;
            }
        }

        result.shrink_to_fit();
        Some(result)
    }
}

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
            .random_range(self.stats.lenghts.start..=self.stats.lenghts.end) as usize;

        let mut result_string = String::new();
        result_string.reserve(len as usize);

        let mut sylgen = SyllableGenerator::new(&self.stats, &mut self.rng);
        while result_string.len() < len {
            result_string.push_str(&sylgen.next().unwrap());
        }

        Some(Morphem::new(result_string, self.morphem))
    }
}
