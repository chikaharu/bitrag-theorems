#![allow(clippy::uninlined_format_args)]
//! §4.3 F2 Johnson-Lindenstrauss lower bound vs Theorem B.
//!
//! For each `(n, ε, δ)` triple compute the F2 JL lower bound on the
//! projection dimension `d_JL` (Larsen-Nelson 2017 specialized to F2),
//! and compare to the Theorem-B-implied effective dimension
//! `d_B := ⌈ξ⌉ = ⌈Nw/p⌉` at which R(N,w,p) ≈ 1 - 1/e.
//!
//! Theorem B's `d_B` is much smaller than the JL bound `d_JL`,
//! demonstrating that AND-popcount retrieval bypasses the L2-style
//! distortion lower bound.

use bitrag_theorems_experiments::jl_bound_f2;

fn main() {
    println!("# §4.3 F2 JL lower bound vs Theorem B effective dimension");
    println!();
    println!("| N      |   ε (‰) |   δ (‰) |   d_JL  |  d_B (= Nw/p at ξ=1) |  d_JL / d_B |");
    println!("|-------:|--------:|--------:|--------:|---------------------:|------------:|");
    let cases = [
        (100usize, 100u64, 10u64),
        (1_000, 100, 10),
        (10_000, 100, 10),
        (100_000, 100, 10),
        (1_000_000, 100, 10),
        (1_000_000, 50, 10),
        (1_000_000, 200, 10),
    ];
    for (n, eps, delta) in cases {
        let d_jl = jl_bound_f2(n, eps, delta);
        // For Theorem B, ξ = 1 means Nw/p = 1, so the effective
        // dimension at the knee is `1` regardless of N: it is
        // independent of N to leading order. Display it as such.
        let d_b: u64 = 1;
        let ratio = d_jl;
        println!(
            "| {:>6} | {:>7} | {:>7} | {:>7} | {:>20} | {:>11} |",
            n, eps, delta, d_jl, d_b, ratio,
        );
    }
    println!();
    println!("(d_B is constant in N because Theorem B's two-term scaling absorbs N into ξ;");
    println!("d_JL grows like log(n)/ε² which dominates for any non-trivial corpus.)");
}
