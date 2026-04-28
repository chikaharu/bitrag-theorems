//! Lightweight benchmark for the closed-form scaling law.  Runs under
//! stable Rust (no `test::Bencher`) by using `harness = false` and
//! defining its own `main`.  Prints CSV to stdout so CI can grep
//! values.

use std::time::Instant;

use bitrag_theorems::scaling::recall_at_k;

fn time_loop<F: FnMut()>(mut f: F, iters: u32) -> f64 {
    let start = Instant::now();
    for _ in 0..iters {
        f();
    }
    let elapsed = start.elapsed();
    elapsed.as_secs_f64() * 1.0e9 / iters as f64
}

fn main() {
    println!("name,ns_per_op");

    // 1) recall_at_k at the Corpus 1 operating point.
    let xi_corpus_1 = bitrag_theorems::scaling::xi(32_768, 96, 4_194_304);
    let mut sink: f64 = 0.0;
    let ns = time_loop(
        || {
            sink += recall_at_k(xi_corpus_1, 1);
        },
        1_000_000,
    );
    println!("recall_at_k_corpus1_k1,{ns:.3}");

    // 2) recall_at_k with k=20 (all four corpora swept in §3.2).
    let ns = time_loop(
        || {
            sink += recall_at_k(xi_corpus_1, 20);
        },
        1_000_000,
    );
    println!("recall_at_k_corpus1_k20,{ns:.3}");

    // 3) Lemma T5 k_max evaluation.
    let ns = time_loop(
        || {
            sink += bitrag_theorems::tropical::k_max(32_768, 96, 4_194_304) as f64;
        },
        1_000_000,
    );
    println!("k_max_corpus1,{ns:.3}");

    // 4) F2-JL ratio at the Section 6.4 operating point.
    let ns = time_loop(
        || {
            sink += bitrag_theorems::jl_bound::ratio(10_000, 10, 0.1, 100.0, 0.9);
        },
        100_000,
    );
    println!("jl_ratio_n_10k,{ns:.3}");

    // Make sure the optimiser cannot drop the loops above.
    if sink.is_nan() {
        std::process::exit(1);
    }
}
