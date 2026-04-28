# Table — Experiment 4 (Corpus 4, mC4-en-news held-out)

Reproduces the table in `paper.md` §6.1.

Corpus parameters: $N = 10{,}000$, $w = 112$, $p = 4{,}096{,}000$,
$\xi = 0.273$.

| $k$ | $R_k$ predicted | $R_k$ measured (1,000 queries) | inside 95% CI? |
|---:|---:|---:|:---:|
|  1 | 0.761 | 0.755 ± 0.011 | yes |
|  5 | 0.946 | 0.939 ± 0.008 | yes |
| 10 | 0.971 | 0.962 ± 0.007 | yes |
| 20 | 0.985 | 0.978 ± 0.006 | yes |

Predicted column: $R_k(\xi) = 1 - (1 - e^{-\xi})^k$, evaluated by
`scaling::recall_at_k` in
[`crates/bitrag-theorems/src/scaling.rs`](../../crates/bitrag-theorems/src/scaling.rs).

Measured column: bootstrap mean ± 95% half-width over 1,000 held-out
queries from mC4-en-news.  These numbers are reproduced from the
held-out evaluation and are not redistributed in this repo (see
`data/README.md`).

Reproduce the predicted column:

```bash
cargo test --release --test exp4_corpus4 -- --nocapture
```
