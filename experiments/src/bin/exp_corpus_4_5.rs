#![allow(clippy::uninlined_format_args)]
//! §4.1 Cross-corpus validation: 4th and 5th corpora.
//!
//! Generate two fresh IID-Bernoulli(p/w) corpora with parameter
//! settings disjoint from the original three (E180/E181/E145), and
//! verify the empirical recall@1 falls within the 95% interval
//! around the Theorem-B prediction `R = f(Nw/p)`.
//!
//! Output: a markdown table to stdout. Deterministic for fixed seeds.

use bitrag_theorems_experiments::{f_two_term, recall_at_k, Corpus, SplitMix64};

const DENOM: u64 = 1_000_000;

#[derive(Clone, Copy)]
struct Setting {
    name: &'static str,
    n: usize,
    w: usize,
    p: usize,
    seed: u64,
}

fn run(s: Setting, queries: usize) -> (u64, u64) {
    let c = Corpus::new(s.n, s.w, s.p, s.seed);
    let mut rng = SplitMix64::new(s.seed.wrapping_add(0xCAFE));
    let mut hits = 0u64;
    for _ in 0..queries {
        let truth = (rng.next_u64() as usize) % c.n;
        // Query = a noisy copy of doc[truth]: keep half the bits.
        let mut q = c.doc(truth).to_vec();
        for w in q.iter_mut() {
            let mask = rng.next_u64();
            *w &= mask;
        }
        if recall_at_k(&c, &q, truth, 1) {
            hits += 1;
        }
    }
    let empirical_milli = (hits * DENOM) / queries as u64;
    // Theorem B prediction at the matched scale (effective ξ = N·w / (2p)
    // because the noisy query halved the load).
    let predicted = f_two_term(s.n, s.w, 2 * s.p, DENOM);
    (empirical_milli, predicted)
}

fn main() {
    let settings = [
        Setting {
            name: "C4 (small/dense)",
            n: 256,
            w: 512,
            p: 32,
            seed: 0xC4001,
        },
        Setting {
            name: "C4 (small/sparse)",
            n: 256,
            w: 512,
            p: 4,
            seed: 0xC4002,
        },
        Setting {
            name: "C5 (med/dense)",
            n: 1024,
            w: 1024,
            p: 64,
            seed: 0xC5001,
        },
        Setting {
            name: "C5 (med/sparse)",
            n: 1024,
            w: 1024,
            p: 8,
            seed: 0xC5002,
        },
    ];
    println!("# §4.1 Cross-corpus validation (C4, C5)");
    println!();
    println!("| corpus              |    N |    w |   p |   ξ=Nw/2p | empirical R@1 | predicted f(ξ) | abs error |");
    println!("|---------------------|-----:|-----:|----:|----------:|--------------:|---------------:|----------:|");
    let queries = 256;
    for s in settings {
        let (emp, pred) = run(s, queries);
        let xi = (s.n * s.w) / (2 * s.p);
        let abs_err = emp.abs_diff(pred);
        println!(
            "| {:<19} | {:>4} | {:>4} | {:>3} | {:>9} | {:>13.4} | {:>14.4} | {:>9.4} |",
            s.name,
            s.n,
            s.w,
            s.p,
            xi,
            emp as f64 / DENOM as f64,
            pred as f64 / DENOM as f64,
            abs_err as f64 / DENOM as f64,
        );
    }
    println!();
    println!("(seeds fixed; reproducible via `cargo run --release --bin exp_corpus_4_5`)");
}
