# bitrag-theorems

**bitRAG retrieval theorems**: a single, honest, reproducible
research artifact for the F2 bit-vector retrieval scaling law.

> Theorem B (two-term scaling): `R = f(Nw/p)` with `f(ξ) = 1 − exp(−ξ)`
>
> Lemma T5: AND-popcount Gram blocks are bounded-residual rank-1
> tropical
>
> Corollary C: LLM-less Rust syntactic repair is `O(Nw/p)`
> sample-complexity

The full statement, assumptions (A1)–(A3), proof sketches, four
supplementary experiments (cross-corpus / asymptotic ξ /
F2 Johnson-Lindenstrauss / failure cases), and the appendix on
numerical reproducibility live in [`paper.md`](paper.md).

## Layout

```
bitrag-theorems/
├── paper.md             ★ The paper. Read this first.
├── cite.bib             BibTeX entries for the paper, cite.bib helpers
├── experiments/         Reproducibility crate (Rust, Apache-2.0, MSRV 1.70)
│   ├── src/lib.rs           Corpus / recall_at_k / f_two_term / Lemma T5 / JL bound
│   └── src/bin/             4 supplementary experiments (§4.1 – §4.4)
├── scripts/reproduce.sh One command to regenerate every table in §4
├── .github/workflows/ci.yml  fmt + clippy + test + reproduce + paper.md sanity
└── LICENSE              Apache-2.0
```

## Reproduce all tables

```bash
git clone https://github.com/chikaharu/bitrag-theorems
cd bitrag-theorems
./scripts/reproduce.sh > my-tables.md
```

The output is byte-identical to the paper's §4 (deterministic seeded
PRNG, integer-only kernel, no floats in the F2 inner product).

## Related repositories

| Repo                                                                          | Role                                                                                       |
| ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| [`chikaharu/bitRAG`](https://github.com/chikaharu/bitRAG)                     | Upstream retrieval engine (E180/E181/E145/E163a-d/E110/E134 referenced in §2 proof sketches) |
| [`chikaharu/bitGradient`](https://github.com/chikaharu/bitGradient)           | Optimization dual of Theorem B (Cor 4 of MAIN-B): Discrete Gradient Descent on bit-vector states |
| [`chikaharu/bitrag-int-diag`](https://github.com/chikaharu/bitrag-int-diag)   | **Appendix A** of this paper: integer-IDF² diagonal unitization (commit `102a3c66`+) for byte-identical numerical reproducibility |

## License

Apache-2.0.
