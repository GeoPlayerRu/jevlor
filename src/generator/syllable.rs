use crate::language::statistics::MorphemStatistics;
use rand::{RngExt, seq::IndexedRandom};
use std::collections::VecDeque;

const NEXT_CONSONANT_PROBABILITY: f64 = 0.5;
const MAX_CONSONANTS_PER_SIDE: u8 = 4;

/// Iterator that returns random syllables
pub struct SyllableGenerator<'a, 'b, R: RngExt> {
    stats: &'a MorphemStatistics<'b>,
    rng: &'a mut R,
    restricted_len: Option<u8>,
}

impl<'a, 'b, R: RngExt> SyllableGenerator<'a, 'b, R> {
    /// Create new syllable generator out of references to stats and random number generator
    pub fn new(stats: &'a MorphemStatistics<'b>, rng: &'a mut R) -> Self {
        Self {
            stats,
            rng,
            restricted_len: None,
        }
    }
    /// Restrict len to [max_len] symbols
    pub fn restrict_len(&mut self, max_len: u8) {
        self.restricted_len = Some(max_len);
    }
    /// Unrestrict len
    pub fn unrestrict_len(&mut self) {
        self.restricted_len = None;
    }
}

impl<'a, 'b, R: RngExt> Iterator for SyllableGenerator<'a, 'b, R> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        // Create new deq with vowel at center
        let mut syl = {
            let vowel_weights: Vec<(char, f64)> = self
                .stats
                .weights
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

            let mut result_dec = VecDeque::new();
            result_dec.push_front(
                vowel_weights
                    .choose_weighted(&mut self.rng, |v| v.1)
                    .unwrap()
                    .0,
            );
            result_dec
        };

        let mut len: u8 = 1;

        let consonant_weights: Vec<(char, f64)> = self
            .stats
            .weights
            .iter()
            .filter_map(|(k, v)| {
                self.stats
                    .lang
                    .unwrap()
                    .alphabet
                    .is_consonant(*k)
                    .then(|| (*k, *v))
            })
            .collect();

        // Add consonants at the beginning
        let mut p = NEXT_CONSONANT_PROBABILITY;
        for _ in 0..MAX_CONSONANTS_PER_SIDE {
            if self.rng.random_bool(1.0 - p) {
                break;
            }
            if let Some(rest_len) = self.restricted_len
                && rest_len == len
            {
                break;
            }
            syl.push_back(
                consonant_weights
                    .choose_weighted(&mut self.rng, |v| v.1)
                    .unwrap()
                    .0,
            );

            p *= NEXT_CONSONANT_PROBABILITY;
            len += 1;
        }

        // Add syllables at the end
        let mut p = NEXT_CONSONANT_PROBABILITY;
        for _ in 0..MAX_CONSONANTS_PER_SIDE {
            if self.rng.random_bool(1.0 - p) {
                break;
            }
            if let Some(rest_len) = self.restricted_len
                && rest_len == len
            {
                break;
            }
            syl.push_front(
                consonant_weights
                    .choose_weighted(&mut self.rng, |v| v.1)
                    .unwrap()
                    .0,
            );

            p *= NEXT_CONSONANT_PROBABILITY;
            len += 1;
        }

        Some(syl.into_iter().collect())
    }
}
