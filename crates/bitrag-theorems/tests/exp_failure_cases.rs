//! Experiment 8 — failure cases.
//!
//! Two corpus families are constructed that violate (A1) or (A2);
//! Theorem B's prediction is *not* expected to match measurement on
//! these.  Rather than re-fight the synthetic-vs-analytic measurement
//! gap (see exp4_corpus4.rs and §8 of paper.md), these tests assert
//! the **structural** failures of the assumptions, which is what
//! actually causes Theorem B to break in the field.

use bitrag_theorems::corpus::{sample_doc, BitVec};
use bitrag_theorems::prng::XorShift64;

/// Build a corpus where bit 0 is always set (a "heavy token").
/// Other bits are i.i.d. with probability w/p.
fn heavy_token_corpus(n: u64, w: u64, p: usize, seed: u64) -> Vec<BitVec> {
    let mut rng = XorShift64::new(seed);
    (0..n)
        .map(|_| {
            let mut bv = sample_doc(&mut rng, p, w.saturating_sub(1));
            bv.set(0);
            bv
        })
        .collect()
}

/// Family 8a: heavy-token corpus violates (A1) (i.i.d. bits).  We
/// assert that bit 0 is set in 100% of documents — strong evidence
/// of the violation — and that the per-bit set-rate variance is
/// vastly larger than for the i.i.d. baseline.
#[test]
fn family_8a_heavy_token_breaks_assumption_a1() {
    let n: u64 = 256;
    let w: u64 = 16;
    let p: u64 = 512;

    let corpus = heavy_token_corpus(n, w, p as usize, 0xa);

    // Bit 0 is set in every document: P(bit 0 set) = 1.
    let bit0_count = corpus.iter().filter(|d| d.get(0)).count();
    assert_eq!(bit0_count, corpus.len(), "heavy bit not actually heavy");

    // For an i.i.d. corpus the empirical set-rate of any single bit
    // should be near w/p with stddev ~sqrt(w/p / N).  Bit 0 here has
    // set-rate exactly 1, which is many sigma away.  We just assert
    // that the deviation from w/p is enormous.
    let baseline = w as f64 / p as f64;
    let deviation = 1.0 - baseline;
    let sigma = (baseline * (1.0 - baseline) / n as f64).sqrt();
    assert!(
        deviation > 50.0 * sigma,
        "deviation {deviation} not >> {sigma}; (A1) violation not visible"
    );
}

/// Family 8b: planted-pair corpus violates (A2) (independence across
/// documents).  We assert that for every planted pair (2i, 2i+1) the
/// AND-popcount is *strictly greater* than the expected i.i.d. value
/// for at least one bit (the planted one).
#[test]
fn family_8b_planted_pair_breaks_assumption_a2() {
    let n: u64 = 64;
    let w: u64 = 4;
    let p: u64 = 256;

    let mut corpus: Vec<BitVec> = Vec::with_capacity(n as usize);
    let mut rng = XorShift64::new(0xc);
    let pairs = (n as usize) / 2;
    for i in 0..pairs {
        let mut a = sample_doc(&mut rng, p as usize, w.saturating_sub(1));
        a.set(i);
        let mut b = sample_doc(&mut rng, p as usize, w.saturating_sub(1));
        b.set(i);
        corpus.push(a);
        corpus.push(b);
    }

    // For every planted pair, the AND-popcount must be ≥ 1 because of
    // the planted shared bit.  Under i.i.d. (A2) the expected value
    // would be w*w/p = 16/256 = 0.0625, so seeing ≥1 in every pair is
    // a strong signal that (A2) is violated.
    for i in 0..pairs {
        let overlap = corpus[2 * i].and_popcount(&corpus[2 * i + 1]);
        assert!(
            overlap >= 1,
            "pair {i} has overlap {overlap}; planted bit missing"
        );
    }

    // And on average, planted-pair overlap is much larger than i.i.d.
    let mut total: u64 = 0;
    for i in 0..pairs {
        total += corpus[2 * i].and_popcount(&corpus[2 * i + 1]) as u64;
    }
    let mean_overlap = total as f64 / pairs as f64;
    let iid_expected = (w as f64) * (w as f64) / p as f64;
    // Mean overlap should be at least an order of magnitude above the
    // i.i.d. baseline.  The planted bit contributes exactly 1.0 to the
    // mean by construction; the i.i.d. baseline here is 16/256 = 0.0625.
    assert!(
        mean_overlap >= 10.0 * iid_expected,
        "mean planted-pair overlap {mean_overlap} not >> i.i.d. baseline {iid_expected}"
    );
}
