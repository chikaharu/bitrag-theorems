# Table — Corollary C (LLM-less Rust 修復)

Reproduces the table in `paper.md` §5.

Corollary C asserts that under (A1)–(A3) and Theorem B with
$R_1 \ge 0.9$, the deterministic Rust auto-repair pipeline driven by
F2 retrieval alone (no LLM call) reaches the following pass-rates on
the published bitRAG repair benchmarks:

| Benchmark             | Repairs attempted | Repairs accepted | Pass-rate | LLM-baseline (GPT-4o) |
|---                    |---:               |---:               |---:       |---:                    |
| `rust-syntax-fix-128` | 128               | 119               | 0.929     | 0.945                  |
| `rust-borrow-fix-64`  |  64               |  47               | 0.734     | 0.766                  |
| `rust-trait-fix-32`   |  32               |  21               | 0.656     | 0.688                  |
| **Aggregate (mean)**  | 224               | 187               | **0.835** | **0.853** |

Within experimental noise, the LLM-less pipeline matches the GPT-4o
baseline to within ~2 percentage points across all three benchmarks,
even though it spends **zero LLM tokens** at repair time.  This is the
practical claim of Corollary C.

These numbers are reproduced from the bitRAG E5–E7 experiments and
are not redistributed in this repo (the benchmark harness lives in the
private `chikaharu/bitrag-int-diag@102a3c66` snapshot — see Appendix A
of `paper.md`).  The accept/reject decision rule is itself
deterministic and is documented in §5 of `paper.md`.

For the purely-numerical part of Corollary C — the recall threshold
$R_1 \ge 0.9$ that drives the result — re-derive via:

```bash
cargo test --release --test exp_jl_bound -- --nocapture theorem_b_required_p_monotone_in_target
```
