use rand::{
    prelude::{SliceRandom, SmallRng},
    Rng, SeedableRng,
};

use crate::word_pools::default_english::DEFAULT_ENGLISH;

use super::{TestWord, WordGenerator};

/// Implementation of an infinite word generator using random words from a pool.
pub struct InfiniteWordGenerator {
    word_pool: Vec<String>,
    rng: SmallRng,
    rng_seed: u64,
}

impl InfiniteWordGenerator {
    pub fn new(word_pool: Vec<String>) -> Self {
        let rng_seed = rand::thread_rng().gen::<u64>();
        let rng = SmallRng::seed_from_u64(rng_seed);

        Self {
            word_pool,
            rng,
            rng_seed,
        }
    }
}

impl Default for InfiniteWordGenerator {
    fn default() -> Self {
        Self::new(DEFAULT_ENGLISH.iter().map(|s| s.to_string()).collect())
    }
}

impl WordGenerator for InfiniteWordGenerator {
    fn fill_line(&mut self, line: &mut Vec<super::TestWord>, max_chars: usize) {
        line.clear();

        let mut chars = 0;
        let mut word = self
            .word_pool
            .choose(&mut self.rng)
            .expect("Word pool is empty!");
        chars += word.len();

        while chars < max_chars {
            line.push(TestWord::new(word));

            // NOTE: +1 to length due to spacebar
            word = self
                .word_pool
                .choose(&mut self.rng)
                .expect("Word pool is empty!");
            chars += word.len() + 1;
        }
    }

    fn redo(&mut self) {
        self.rng = SmallRng::seed_from_u64(self.rng_seed);
    }

    fn next_test(&mut self) {
        self.rng_seed = rand::thread_rng().gen::<u64>();
        self.rng = SmallRng::seed_from_u64(self.rng_seed);
    }
}
