# Table — Corpora 1–3 (Theorem B closed-form fit)

Reproduces the table in `paper.md` §3.2.

| Corpus | $N$ | $w$ | $p$ | $\xi$ | $R_1$ predicted | $R_1$ measured |
|---|---:|---:|---:|---:|---:|---:|
| 1 (Wiki-EN-mini)         | 32,768 |  96 | 4,194,304  | 0.750 | 0.472 | 0.471 |
| 2 (CodeSearchNet-Rust)   | 16,384 | 128 | 8,388,608  | 0.250 | 0.779 | 0.781 |
| 3 (StackOverflow-titles) | 65,536 |  64 | 16,777,216 | 0.250 | 0.779 | 0.770 |

Predicted column is `recall_at_k(xi(N, w, p), 1)`, evaluated by
`crates/bitrag-theorems/src/scaling.rs` (`scaling::recall_at_k`).

The measured column is from the bitRAG E0–E2 experiments and was
recorded prior to this paper; it is reproduced here unchanged.

All three rows fall inside the $\pm 0.02$ envelope predicted by the
$O(\xi^2 / p)$ error term, so Theorem B is empirically tight on these
three corpora.

Reproduce:

```bash
cargo test --release --lib -- --nocapture scaling::tests::xi_basic
cargo test --release --lib -- --nocapture scaling::tests::paper_predictions_match
```
