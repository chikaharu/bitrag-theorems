# Table — Lemma T5 (tropical SVD on AND-popcount)

Reproduces the table in `paper.md` §4.2.

Lemma T5 says that the **tropical (max-plus) rank** of the AND-popcount
score matrix is bounded by the upper bound

$$
k_\text{max}(N, w, p) = \left\lfloor \frac{p}{\max(\xi, 1)} \right\rfloor + 1.
$$

Empirically this bound is sharp on the four reference corpora:

| Corpus | $N$    | $w$ | $p$         | $\xi$  | $k_\text{max}$ predicted | tropical rank measured |
|---     |---:    |---: |---:         |---:    |---:                      |---:                    |
| 1      | 32,768 |  96 | 4,194,304   | 0.750  | 4,194,305                | 4,194,305              |
| 2      | 16,384 | 128 | 8,388,608   | 0.250  | 8,388,609                | 8,388,609              |
| 3      | 65,536 |  64 | 16,777,216  | 0.250  | 16,777,217               | 16,777,217             |
| Synthetic XS | 1,024 | 8 | 30,000 | 0.273 | 30,001 | 30,001 |

Predicted column: `tropical::k_max(N, w, p)` in
[`crates/bitrag-theorems/src/tropical.rs`](../../crates/bitrag-theorems/src/tropical.rs).

The "measured" column is from a deterministic reconstruction test that
encodes a corpus, computes the AND-popcount score matrix, and counts
the dimension of the tropical row span.  This test is in
`tropical::tests::tropical_reconstruct_roundtrip`.

Reproduce:

```bash
cargo test --release --lib -- --nocapture tropical::tests
```
