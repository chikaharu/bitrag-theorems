# `data/`

This directory is **intentionally almost empty** in source control.

It is the destination for the reproduction harness's output:

```
data/
├── README.md              <- this file
└── reproduce_output/      <- created by scripts/reproduce_all.sh
    ├── experiments.log
    ├── exp4_corpus4.log
    ├── exp5_corpus5.log
    ├── exp_xi_asymptotic.log
    ├── exp_jl_bound.log
    ├── exp_failure_cases.log
    ├── lib_tests.log
    ├── scaling_bench.csv
    ├── render_paper.log
    ├── paper.md
    └── tables/
        ├── table_corpus_1to3.md
        ├── table_exp4_corpus4.md
        ├── table_exp_jl_bound.md
        └── table_exp_failure_cases.md
```

## Why no real corpora are checked in

The four added experiments (Exp 4, Exp 5, Exp 7, Exp 8) are
**closed-form numerical reproductions** of the Theorem B prediction
plus deterministic synthetic-corpus measurements.  No external corpus
is downloaded or required.  Every number in `paper.md` can be
re-derived from the integer seeds in the `tests/exp*.rs` files alone.

For the (paper-only) reference to mC4-en-news (Corpus 4) and BEIR
NFCorpus (Corpus 5), download links and SHAs are listed in `paper.md`
§6.1 and §6.2.  These are not redistributed here.

## How to regenerate

```bash
bash scripts/reproduce_all.sh
```

That single command re-runs every experiment, regenerates every table,
and writes the artifacts under `data/reproduce_output/`.  It is the
exact command the CI workflow runs — see
[`.github/workflows/ci.yml`](../.github/workflows/ci.yml).
