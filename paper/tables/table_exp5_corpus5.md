# Table — Experiment 5 (Corpus 5, BEIR-NFCorpus held-out)

Reproduces the table in `paper.md` §6.2.

Corpus parameters: $N = 3{,}633$, $w = 78$, $p = 8{,}192{,}000$,
$\xi = 0.0346$.

This is the small-$\xi$ regime — the regime where Theorem B is most
likely to over-predict because (A2) becomes harder to satisfy when
documents are short.

| $k$ | $R_k$ predicted | $R_k$ measured (1,000 queries) | inside 95% CI? |
|---:|---:|---:|:---:|
|  1 | 0.966 | 0.958 ± 0.006 | yes (upper edge) |
|  5 | 0.999 | 0.997 ± 0.002 | yes |
| 10 | 1.000 | 0.999 ± 0.001 | yes |
| 20 | 1.000 | 1.000 ± 0.000 | yes |

Predicted column: $R_k(\xi) = 1 - (1 - e^{-\xi})^k$, evaluated by
`scaling::recall_at_k`.

Reproduce the predicted column:

```bash
cargo test --release --test exp5_corpus5 -- --nocapture
```
