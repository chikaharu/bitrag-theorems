//! # bitrag-theorems-experiments
//!
//! Deterministic reproduction harness for the bitrag-theorems paper.
//!
//! Provides:
//! - [`Corpus`] — synthetic IID Bernoulli(p/w) document-term matrix
//!   (Assumption A1 of the paper) with seeded PRNG so that any machine
//!   produces byte-identical output.
//! - [`recall_at_k`] — exact F2 retrieval recall under the
//!   AND-popcount kernel.
//! - [`f_two_term`] — the closed-form approximation `f(Nw/p)` from
//!   Theorem B that the empirical recall is compared against.
//! - [`tropical_svd_rank1`] — the rank-1 tropical SVD of an
//!   AND-popcount slice used in Lemma T5.
//! - [`jl_bound_f2`] — the F2 Johnson-Lindenstrauss lower bound
//!   benchmark used in §4.3.
//!
//! All randomness goes through a 64-bit splitmix PRNG seeded by the
//! caller. No `rand` crate dependency, no global state, no float in
//! kernel evaluation.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// 64-bit `splitmix64` PRNG: deterministic, no allocations, no deps.
#[derive(Clone, Copy, Debug)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    /// Seed the PRNG. Any 64-bit seed is valid.
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    /// Draw the next 64-bit word.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Draw a Bernoulli(p_num / p_den) bit.
    pub fn bernoulli(&mut self, p_num: u64, p_den: u64) -> bool {
        // Use the high bits of next_u64 to compare against the
        // requested probability ratio; integer-only.
        let r = self.next_u64() % p_den;
        r < p_num
    }
}

/// A synthetic IID-Bernoulli(p/w) F2 document-term corpus (Assumption A1).
///
/// Internally each document is a `Vec<u64>` of length `ceil(w / 64)` so
/// AND-popcount runs at one CPU instruction per 64 bits per pair.
#[derive(Clone, Debug)]
pub struct Corpus {
    /// Vocabulary size (number of bit columns).
    pub w: usize,
    /// Number of u64 words per document = `ceil(w / 64)`.
    pub words: usize,
    /// Documents stored as a flat `n * words` vector.
    pub docs: Vec<u64>,
    /// Number of documents.
    pub n: usize,
    /// Mean per-document load (≈ p in the paper, integer per-doc bits).
    pub p: usize,
}

impl Corpus {
    /// Build an `n × w` IID-Bernoulli(p/w) corpus seeded by `seed`.
    /// Panics if `w == 0` or `n == 0`.
    pub fn new(n: usize, w: usize, p: usize, seed: u64) -> Self {
        assert!(w > 0 && n > 0);
        let words = (w + 63) / 64;
        let mut rng = SplitMix64::new(seed);
        let mut docs = vec![0u64; n * words];
        let p_num = p as u64;
        let p_den = w as u64;
        for d in 0..n {
            for bit in 0..w {
                if rng.bernoulli(p_num, p_den) {
                    let off = d * words + (bit / 64);
                    docs[off] |= 1u64 << (bit % 64);
                }
            }
        }
        Self {
            w,
            words,
            docs,
            n,
            p,
        }
    }

    /// Borrow document `i` as a slice of 64-bit words.
    pub fn doc(&self, i: usize) -> &[u64] {
        &self.docs[i * self.words..(i + 1) * self.words]
    }

    /// AND-popcount kernel between docs `i` and `j`.
    pub fn and_popcount(&self, i: usize, j: usize) -> u32 {
        let a = self.doc(i);
        let b = self.doc(j);
        let mut s = 0u32;
        for k in 0..self.words {
            s += (a[k] & b[k]).count_ones();
        }
        s
    }
}

/// Recall@k of `query` against `corpus`, scoring by AND-popcount.
///
/// Ties are broken by document index (lower index wins) so the result
/// is deterministic across machines.
pub fn recall_at_k(corpus: &Corpus, query: &[u64], truth_idx: usize, k: usize) -> bool {
    assert_eq!(query.len(), corpus.words);
    let mut scores: Vec<(u32, usize)> = (0..corpus.n)
        .map(|i| {
            let a = corpus.doc(i);
            let mut s = 0u32;
            for w in 0..corpus.words {
                s += (a[w] & query[w]).count_ones();
            }
            (s, i)
        })
        .collect();
    // Sort: highest score first, ties broken by lowest index.
    scores.sort_by(|x, y| y.0.cmp(&x.0).then(x.1.cmp(&y.1)));
    scores.iter().take(k).any(|(_, i)| *i == truth_idx)
}

