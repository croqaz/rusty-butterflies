//
// From: https://github.com/ondras/rot.js/blob/master/src/rng.ts
// From: https://github.com/macmcmeans/aleaPRNG/blob/master/aleaPRNG-1.1.js
//

use std::cmp;

static FRAC: f64 = 2.3283064365386963e-10;

#[derive(Default)]
pub struct Rng {
    seed: f64,
    s0: f64,
    s1: f64,
    s2: f64,
    c: i64,
}

#[allow(dead_code)]
impl Rng {
    /// Create a random number generator from specified seed
    ///
    pub fn from_seed(seed: i64) -> Self {
        let mut rng = Rng::default();

        let seed: f64 = if seed < 1 {
            1.0 / seed as f64
        } else {
            seed as f64
        };

        rng.seed = seed;
        rng.s0 = (seed as i64) as f64 * FRAC;

        let seed: f64 = seed * 69069.0 + 1.0;
        rng.s1 = seed * FRAC;

        let seed: f64 = seed * 69069.0 + 1.0;
        rng.s2 = seed * FRAC;

        rng.c = 1;

        // run once to spin the state
        rng.get_uniform();
        rng
    }

    /// Pseudorandom a 32-bit fraction [0,1), uniformly distributed
    ///
    pub fn get_uniform(&mut self) -> f64 {
        let t = 2091639.0 * self.s0 + self.c as f64 * FRAC;
        self.s0 = self.s1;
        self.s1 = self.s2;
        self.c = t as i64 | 0;
        self.s2 = t - self.c as f64;
        self.s2
    }

    /// Pseudorandom value [lower,upper], using get_uniform() to distribute the value
    ///
    pub fn get_uniform_int(&mut self, lower_bound: i64, upper_bound: i64) -> i32 {
        let max = cmp::max(lower_bound, upper_bound);
        let min = cmp::min(lower_bound, upper_bound);
        let mm = (max - min + 1) as f64;
        let uni = min as f64 + (self.get_uniform() * mm);
        uni.floor() as i32
    }

    /// Pseudorandom value [1,100], uniformly distributed
    ///
    pub fn get_percentage(&mut self) -> u8 {
        1 + (self.get_uniform() * 100.0).floor() as u8
    }
}
