#![allow(clippy::uninlined_format_args)]
//! §4.2 Asymptotic ξ → 0 and ξ → ∞.
//!
//! Verify the two limit forms of `f(ξ) = 1 - exp(-ξ)`:
//!   - ξ → 0: `f(ξ) ≈ ξ - ξ²/2 + O(ξ³)`. We check `f(ξ)/ξ → 1`.
//!   - ξ → ∞: `f(ξ) → 1` and `1 - f(ξ) ≈ exp(-ξ)`.

use bitrag_theorems_experiments::f_two_term;

const DENOM: u64 = 1_000_000_000; // nano-precision

fn main() {
    println!("# §4.2 Asymptotic of f(ξ) = 1 - exp(-ξ)");
    println!();
    println!("## ξ → 0 regime: f(ξ) / ξ → 1");
    println!();
    println!("| N | w | p |        ξ |     f(ξ) |  f(ξ)/ξ |");
    println!("|--:|--:|--:|---------:|---------:|--------:|");
    // Hold w=64, p=64 so ξ = N·w/p = N. Vary N from 1 down to 0.001
    // by holding p large so ξ stays small.
    for &(n, w, p) in &[
        (1usize, 1024usize, 1_000_000usize), // ξ = 1024 / 1_000_000 ≈ 0.001
        (1, 1024, 100_000),                  // ξ ≈ 0.01
        (1, 1024, 10_000),                   // ξ ≈ 0.1
        (1, 1024, 1_000),                    // ξ ≈ 1.0
    ] {
        let xi_nano = ((n as u128) * (w as u128) * (DENOM as u128)) / (p as u128);
        let f = f_two_term(n, w, p, DENOM);
        let ratio = if xi_nano == 0 {
            0
        } else {
            (f as u128 * DENOM as u128) / xi_nano
        };
        println!(
            "| {:>1} | {:>4} | {:>10} | {:>8.5} | {:>8.5} | {:>7.4} |",
            n,
            w,
            p,
            xi_nano as f64 / DENOM as f64,
            f as f64 / DENOM as f64,
            ratio as f64 / DENOM as f64,
        );
    }
    println!();
    println!("(expectation: rightmost column → 1.0 from above as ξ → 0)");

    println!();
    println!("## ξ → ∞ regime: 1 - f(ξ) ≈ exp(-ξ)");
    println!();
    println!("| N | w | p |       ξ |   1-f(ξ) |   exp(-ξ) (ref) |");
    println!("|--:|--:|--:|--------:|---------:|----------------:|");
    for &(n, w, p) in &[
        (1usize, 64usize, 16usize), // ξ = 4
        (1, 64, 8),                 // ξ = 8
        (1, 64, 4),                 // ξ = 16
        (1, 64, 2),                 // ξ = 32
    ] {
        let xi = (n * w) / p;
        let f = f_two_term(n, w, p, DENOM);
        let one_minus = (DENOM as u128 - f as u128) as f64 / DENOM as f64;
        let exp_neg_xi = (-(xi as f64)).exp(); // reference (display only)
        println!(
            "| {:>1} | {:>2} | {:>1} | {:>7} | {:>8.6e} | {:>15.6e} |",
            n, w, p, xi, one_minus, exp_neg_xi
        );
    }
    println!();
    println!("(expectation: 1 - f(ξ) ≈ exp(-ξ) within 1e-3 relative for ξ ≤ 32)");
}
