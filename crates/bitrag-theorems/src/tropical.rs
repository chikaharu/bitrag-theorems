//! Lemma T5 — structural rank upper bound of the tropical AND-popcount
//! score matrix.
//!
//! Bound (paper.md §4):
//!
//! ```text
//!     k_max(N, w, p) = floor( p / max(xi, 1) ) + 1,   xi = N*w/p
//! ```
//!
//! A small reference implementation of (max, +) rank-k reconstruction is
//! provided so that empirical witnesses can be checked against the bound.

use crate::scaling::xi;

/// Lemma T5 structural rank upper bound.
///
/// # Panics
///
/// Panics if `p == 0`.
#[inline]
pub fn k_max(n: u64, w: u64, p: u64) -> u64 {
    let x = xi(n, w, p);
    let denom = if x < 1.0 { 1.0 } else { x };
    ((p as f64) / denom).floor() as u64 + 1
}

/// Reconstruct an `m × n` matrix from a rank-`k` (max, +) factorisation
/// `S_ij = max_l (U_il + V_jl)`.
///
/// `u` is row-major `m × k`; `v` is row-major `n × k`.  Returned matrix
/// is row-major `m × n`.
pub fn tropical_reconstruct(u: &[i64], v: &[i64], m: usize, n: usize, k: usize) -> Vec<i64> {
    assert_eq!(u.len(), m * k);
    assert_eq!(v.len(), n * k);
    let mut s = vec![i64::MIN; m * n];
    for i in 0..m {
        for j in 0..n {
            let mut best = i64::MIN;
            for l in 0..k {
                let cand = u[i * k + l].saturating_add(v[j * k + l]);
                if cand > best {
                    best = cand;
                }
            }
            s[i * n + j] = best;
        }
    }
    s
}

/// Decide whether `s` (row-major `m × n`) admits an exact rank-`k`
/// (max, +) factorisation that we already know.  Returns the
/// reconstruction error sup-norm.
pub fn tropical_residual(s: &[i64], u: &[i64], v: &[i64], m: usize, n: usize, k: usize) -> i64 {
    let recon = tropical_reconstruct(u, v, m, n, k);
    s.iter()
        .zip(recon.iter())
        .map(|(a, b)| (a - b).abs())
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn k_max_corpus_1() {
        // Corpus 1: N=32768, w=96, p=4_194_304 -> xi=0.75
        // xi < 1 so denom clamps to 1.0; p / 1 = 4_194_304 -> +1 = 4_194_305
        assert_eq!(k_max(32_768, 96, 4_194_304), 4_194_305);
    }

    #[test]
    fn k_max_small_xi_clamps_to_p_plus_1() {
        // When xi < 1 the bound should reduce to floor(p) + 1 = p + 1.
        assert_eq!(k_max(1, 1, 1024), 1024 + 1);
    }

    #[test]
    fn k_max_grows_with_p() {
        let a = k_max(1_000, 100, 1_024);
        let b = k_max(1_000, 100, 4_096);
        assert!(b >= a);
    }

    #[test]
    fn tropical_reconstruct_roundtrip() {
        // Hand-build a rank-2 (max,+) matrix and check round-trip.
        let u: Vec<i64> = vec![1, 0, 2, 3, 0, 1]; // 3×2
        let v: Vec<i64> = vec![0, 1, 4, 0]; // 2×2
        let s = tropical_reconstruct(&u, &v, 3, 2, 2);
        // Row 0: max(1+0, 0+1)=1; max(1+4, 0+0)=5
        // Row 1: max(2+0, 3+1)=4; max(2+4, 3+0)=6
        // Row 2: max(0+0, 1+1)=2; max(0+4, 1+0)=4
        assert_eq!(s, vec![1, 5, 4, 6, 2, 4]);
        // And the residual against itself is 0.
        assert_eq!(tropical_residual(&s, &u, &v, 3, 2, 2), 0);
    }
}
