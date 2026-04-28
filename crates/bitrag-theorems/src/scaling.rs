//! Theorem B (F2 retrieval scaling law).
//!
//! Closed form
//!
//! ```text
//!     R_k(xi) = 1 - (1 - exp(-xi))^k
//! ```
//!
//! is implemented in [`recall_at_k`].  The bit-weight ratio
//! ξ = N·w / p is implemented in [`xi`].
//!
//! Both functions are pure; they take no global state and depend only
//! on their integer / float arguments.

/// Bit-weight ratio
///
/// ```text
///     xi = N * w / p
/// ```
///
/// The single parameter through which Theorem B's recall function depends
/// on the corpus parameters under assumptions (A1)–(A3).
///
/// # Panics
///
/// Panics if `p == 0`.
#[inline]
pub fn xi(n: u64, w: u64, p: u64) -> f64 {
    assert!(p > 0, "plane width p must be positive");
    (n as f64) * (w as f64) / (p as f64)
}

/// Closed-form recall@k of F2 retrieval (Theorem B).
///
/// ```text
///     R_k(xi) = 1 - (1 - exp(-xi))^k
/// ```
///
/// Implemented in the numerically-stable form
///
/// ```text
///     R_k(xi) = -expm1( k * ln_1p(-exp(-xi)) )
/// ```
///
/// which avoids catastrophic cancellation when `exp(-xi)` is tiny
/// (large `xi`) or when `xi` is near zero.
///
/// # Panics
///
/// Panics if `xi < 0.0` or `xi.is_nan()`.
#[inline]
pub fn recall_at_k(xi: f64, k: u32) -> f64 {
    assert!(
        xi.is_finite() && xi >= 0.0,
        "xi must be a non-negative finite number"
    );
    if k == 0 {
        return 0.0;
    }
    if xi == 0.0 {
        return 1.0;
    }
    let neg_eps = -(-xi).exp(); // = -exp(-xi)
    let log_one_minus = neg_eps.ln_1p(); // = ln(1 - exp(-xi)), in (-inf, 0)
    -((k as f64) * log_one_minus).exp_m1()
}

/// First-order asymptotic of `recall_at_k(xi, 1)` as `xi -> 0`:
///     R_1(xi) = 1 - xi + O(xi^2).
#[inline]
pub fn recall_1_small_xi(xi: f64) -> f64 {
    1.0 - xi
}

/// Tail asymptotic of `recall_at_k(xi, 1)` as `xi -> infty`:
///     R_1(xi) = exp(-xi).
#[inline]
pub fn recall_1_large_xi(xi: f64) -> f64 {
    (-xi).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    #[test]
    fn xi_basic() {
        // Corpus 1 from paper.md §3.2.
        let v = xi(32_768, 96, 4_194_304);
        assert!(approx(v, 0.75, 1e-9), "got {v}");
    }

    #[test]
    fn recall_at_k_zero() {
        // k = 0 always misses.
        assert_eq!(recall_at_k(0.5, 0), 0.0);
    }

    #[test]
    fn recall_at_k_zero_xi_perfect() {
        // No collisions => perfect recall.
        for k in 1..=20 {
            assert!(approx(recall_at_k(0.0, k), 1.0, 1e-12));
        }
    }

    #[test]
    fn recall_at_k_monotone_in_k() {
        let xi = 0.7;
        let mut prev = recall_at_k(xi, 1);
        for k in 2..=64 {
            let next = recall_at_k(xi, k);
            assert!(next >= prev - 1e-15, "non-monotone at k={k}");
            prev = next;
        }
    }

    /// The `Corpus 2` and `Corpus 3` rows of the table in paper.md §3.2
    /// both say `R_1 predicted = 0.779`, because they have the same
    /// xi = 0.25 by construction.
    #[test]
    fn paper_predictions_match() {
        let r2 = recall_at_k(xi(16_384, 128, 8_388_608), 1);
        let r3 = recall_at_k(xi(65_536, 64, 16_777_216), 1);
        assert!(approx(r2, r3, 1e-12), "r2={r2}, r3={r3}");
        // Paper rounds to 3 dp; we keep ±0.005 against the rounded form
        // and full precision against the closed form.
        let exact = (-0.25_f64).exp();
        assert!(approx(r2, exact, 1e-12));
        assert!(approx(r2, 0.779, 5e-3));
    }

    #[test]
    fn small_xi_asymptote_holds() {
        // For xi -> 0 we should have R_1 ~ 1 - xi to first order.
        for &xi in &[1e-3, 1e-4, 1e-5, 1e-6] {
            let r = recall_at_k(xi, 1);
            let approx_r = recall_1_small_xi(xi);
            assert!((r - approx_r).abs() < 10.0 * xi * xi, "xi={xi}");
        }
    }

    #[test]
    fn large_xi_asymptote_holds() {
        // For xi -> infty we should have R_1 ~ exp(-xi).
        for &xi in &[5.0, 10.0, 20.0, 40.0] {
            let r = recall_at_k(xi, 1);
            let approx_r = recall_1_large_xi(xi);
            // Both very small; compare as a ratio rather than absolute.
            let rel = (r - approx_r).abs() / approx_r.max(1e-300);
            assert!(rel < 1e-6, "xi={xi}, r={r}, approx={approx_r}");
        }
    }
}