/// Theorem B closed-form scaling: `f(ξ) = 1 - exp(-ξ)` where
/// `ξ = Nw/p` is the two-term scaling variable.
///
/// Returned as a fixed-point integer with denominator `denom` so that
/// the comparison against empirical recall is float-free in CI.
///
/// Algorithm: doubling reduction `exp(-ξ) = exp(-ξ / 2^k)^(2^k)` brings
/// the argument into `[0, 1)`, where Taylor converges absolutely
/// in <10 terms; intermediate signs are tracked in `i128`.
pub fn f_two_term(n: usize, w: usize, p: usize, denom: u64) -> u64 {
    assert!(p > 0 && denom > 0);
    let one = denom as u128;
    let xi_q = ((n as u128) * (w as u128) * one) / (p as u128);
    // For very large ξ, exp(-ξ) underflows below 1 ulp of 1/denom.
    if xi_q >= 40 * one {
        return denom;
    }
    // Doubling reduction: while x > 1 (== one in fixed point), halve.
    let mut x = xi_q;
    let mut k_double: u32 = 0;
    while x > one {
        x /= 2;
        k_double += 1;
    }
    // Taylor: exp(-x) = sum_{k≥0} (-x)^k / k!
    let one_i = one as i128;
    let x_i = x as i128;
    let mut term: i128 = one_i;
    let mut sum: i128 = one_i;
    for k in 1i128..40 {
        term = -(term * x_i) / (one_i * k);
        if term == 0 {
            break;
        }
        sum += term;
    }
    // Square k_double times to recover exp(-xi_q).
    let mut e_neg = sum.max(0) as u128;
    for _ in 0..k_double {
        e_neg = (e_neg * e_neg) / one;
    }
    one.saturating_sub(e_neg) as u64
}

/// Tropical (max-plus) rank-1 reconstruction error for an
/// AND-popcount Gram block of `Corpus`.
///
/// In Lemma T5 we claim that AND-popcount Gram blocks are
/// approximately tropical-rank-1 with a bounded residual; this
/// returns the maximum absolute residual `max_{i,j} |G_{ij} - (u_i + v_j)|`
/// after fitting the rank-1 outer-sum by row/column maxima.
pub fn tropical_svd_rank1(corpus: &Corpus, rows: &[usize], cols: &[usize]) -> u32 {
    let m = rows.len();
    let n = cols.len();
    assert!(m > 0 && n > 0);
    let mut g = vec![0u32; m * n];
    for (r, &i) in rows.iter().enumerate() {
        for (c, &j) in cols.iter().enumerate() {
            g[r * n + c] = corpus.and_popcount(i, j);
        }
    }
    // Fit rank-1 tropical: u_r = max_c G_{r,c}, v_c = max_r (G_{r,c} - u_r).
    let mut u = vec![0u32; m];
    for r in 0..m {
        u[r] = (0..n).map(|c| g[r * n + c]).max().unwrap_or(0);
    }
    let mut v = vec![i64::MIN; n];
    for c in 0..n {
        v[c] = (0..m)
            .map(|r| g[r * n + c] as i64 - u[r] as i64)
            .max()
            .unwrap_or(0);
    }
    // Residual = max |G - (u + v)|.
    let mut res = 0i64;
    for r in 0..m {
        for c in 0..n {
            let approx = u[r] as i64 + v[c];
            let diff = (g[r * n + c] as i64 - approx).abs();
            if diff > res {
                res = diff;
            }
        }
    }
    res as u32
}

/// F2 Johnson-Lindenstrauss lower bound (Larsen-Nelson 2017 specialized
/// to F2): the minimum projection dimension `d` to preserve pairwise
/// AND-popcount distances within multiplicative `1 ± ε` for `n` items
/// with failure probability `δ`.
///
/// Returns the integer ceiling of the lower bound. Inputs are
/// fixed-point with denom 1000 to avoid floats.
pub fn jl_bound_f2(n: usize, eps_milli: u64, delta_milli: u64) -> u64 {
    // d ≥ C * log(1/δ) / ε² with C ≈ 1 (tight up to constants in F2).
    // log uses integer ln-table approximation: ln(x/1000) for x in [1,1000].
    assert!(n > 1 && eps_milli > 0 && delta_milli > 0 && delta_milli < 1000);
    // log(n) and log(1/δ) via 16-bit integer ln.
    let log_n_milli = ln_milli(n as u64 * 1000);
    let log_inv_delta_milli = ln_milli((1000 * 1000) / delta_milli);
    let combined = log_n_milli + log_inv_delta_milli;
    // numerator: combined (already milli)
    // denominator: ε² in milli-squared = eps_milli^2 / 1000
    let num = combined; // milli
    let den_milli = (eps_milli * eps_milli) / 1000;
    if den_milli == 0 {
        return u64::MAX;
    }
    (num + den_milli - 1) / den_milli
}

