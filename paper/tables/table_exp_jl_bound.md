# Table — Experiment 7 (F2-JL lower-bound comparison)

Reproduces the table in `paper.md` §6.4.

Constants: $\epsilon = 0.1$, $C = 1$, target recall $R_1 \ge 0.9$.

## Scaled operating point — `w(N) = -ln(0.9) · p_JL(N) / N`

This is the comparison cited in §6.4.  At this scaled `w`, Theorem B's
required plane width *equals* the F2-JL lower bound exactly.

| $N$       | F2-JL lower bound on $p$ | Theorem B's $p$ for $R_1 \ge 0.9$ | Ratio |
|---:       |---:                       |---:                                |---:   |
| $10^3$    |   996                     |   996                              | 1.00  |
| $10^4$    | 1,329                     | 1,329                              | 1.00  |
| $10^5$    | 1,661                     | 1,661                              | 1.00  |
| $10^6$    | 1,993                     | 1,993                              | 1.00  |

## Fixed `w` (unscaled) — for context

At a fixed `w = 10` (and the same $\epsilon, C, R_1$ targets) the ratio
diverges linearly in $N$, exactly as expected — the F2-JL bound only
needs to *distinguish* the documents, but Theorem B asks for the
relevant doc to win the AND-popcount competition.

| $N$       | F2-JL on $p$ | Theorem B's $p$ | Ratio (≈ N · w / (-ln R · p_JL)) |
|---:       |---:           |---:              |---: |
| $10^3$    |   996         |     94,913       |  ~95 |
| $10^6$    | 1,993         | 94,912,422       |  ~47,624 |

Reproduce both tables:

```bash
cargo test --release --test exp_jl_bound -- --nocapture
```
