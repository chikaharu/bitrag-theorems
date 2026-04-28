#![allow(clippy::uninlined_format_args)]
//! §4.4 Failure cases: where the two-term scaling R = f(Nw/p) breaks.
//!
//! The two-term scaling assumes (A1) IID-Bernoulli columns. We
//! deliberately violate (A1) and quantify the deviation:
//!
//! 1. **Correlated-column**: every column is a copy of the previous
//!    one with probability γ. As γ → 1, the effective vocabulary
//!    collapses to `w/(1-γ)` and recall drops far below f(ξ).
//! 2. **Low-load (p=1)**: each document holds exactly one bit.
//!    AND-popcount becomes Iverson-bracket retrieval and the
//!    Theorem-B prediction overshoots empirical recall.
//! 3. **Adversarial duplicate documents**: half the corpus is
//!    identical (zero entropy). The "truth" can never be uniquely
//!    identified — recall@1 is bounded by 0.5 + 1/(2N).

use bitrag_theorems_experiments::{f_two_term, recall_at_k, Corpus, SplitMix64};

const DENOM: u64 = 1_000_000;

fn correlated_corpus(n: usize, w: usize, p: usize, gamma_milli: u64, seed: u64) -> Corpus {
    let mut c = Corpus::new(n, w, p, seed);
    let mut rng = SplitMix64::new(seed ^ 0xC0FFEE_u64);
    for d in 0..n {
        let row_off = d * c.words;
        for bit in 1..w {
            // With probability γ, copy bit (bit-1) onto bit `bit`.
            if rng.bernoulli(gamma_milli, 1000) {
                let src_word = (bit - 1) / 64;
                let src_mask = 1u64 << ((bit - 1) % 64);
                let src_set = (c.docs[row_off + src_word] & src_mask) != 0;
                let dst_word = bit / 64;
                let dst_mask = 1u64 << (bit % 64);
                if src_set {
                    c.docs[row_off + dst_word] |= dst_mask;
                } else {
                    c.docs[row_off + dst_word] &= !dst_mask;
                }
            }
        }
    }
    c
}

fn empirical_recall(c: &Corpus, queries: usize, seed: u64) -> u64 {
    let mut rng = SplitMix64::new(seed);
    let mut hits = 0u64;
    for _ in 0..queries {
        let truth = (rng.next_u64() as usize) % c.n;
        let mut q = c.doc(truth).to_vec();
        for w in q.iter_mut() {
            *w &= rng.next_u64();
        }
        if recall_at_k(c, &q, truth, 1) {
            hits += 1;
        }
    }
    (hits * DENOM) / queries as u64
}

fn main() {
    println!("# §4.4 Failure cases of the two-term scaling");
    println!();

    println!("## (1) Correlated columns (γ-violation of A1)");
    println!();
    println!("| γ (‰) | empirical R@1 | predicted f(ξ) | drop (pred − emp) |");
    println!("|------:|--------------:|---------------:|------------------:|");
    let n = 256;
    let w = 512;
    let p = 16;
    let predicted = f_two_term(n, w, 2 * p, DENOM);
    for gamma in [0u64, 200, 500, 800, 950] {
        let c = correlated_corpus(n, w, p, gamma, 0xFA11A);
        let emp = empirical_recall(&c, 128, 0xFA11A_u64.wrapping_add(gamma));
        let drop = predicted.saturating_sub(emp);
        println!(
            "| {:>5} | {:>13.4} | {:>14.4} | {:>17.4} |",
            gamma,
            emp as f64 / DENOM as f64,
            predicted as f64 / DENOM as f64,
            drop as f64 / DENOM as f64,
        );
    }
    println!();
    println!("(expectation: the gap grows monotonically with γ; at γ=0.95 the");
    println!("effective vocabulary is ≈ w/(1-γ) = 25.6 and recall collapses)");

    println!();
    println!("## (2) Low-load p = 1 (Iverson-bracket regime)");
    println!();
    println!("| N    | w   | empirical R@1 | predicted f(ξ) | overshoot |");
    println!("|-----:|----:|--------------:|---------------:|----------:|");
    for &(n, w) in &[(64usize, 64usize), (256, 256), (1024, 1024)] {
        let c = Corpus::new(n, w, 1, 0xBEEF);
        let emp = empirical_recall(&c, 128, 0xBEEF1);
        let pred = f_two_term(n, w, 2, DENOM);
        let overshoot = pred.saturating_sub(emp);
        println!(
            "| {:>4} | {:>3} | {:>13.4} | {:>14.4} | {:>9.4} |",
            n,
            w,
            emp as f64 / DENOM as f64,
            pred as f64 / DENOM as f64,
            overshoot as f64 / DENOM as f64,
        );
    }

    println!();
    println!("## (3) Duplicate-document corpus (zero entropy half)");
    println!();
    // With (n/2) duplicates of doc 0 and tie-breaking by lowest index,
    // the truth in the duplicate half resolves to doc 0 with prob 2/N,
    // giving R@1 = 1/2 · 1 + 1/2 · 2/N = 0.5 + 1/N asymptotically.
    println!("| N    | empirical R@1 | analytic 0.5 + 1/N |");
    println!("|-----:|--------------:|-------------------:|");
    for &n in &[64usize, 256, 1024] {
        let mut c = Corpus::new(n, 256, 16, 0xDEAD);
        // Force the second half of the corpus to be identical to doc 0.
        let template = c.doc(0).to_vec();
        for d in (n / 2)..n {
            let off = d * c.words;
            c.docs[off..off + c.words].copy_from_slice(&template);
        }
        let emp = empirical_recall(&c, 128, 0xDEAD1);
        let analytic = (DENOM / 2) + (DENOM / n as u64);
        println!(
            "| {:>4} | {:>13.4} | {:>18.4} |",
            n,
            emp as f64 / DENOM as f64,
            analytic as f64 / DENOM as f64,
        );
    }
    println!();
    println!("(expectation: empirical R@1 ≈ 0.5 + 1/N within sampling noise from 128 queries)");
}
