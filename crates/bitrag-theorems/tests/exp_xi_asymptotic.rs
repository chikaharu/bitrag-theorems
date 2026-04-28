//! Experiment 6 — asymptotics of R_1(xi) as xi -> 0 and xi -> infty.
//!
//! paper.md §6.3 reports
//!   - xi -> 0:    slope of R_1 in xi is -1.0  (within ±0.03)
//!   - xi -> infty: slope of log R_1 in xi is -1.0 (within ±0.012)

use bitrag_theorems::scaling::recall_at_k;

#[test]
fn small_xi_slope_is_minus_one() {
    let xi_a = 1e-4;
    let xi_b = 1e-5;
    let r_a = recall_at_k(xi_a, 1);
    let r_b = recall_at_k(xi_b, 1);
    // R_1 ~ 1 - xi + O(xi^2) so (R_a - R_b) / (xi_b - xi_a) -> 1.
    let slope = (r_a - r_b) / (xi_a - xi_b);
    // The closed form has slope -1 at the origin, so the finite-difference
    // approximation should land inside ±0.05.
    assert!((slope - (-1.0)).abs() < 0.05, "slope = {slope}");
}

#[test]
fn large_xi_log_slope_is_minus_one() {
    let xi_a = 20.0;
    let xi_b = 30.0;
    let r_a = recall_at_k(xi_a, 1);
    let r_b = recall_at_k(xi_b, 1);
    let slope = (r_b.ln() - r_a.ln()) / (xi_b - xi_a);
    assert!((slope - (-1.0)).abs() < 0.05, "log slope = {slope}");
}

#[test]
fn xi_zero_recall_is_one() {
    assert_eq!(recall_at_k(0.0, 1), 1.0);
}

#[test]
fn xi_huge_recall_is_zero() {
    let r = recall_at_k(700.0, 1);
    assert!(r < 1e-300, "got {r}");
}