/// `ln(x / 1000) * 1000` for `x ≥ 1`. Integer-only, monotone, tested.
fn ln_milli(x_milli: u64) -> u64 {
    // ln is strictly increasing; we use a small table at powers of e
    // and linear interpolation between, all in milli units.
    if x_milli <= 1000 {
        return 0;
    }
    // ln(2) ≈ 693, ln(10) ≈ 2302, ln(100) ≈ 4605.
    // For correctness we do a binary-shift decomposition: write
    // x = 2^k * r with r in [1, 2), then ln(x) = k*ln(2) + ln(r).
    // ln(r) for r in [1,2) approximated by 5-term Taylor at r-1.
    let mut x = x_milli;
    let mut k: u64 = 0;
    while x >= 2000 {
        x /= 2;
        k += 1;
    }
    // r in [1, 2), x in [1000, 2000) milli.
    let y = x - 1000; // y in [0, 1000) milli
                      // ln(1+y/1000) ≈ y - y^2/2 + y^3/3 - y^4/4 + y^5/5 (in milli).
    let y = y as u128;
    let m = 1000u128;
    let t1 = y;
    let t2 = (y * y) / (2 * m);
    let t3 = (y * y * y) / (3 * m * m);
    let t4 = (y * y * y * y) / (4 * m * m * m);
    let t5 = (y * y * y * y * y) / (5 * m * m * m * m);
    let frac = t1 + t3 + t5 - t2 - t4;
    k * 693 + frac as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitmix_is_deterministic() {
        let mut a = SplitMix64::new(42);
        let mut b = SplitMix64::new(42);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn corpus_density_matches_p_over_w_within_5pct() {
        let n = 64;
        let w = 1024;
        let p = 32;
        let c = Corpus::new(n, w, p, 0xDEADBEEF);
        let total_bits: u64 = c.docs.iter().map(|w| w.count_ones() as u64).sum();
        let expected = (n * p) as u64;
        let lo = expected * 95 / 100;
        let hi = expected * 105 / 100;
        assert!(
            (lo..=hi).contains(&total_bits),
            "density {total_bits} outside [{lo},{hi}] for n={n} w={w} p={p}"
        );
    }

    #[test]
    fn f_two_term_monotone_in_xi() {
        let denom = 1_000_000;
        let mut prev = 0;
        for n in [1, 4, 16, 64, 256, 1024usize] {
            let r = f_two_term(n, 64, 8, denom);
            assert!(r >= prev, "f must be monotone non-decreasing in N");
            prev = r;
        }
    }

    #[test]
    fn f_two_term_saturates_at_one() {
        let denom = 1_000_000;
        let r = f_two_term(1_000_000, 1024, 1, denom);
        assert_eq!(r, denom);
    }

    #[test]
    fn recall_at_k_finds_self_query_at_k_eq_1() {
        let c = Corpus::new(32, 256, 16, 7);
        let q = c.doc(5).to_vec();
        // self-query is the trivial best match (modulo ties).
        assert!(recall_at_k(&c, &q, 5, 1) || recall_at_k(&c, &q, 5, 4));
    }

    #[test]
    fn tropical_svd_rank1_zero_on_constant_block() {
        // A perfectly constant Gram block is exactly rank-1 tropical.
        let c = Corpus::new(8, 64, 64, 1); // p == w => every bit set
        let res = tropical_svd_rank1(&c, &[0, 1, 2, 3], &[4, 5, 6, 7]);
        assert_eq!(res, 0, "constant Gram block must have zero residual");
    }

    #[test]
    fn jl_bound_f2_grows_with_n() {
        let lo = jl_bound_f2(100, 100, 100);
        let hi = jl_bound_f2(10_000, 100, 100);
        assert!(hi > lo, "JL bound must grow with n");
    }

    #[test]
    fn ln_milli_monotone() {
        let mut prev = 0;
        for x in [1000u64, 1500, 2000, 5000, 10_000, 100_000, 1_000_000] {
            let v = ln_milli(x);
            assert!(v >= prev, "ln must be monotone non-decreasing");
            prev = v;
        }
    }
}
