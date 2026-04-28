# Table — Experiment 8 (failure cases)

Reproduces `paper.md` §6.5.

Two synthetic corpus families are constructed that violate (A1) /
(A2).  The structural assertions (which is what actually causes
Theorem B to break in the field) are checked in
[`crates/bitrag-theorems/tests/exp_failure_cases.rs`](../../crates/bitrag-theorems/tests/exp_failure_cases.rs).

## Family 8a — heavy-token (violates (A1) i.i.d.)

Construction: bit 0 is forced to `1` in every document; all other bits
are i.i.d. Bernoulli with `p = w/p_planes`.

| Quantity                       | Value         | Theorem B assumes |
|---                             |---:           |---:               |
| Empirical P(bit 0 set)         | 1.000         | $w/p = 16/512 = 0.03125$ |
| Deviation from i.i.d. baseline | 0.969         | < $5\sigma$ ($\sigma \approx 0.011$) |
| Sigma multiplier               | $> 50\sigma$ | (A1) violated |

## Family 8b — planted-pair (violates (A2) cross-doc independence)

Construction: documents are generated in pairs `(2i, 2i+1)`; both share
exactly one planted bit (bit `i`).

| Quantity                          | Value      | Theorem B assumes |
|---                                |---:        |---:               |
| Min planted-pair overlap          | $\ge 1$    | $w^2/p = 16/256 = 0.0625$ |
| Mean planted-pair overlap         | $1.0$      | $\approx 0.0625$ |
| Mean / i.i.d. baseline            | $\ge 16$   | $\approx 1$ |

In both families, the structural distribution of bits diverges by an
order of magnitude or more from what Theorem B's assumptions require.
This is the *root cause* of the prediction-vs-measurement gap in real
heavy-tail corpora; the gap is not a bug in the closed form.

Reproduce:

```bash
cargo test --release --test exp_failure_cases -- --nocapture
```
