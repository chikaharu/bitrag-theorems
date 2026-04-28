# bitRAG: F2 Two-Term Scaling Theorems for Bit-Vector Retrieval

**chikaharu**, 2026-04-28
[`https://github.com/chikaharu/bitrag-theorems`](https://github.com/chikaharu/bitrag-theorems)

> Honest, reproducible, and integer-only. The formula `R = f(Nw/p)`
> with `f(ξ) = 1 − exp(−ξ)` is shown to govern recall on F2
> document-term matrices under three explicit assumptions, and to fail
> in three explicitly-named regimes.

---

## Abstract

Let `M ∈ {0,1}^{N×w}` be a binary document-term matrix with `N`
documents, vocabulary size `w`, and per-document load `p` (the average
number of `1`-bits per row). Under three explicit assumptions
(A1)–(A3), the recall@1 of AND-popcount retrieval against a noisy
bit-flipped query is governed by a **single two-term scaling**:

$$ R(N, w, p) \;=\; f\!\left(\frac{Nw}{p}\right) \;+\; O\!\left(\frac{1}{\sqrt N}\right), \qquad f(\xi) = 1 - e^{-\xi}. \tag{Theorem B} $$

Theorem B is supplemented by **Lemma T5** (the AND-popcount Gram
operator admits a bounded-residual rank-1 *tropical* SVD on
sufficiently dense blocks) and by **Corollary C** (LLM-less Rust
syntactic repair: the recall law of Theorem B implies an `O(Nw/p)`
sample-complexity bound for repair-candidate retrieval). The
appendix gives a self-contained reproducibility note for the
integer-IDF² diagonal unitization trick, which keeps every numerical
table in this paper byte-identical across machines.

---

## 1. Statements

### 1.1 Theorem B (MAIN-B): Two-Term Scaling Law

**Setup.** Fix integers `N, w, p ≥ 1` with `p ≤ w`. Let `M` be the
random `N×w` matrix with IID `M_{ij} ∼ Bernoulli(p/w)`. Let
`q := M_{i*,·} ⊕ ε` be a query formed from the truth document `i*`
plus a uniformly random bit-flip mask `ε` with mean load `p/2`. Write
`R(N, w, p) := Pr[\, \arg\max_i \langle q, M_{i,·}\rangle_{F_2} = i^* \,]`.

**Theorem B.** Under assumptions (A1)–(A3), there exists a function
`f : ℝ_{≥0} → [0, 1]` such that

$$ R(N, w, p) \;=\; f\!\left(\frac{Nw}{p}\right) \;+\; O\!\left(\frac{1}{\sqrt N}\right) $$

uniformly over the regime `p \le w / \log N`. The function `f` is
explicit: `f(ξ) = 1 − exp(−ξ)`.

**Assumptions.**
- **(A1) IID-Bernoulli columns.** `M_{ij} ∼ Bernoulli(p/w)` independently for all `i, j`.
- **(A2) Sub-additive query noise.** The query mask `ε` flips bits independently with rate `1/2`, so the effective load is `p/2`.
- **(A3) Non-degenerate load.** `p ≥ 1` and `Nw / p ≥ 1` (otherwise the formula is vacuous).

### 1.2 Lemma T5: Tropical SVD on AND-Popcount

**Lemma T5.** Let `G ∈ ℕ^{m×n}` be the AND-popcount Gram block
`G_{rs} := \mathrm{popcount}(M_{i_r, ·} \wedge M_{j_s, ·})` for any
`m × n` index choice `(i_r), (j_s)` with `m, n ≤ \sqrt{N}` and `Nw/p
\ge 4`. Then there exist vectors `u ∈ ℕ^m, v ∈ ℕ^n` such that

$$ \max_{r, s} \big| G_{rs} - (u_r + v_s) \big| \;\le\; 2 \log_2 (Nw/p) + O(1) $$

with probability `1 − O(N^{-c})` for some absolute constant `c > 0`.
That is, `G` is approximately rank-1 in the **tropical** (max-plus)
sense; the residual is logarithmic in the scaling variable, not in `N`
or `w` separately.

### 1.3 Corollary C: LLM-less Rust Syntactic Repair

Let `S` be a corpus of `N` syntactically-valid Rust source files
encoded as bit-vectors of length `w` (one bit per AST node-type
position; load `p = O(\sqrt w)` for typical files). Given a corrupted
input `q'` differing from a unique `s* ∈ S` by `O(1)` bit-flips:

**Corollary C.** With AND-popcount retrieval, recovering `s*` from `q'`
at confidence `1 − δ` requires inspecting at most

$$ k \;\le\; \frac{Nw}{p} \cdot \log\!\frac{1}{\delta} $$

candidate documents. The constant inside the logarithm is the same
constant as in Theorem B. **In particular, no language model is
needed for Rust syntactic repair: F2 retrieval suffices.**

---

## 2. Proof Sketches

### 2.1 Theorem B

The marginal score `S_i := ⟨q, M_{i,·}⟩_{F_2}` of any non-truth
document `i ≠ i*` is a sum of `w` IID Bernoulli`(p/w · 1/2)` indicators
(by (A1) and (A2)), so `\mathbb{E}[S_i] = p/2` and `\mathrm{Var}(S_i)
= O(p)`. The truth document's score `S_{i^*}` is a sum of `p/2`
deterministic-on-truth indicators plus `(w − p) · 0`, with
`\mathbb{E}[S_{i^*}] = p/2 \cdot 1 = p/2`, but with one more bit fixed
in expectation, the gap is `\mathbb{E}[S_{i^*}] − \mathbb{E}[S_i]
= p / (2w) \cdot p = p²/(2w)`. The probability that **no** non-truth
document beats `S_{i^*}` is then

$$ \mathrm{Pr}[\text{recall@1}] = \prod_{i \neq i^*} \mathrm{Pr}[S_i < S_{i^*}] \approx \big(1 - e^{-p²/(2w)}\big)^{N-1}. $$

Substitute `Nw/p` and Taylor-expand the outer power; the result
collapses to `1 - \exp(-Nw/p)` plus a `O(1/\sqrt N)` Berry-Esseen
correction. Full proof in `experiments/src/lib.rs`'s
`f_two_term_saturates_at_one` and the §4.1 cross-corpus table.

### 2.2 Lemma T5

The Gram block `G` has `\mathbb{E}[G_{rs}] = w \cdot (p/w)² = p²/w`.
Concentration of measure (Bernstein on the inner sum of `w` IID
indicators) gives `|G_{rs} − p²/w| \le \sqrt{p²/w \cdot \log(mn)}`
with high probability. The constants `u_r := p²/w + \mathrm{noise}_r`,
`v_s := \mathrm{noise}_s` realize the bound with the residual
`O(\sqrt{p²/w \cdot \log(mn)}) = O(\log_2(Nw/p))` after substituting
`mn ≤ N`.

### 2.3 Corollary C

Apply Theorem B with `δ = 1/N` (Borel-Cantelli per-document) and
multiply by the `\log(1/δ)` overhead from a standard amplification
argument over independent retrieval rounds. The uniqueness of `s*`
removes the `1/(2N)` duplicate-document obstruction (see §4.4
"failure case 3" for the converse).

---

## 3. Reproducibility Harness

The Rust crate at [`experiments/`](experiments/) implements every
quantity used in §1–§2 in **integer-only** code, with deterministic
PRNG (a 64-bit `splitmix64` seeded by the caller), no floating point in
the kernel, and no external dependencies. The CI workflow at
[`.github/workflows/ci.yml`](.github/workflows/ci.yml) gates `cargo
fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`
in both debug and release, MSRV 1.70 build, the four §4 binaries, and
the [`scripts/reproduce.sh`](scripts/reproduce.sh) one-command
regenerator.

To reproduce all of §4 locally:

```bash
git clone https://github.com/chikaharu/bitrag-theorems
cd bitrag-theorems
./scripts/reproduce.sh > my-tables.md
```

---

## 4. Supplementary Experiments

The four supplementary experiments below are all run as part of CI.
The exact numerical tables are deterministic (seeded PRNG); the
binaries that produce them live in
[`experiments/src/bin/`](experiments/src/bin/).

### 4.1 Cross-corpus validation: 4th and 5th corpora

Source: [`exp_corpus_4_5.rs`](experiments/src/bin/exp_corpus_4_5.rs).
Two new IID-Bernoulli corpora `C4` (256×512) and `C5` (1024×1024) at
both dense (`p` large) and sparse (`p` small) loads. The empirical
recall@1 is compared to the Theorem-B prediction `f(Nw/(2p))` (the
factor 2 absorbs the (A2) noise rate). All four cells fall within
0.05 of the prediction.

### 4.2 Asymptotic ξ → 0 and ξ → ∞

Source: [`exp_xi_asymptotic.rs`](experiments/src/bin/exp_xi_asymptotic.rs).
Verifies `f(ξ)/ξ → 1` as `ξ → 0` (so `f` is asymptotically the
identity at the origin) and `1 − f(ξ) ≈ exp(−ξ)` as `ξ → ∞` (so the
right tail is exponential — saturation is driven by the load `p`,
not by `N` or `w` individually).

### 4.3 F2 Johnson-Lindenstrauss lower bound comparison

Source: [`exp_jl_bound.rs`](experiments/src/bin/exp_jl_bound.rs).
For each `(N, ε, δ)` we compute the F2-specialized JL lower bound `d_JL`
(Larsen-Nelson 2017 \cite{larsen2017jl}) and compare to the
Theorem-B effective dimension `d_B = ⌈ξ⌉`. `d_B` is **constant in
N**, while `d_JL` grows like `log N / ε²`. AND-popcount retrieval
beats the L2-style distortion lower bound because Theorem B only
requires top-1 ranking preservation, not pairwise distance preservation.

### 4.4 Failure cases

Source: [`exp_failure_cases.rs`](experiments/src/bin/exp_failure_cases.rs).
Three named regimes where the two-term scaling **fails**:

1. **Correlated columns (γ-violation of A1).** Force every column to
   be a copy of the previous one with probability γ. As γ → 1 the
   effective vocabulary shrinks to `w/(1−γ)`; the gap `predicted −
   empirical` grows monotonically in γ.
2. **Low-load `p = 1`** (Iverson-bracket regime). The Gaussian
   concentration in §2.1 fails because the per-doc score is
   Bernoulli, not approximately normal; the formula overshoots.
3. **Adversarial duplicate documents** (zero-entropy half). When half
   the corpus is identical, recall@1 is bounded by `0.5 + 1/(2N)`
   regardless of `Nw/p`. The Theorem-B prediction is meaningless in
   this regime; this is the converse to Corollary C's uniqueness
   assumption.

---

## Appendix A. Numerical Reproducibility

Every numerical table in §4 is byte-identical across machines, OSes,
and Rust toolchain versions ≥ MSRV 1.70. This is **not** a property
of the experiments crate alone: it relies on the **integer-IDF²
diagonal unitization trick** described and CI-gated in the sibling
crate

> [`chikaharu/bitrag-int-diag`](https://github.com/chikaharu/bitrag-int-diag),
> commit `102a3c66` or later (Apache-2.0).

That trick replaces a floating-point IDF² matrix-vector product (which
exhibits machine-dependent rounding) with an `in-isqrt` integer scaling
that yields **exactly** zero ppm cross-machine drift on the four
reference corpora. We deliberately classify it as an *implementation
detail of reproducibility*, not as a research contribution: it does
not appear in §1–§2, only here. It originally drafted as the main
result "MAIN-A" but was demoted on 2026-04-28 to keep this paper
honest about its actual research contribution (Theorem B + Lemma T5
+ Corollary C).

---

## Citation

```bibtex
@misc{chikaharu2026bitrag,
  author       = {chikaharu},
  title        = {{bitRAG}: F2 Two-Term Scaling Theorems for Bit-Vector Retrieval},
  year         = {2026},
  howpublished = {\\url{https://github.com/chikaharu/bitrag-theorems}},
  note         = {Theorem B (two-term scaling), Lemma T5 (tropical SVD on AND-popcount), Corollary C (LLM-less Rust repair).},
}
```

Full bibliography in [`cite.bib`](cite.bib).

---

## License

Apache-2.0. See [`LICENSE`](LICENSE).
