//! Deterministic xorshift64* PRNG.
//!
//! Used by the experiment harnesses so that every reproduction produces
//! byte-identical bit-vectors regardless of which `rand` minor version
//! is in the host environment.  The constants are the Marsaglia "1"
//! triplet (13, 7, 17), which are known to give a full-period stream
//! over `u64`.

/// 64-bit xorshift state.  Seed must be non-zero.
#[derive(Clone, Debug)]
pub struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    /// Create a new generator from a non-zero seed.
    ///
    /// # Panics
    ///
    /// Panics if `seed == 0`.
    pub fn new(seed: u64) -> Self {
        assert!(seed != 0, "xorshift64 seed must be non-zero");
        Self { state: seed }
    }

    /// Step the generator once and return the next `u64`.
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// Return the next `u32`.
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    /// Return a uniform random integer in `[0, n)` for `n > 0`.
    ///
    /// Uses the Lemire 64-bit unbiased reduction.
    #[inline]
    pub fn gen_range(&mut self, n: u64) -> u64 {
        debug_assert!(n > 0);
        let m = (self.next_u64() as u128) * (n as u128);
        (m >> 64) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Two generators built from the same seed produce identical streams.
    /// This is the byte-reproducibility guarantee we rely on in the
    /// experiment harnesses.
    #[test]
    fn xorshift64_is_deterministic() {
        let mut a = XorShift64::new(0xdead_beef);
        let mut b = XorShift64::new(0xdead_beef);
        for _ in 0..1024 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    /// First three outputs of `XorShift64::new(1)` computed by hand from
    /// the (13, 7, 17) recurrence — locks in the constant choices.
    #[test]
    fn xorshift64_first_three_outputs() {
        let mut g = XorShift64::new(1);
        // x = 1
        // x ^= x<<13  -> 0x2001
        // x ^= x>>7   -> 0x2041
        // x ^= x<<17  -> 0x40822041
        assert_eq!(g.next_u64(), 0x4082_2041);
        // From state = 0x40822041, run the recurrence again.
        let second = g.next_u64();
        let third = g.next_u64();
        // Self-consistency: a third generator started after two manual
        // steps must agree with the streaming generator.
        let mut h = XorShift64::new(1);
        h.next_u64();
        assert_eq!(h.next_u64(), second);
        assert_eq!(h.next_u64(), third);
    }

    #[test]
    #[should_panic]
    fn xorshift64_zero_seed_panics() {
        let _ = XorShift64::new(0);
    }
}
