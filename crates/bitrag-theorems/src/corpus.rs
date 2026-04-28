//! Synthetic corpus generation and empirical recall@k measurement.
//!
//! Used by the experiment harnesses (`tests/exp*.rs`) to confirm that
//! the closed-form Theorem B prediction matches measurement on a
//! deterministic synthetic corpus.  All randomness flows through
//! [`crate::prng::XorShift64`] so reproductions are byte-identical.

use crate::prng::XorShift64;

/// A bit-vector with `p` bits.  Stored as a `Vec<u64>` of `p / 64`
/// limbs (big-endian within each limb does not matter).
#[derive(Clone, Debug)]
pub struct BitVec {
    p: usize,
    limbs: Vec<u64>,
}

impl BitVec {
    /// Allocate an all-zero bit-vector of length `p`.
    ///
    /// # Panics
    ///
    /// Panics if `p == 0` or `p % 64 != 0`.
    pub fn zeros(p: usize) -> Self {
        assert!(p > 0 && p % 64 == 0, "p must be a positive multiple of 64");
        Self {
            p,
            limbs: vec![0u64; p / 64],
        }
    }

    /// Set `bit_idx` to 1.
    ///
    /// # Panics
    ///
    /// Panics if `bit_idx >= p`.
    pub fn set(&mut self, bit_idx: usize) {
        assert!(bit_idx < self.p, "bit out of range");
        let limb = bit_idx / 64;
        let off = bit_idx % 64;
        self.limbs[limb] |= 1u64 << off;
    }

    /// Read `bit_idx` (returns `true` if set).
    ///
    /// # Panics
    ///
    /// Panics if `bit_idx >= p`.
    pub fn get(&self, bit_idx: usize) -> bool {
        assert!(bit_idx < self.p, "bit out of range");
        let limb = bit_idx / 64;
        let off = bit_idx % 64;
        (self.limbs[limb] >> off) & 1 == 1
    }

    /// `popcount(self & other)`.
    ///
    /// # Panics
    ///
    /// Panics if `self.p != other.p`.
    pub fn and_popcount(&self, other: &BitVec) -> u32 {
        assert_eq!(self.p, other.p);
        self.limbs
            .iter()
            .zip(other.limbs.iter())
            .map(|(a, b)| (a & b).count_ones())
            .sum()
    }

    /// In-place bit-wise OR (`self |= other`).
    ///
    /// # Panics
    ///
    /// Panics if `self.p != other.p`.
    pub fn or_assign(&mut self, other: &BitVec) {
        assert_eq!(self.p, other.p);
        for (a, b) in self.limbs.iter_mut().zip(other.limbs.iter()) {
            *a |= *b;
        }
    }

    /// Plane width in bits.
    #[inline]
    pub fn len(&self) -> usize {
        self.p
    }

    /// `true` if every bit is zero.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.limbs.iter().all(|&l| l == 0)
    }
}

/// Sample a uniform random F2-encoded "document" of plane width `p` and
/// expected weight `w`.  Each bit is set independently with probability
/// `w / p` — this is exactly the (A1) + (A2) hypothesis of Theorem B.
pub fn sample_doc(rng: &mut XorShift64, p: usize, w: u64) -> BitVec {
    let mut bv = BitVec::zeros(p);
    let threshold = ((w as u128) << 64) / (p as u128);
    let threshold = threshold.min(u64::MAX as u128) as u64;
    for i in 0..p {
        if rng.next_u64() < threshold {
            bv.set(i);
        }
    }
    bv
}

/// Build a synthetic corpus and a matching set of queries, then
/// measure empirical recall@k for the F2 retrieval kernel.
///
/// Returns the fraction of queries for which the relevant document
/// appears in the top-`k` of the score-sorted list.
///
/// The "relevant" document for query `i` is the corpus document at
/// index `i` perturbed by independent noise of expected weight
/// `noise_w`.
pub fn measure_recall_at_k(
    n: u64,
    w: u64,
    p: u64,
    n_queries: u64,
    k: u32,
    noise_w: u64,
    seed: u64,
) -> f64 {
    let p_us = p as usize;
    let mut rng = XorShift64::new(seed);

    // Sample the corpus.
    let mut corpus: Vec<BitVec> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        corpus.push(sample_doc(&mut rng, p_us, w));
    }

    // For the first `n_queries` documents, build a perturbed query and
    // count whether the right document appears in top-k.
    let mut hits: u64 = 0;
    let nq = n_queries.min(n) as usize;
    for q_idx in 0..nq {
        // Query = corpus[q_idx] OR sample_doc(noise_w).
        let noise = sample_doc(&mut rng, p_us, noise_w);
        let mut query = corpus[q_idx].clone();
        for limb in 0..query.limbs.len() {
            query.limbs[limb] |= noise.limbs[limb];
        }

        // Score every doc in the corpus.
        let true_score = query.and_popcount(&corpus[q_idx]);
        // Count how many docs strictly beat the true doc.
        let mut better: u32 = 0;
        for (j, doc) in corpus.iter().enumerate() {
            if j == q_idx {
                continue;
            }
            let s = query.and_popcount(doc);
            if s > true_score {
                better += 1;
                if better >= k {
                    break;
                }
            }
        }
        if better < k {
            hits += 1;
        }
    }

    hits as f64 / nq as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_corpus_recall_is_one() {
        // A 1-doc corpus with the same query → recall = 1.
        let r = measure_recall_at_k(1, 16, 64, 1, 1, 0, 7);
        assert_eq!(r, 1.0);
    }

    #[test]
    fn deterministic_across_runs() {
        let a = measure_recall_at_k(64, 8, 128, 32, 3, 2, 12345);
        let b = measure_recall_at_k(64, 8, 128, 32, 3, 2, 12345);
        assert_eq!(a, b);
    }

    #[test]
    fn andpopcount_zero_for_zero_bv() {
        let a = BitVec::zeros(128);
        let b = BitVec::zeros(128);
        assert_eq!(a.and_popcount(&b), 0);
    }

    #[test]
    fn andpopcount_reflexive() {
        let mut bv = BitVec::zeros(128);
        for &i in &[3_usize, 17, 64, 100, 127] {
            bv.set(i);
        }
        assert_eq!(bv.and_popcount(&bv), 5);
    }
}
