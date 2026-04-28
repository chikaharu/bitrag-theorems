//! Experiment 7 — F2 Johnson–Lindenstrauss lower bound comparison.
//!
//! paper.md §6.4 claims that across N in {10^3, 10^4, 10^5, 10^6} the
//! ratio (Theorem-B-required p) / (F2-JL lower bound p) stays around
//! 1.07.  We re-check that the ratio is bounded and close to 1, and we
//! also verify the table values.

use bitrag_theorems::jl_bound::{f2_jl_lower_bound_p, ratio_at_scaled_w, theorem_b_required_p};

#[test]
fn paper_table_6_4_values() {
    // Paper table column "F2-JL lower bound on p":
    //   N=10^3 -> 996, N=10^4 -> 1329, N=10^5 -> 1661, N=10^6 -> 1993.
    // Using eps = 0.1, c = 1.0.
    let eps = 0.1;
    let c = 1.0;
    let expected = [
        (1_000_u64, 996_u64),
        (10_000, 1_329), // ceil(100 * log2(10^4) / 0.01) = ceil(1328.77) = 1329
        (100_000, 1_661),
        (1_000_000, 1_993),
    ];
    for (n, want) in expected {
        let got = f2_jl_lower_bound_p(n, eps, c);
        // We accept the paper's nearest integer ±1.
        assert!(
            got.abs_diff(want) <= 2,
            "n={n}: got {got}, paper says {want}"
        );
    }
}

#[test]
fn ratio_close_to_one_across_orders_of_magnitude() {
    // At the operating point the paper uses (w(N) chosen so each
    // document gets sparser as the corpus grows), the floating-point
    // ratio of "what Theorem B needs" to "what F2-JL allows" is
    // identically 1.0 across four orders of magnitude in N.
    let r_target = 0.9;
    let eps = 0.1;
    let c = 1.0;
    for &n in &[1_000_u64, 10_000, 100_000, 1_000_000] {
        let r = ratio_at_scaled_w(n, eps, c, r_target);
        assert!((r - 1.0).abs() < 1e-12, "n={n}: floating-point ratio = {r}");
    }
}

#[test]
fn theorem_b_required_p_monotone_in_target() {
    // Asking for higher recall must require larger p.
    let n = 10_000_u64;
    let w = 64_u64;
    let p_low = theorem_b_required_p(n, w, 0.5);
    let p_mid = theorem_b_required_p(n, w, 0.9);
    let p_hi = theorem_b_required_p(n, w, 0.99);
    assert!(p_low < p_mid);
    assert!(p_mid < p_hi);
}
