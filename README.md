# bitrag-theorems

> Main Theorem **MAIN-B** of [bitRAG](https://github.com/chikaharu/bitRAG) — the
> two-parameter scaling law of F2 retrieval — together with its lemma, its
> corollary, and the four external-validation experiments that pin it down.

This repository is the **independent, citable companion** to the
[bitRAG](https://github.com/chikaharu/bitRAG) research line.  It exists so
other researchers can read, reproduce and cite the main theorem **without**
having to clone the 718-commit experimental monorepo.

## Contents

| File | Purpose |
| --- | --- |
| [`paper.md`](paper.md) | The paper itself (Theorem B, Lemma T5, Corollary C, four experiments). |
| [`cite.bib`](cite.bib) | BibTeX entry for citation. |
| [`crates/bitrag-theorems/`](crates/bitrag-theorems) | Rust crate that re-implements the analytical pieces (scaling law, tropical rank bound, F2 JL lower bound). |
| [`scripts/reproduce_all.sh`](scripts/reproduce_all.sh) | Single command that regenerates every plot and table in `paper.md`. |
| [`paper/tables/`](paper/tables) | Reproduced tables (auto-generated). |
| [`paper/figures/`](paper/figures) | Reproduced figures (auto-generated). |
| [`data/README.md`](data/README.md) | Where to obtain the five corpora used for external validation. |

## Quick reproduction

```bash
git clone https://github.com/chikaharu/bitrag-theorems
cd bitrag-theorems
./scripts/reproduce_all.sh        # ~3 minutes on a laptop
```

Every number that appears in `paper.md` is regenerated under
`paper/tables/` and `paper/figures/`.  Byte-for-byte reproducibility across
machines is guaranteed by the integer-only arithmetic discussed in
[Appendix A](paper.md#appendix-a-numerical-reproducibility), which delegates
to the external crate
[`chikaharu/bitrag-int-diag`](https://github.com/chikaharu/bitrag-int-diag)
(commit `102a3c66` or later).

## Continuous Integration

The CI workflow lives at [`ci/ci.yml`](ci/ci.yml).  GitHub Actions
expects workflow files under `.github/workflows/`, so install it once
with:

```bash
mkdir -p .github/workflows && git mv ci/ci.yml .github/workflows/ci.yml
git commit -m "Install CI workflow"
git push
```

(The file lives at `ci/ci.yml` because the Replit GitHub connector
that bootstrapped this repo lacks the `workflow` OAuth scope; once
the file is moved the workflow runs on every push to `main` and on
every pull request.  It runs `cargo fmt --check`, `cargo clippy
-D warnings`, `cargo test --release`, and finally `bash
scripts/reproduce_all.sh`, all of which pass on a fresh checkout.)

## How to cite

```bibtex
@misc{chikaharu2026bitrag,
  author       = {Chikaharu},
  title        = {bitRAG: A Two-Parameter Scaling Law for {F2} Retrieval},
  year         = {2026},
  howpublished = {\url{https://github.com/chikaharu/bitrag-theorems}},
  note         = {Theorem MAIN-B, Lemma T5, Corollary C}
}
```

See [`cite.bib`](cite.bib) for the full entry.

## Related repositories

- [`chikaharu/bitRAG`](https://github.com/chikaharu/bitRAG) — the experimental
  monorepo (E110, E132–E134, E145, E163a–d, E180, E181) that produced the
  empirical evidence.
- [`chikaharu/bitrag-int-diag`](https://github.com/chikaharu/bitrag-int-diag) —
  the integer-IDF² diagonal-unitisation crate referenced from Appendix A.
- [`chikaharu/bitGradient`](https://github.com/chikaharu/bitGradient) —
  Discrete Gradient Descent on {0,1}ⁿ; uses Theorem B as a parameter
  oracle.
- [`chikaharu/tren-crc`](https://github.com/chikaharu/tren-crc) — defines the
  AND-popcount routing operator that Lemma T5 acts on.

## License

Apache-2.0 © 2026 Chikaharu.  See [`LICENSE`](LICENSE).
