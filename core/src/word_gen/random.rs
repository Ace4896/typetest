use super::{DisplayedWord, WordGenerator};

use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::word_pool::default::DEFAULT_WORDS;

/// A random word generator.
/// This uses a seedable RNG to allow for retries of the same test.
pub struct RandomWordGenerator {
    word_pool: Vec<String>,
    rng: SmallRng,
    rng_seed: u64,
}

impl Default for RandomWordGenerator {
    fn default() -> Self {
        RandomWordGenerator::new(DEFAULT_WORDS.iter().map(|s| s.to_string()).collect())
    }
}

impl RandomWordGenerator {
    pub fn new(word_pool: Vec<String>) -> RandomWordGenerator {
        let rng_seed = rand::thread_rng().gen::<u64>();
        let rng = SmallRng::seed_from_u64(rng_seed);

        RandomWordGenerator {
            word_pool,
            rng,
            rng_seed,
        }
    }
}

impl WordGenerator for RandomWordGenerator {
    fn is_finite(&self) -> bool {
        false
    }

    fn remaining_words(&self) -> Option<usize> {
        None
    }

    fn prepare_for_retry(&mut self) {
        self.rng = SmallRng::seed_from_u64(self.rng_seed);
    }

    fn prepare_for_next_test(&mut self) {
        self.rng_seed = rand::thread_rng().gen::<u64>();
        self.rng = SmallRng::seed_from_u64(self.rng_seed);
    }

    fn fill_words(&mut self, vec: &mut Vec<DisplayedWord>, max_chars: usize) {
        vec.clear();

        let mut chars = 0;
        loop {
            let word = &self.word_pool[self.rng.gen_range(0..self.word_pool.len())];
            chars += word.len() + !vec.is_empty() as usize;

            if chars > max_chars {
                break;
            }

            vec.push(DisplayedWord::new(word));
        }
    }
}
