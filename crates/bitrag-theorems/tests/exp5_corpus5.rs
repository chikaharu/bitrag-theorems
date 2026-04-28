//! Experiment 5 — Corpus 5 (BEIR-NFCorpus, held-out, small-xi).
//!
//! paper.md §6.2: N = 3_633, w = 78, p = 8_192_000 -> xi ≈ 0.0346
//! Predicted R_1 = 0.966; measured R_1 = 0.958 ± 0.006.

use bitrag_theorems::corpus::measure_recall_at_k;
use bitrag_theorems::scaling::{recall_at_k, xi};

#[test]
fn corpus_5_predicted_recall_matches_paper() {
    let r = recall_at_k(xi(3_633, 78, 8_192_000), 1);
    let paper_value = 0.966;
    assert!(
        (r - paper_value).abs() < 5e-3,
        "predicted R_1 = {r}, paper = {paper_value}"
    );
}

/// Small-xi regime: synthetic corpus with small xi must show
/// near-perfect recall (qualitative reproduction of paper §6.2).
#[test]
fn corpus_5_synthetic_recall_high() {
    // Pick (N, w, p) with very small xi (≈ 0.05).
    let n: u64 = 64;
    let w: u64 = 4;
    let p: u64 = 8_192;
    let xi_small = xi(n, w, p);
    assert!(xi_small < 0.1, "xi too large to be 'small': {xi_small}");

    let predicted = recall_at_k(xi_small, 1);
    assert!(
        predicted > 0.95,
        "predicted R_1 should be >0.95, got {predicted}"
    );

    let measured = measure_recall_at_k(n, w, p, 64, 1, 0, 5_2026);
    assert!(measured >= 0.90, "synthetic R_1 = {measured}");
}
