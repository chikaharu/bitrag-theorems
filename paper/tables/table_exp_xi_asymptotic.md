# Table — Experiment 6 (small/large-$\xi$ asymptotic verification)

Reproduces the table in `paper.md` §6.3.

Theorem B claims two asymptotic regimes:

* **Small $\xi$ ($\xi \to 0$):** $R_1 \approx 1 - \xi$
* **Large $\xi$ ($\xi \to \infty$):** $\log R_1 \approx -\xi$

We verify these directly using `scaling::recall_at_k` over a sweep of
$\xi$ values.  All quantities are closed-form, no synthetic corpus is
required.

## Small-$\xi$ regime — slope of $1 - R_1$ vs $\xi$ should be $1$

| $\xi$    | $1 - R_1$ measured | $\xi$ (target) | rel. error |
|---:      |---:                |---:            |---:        |
| $10^{-6}$ | $9.999995 \times 10^{-7}$ | $10^{-6}$ | $5 \times 10^{-7}$ |
| $10^{-5}$ | $9.99995 \times 10^{-6}$ | $10^{-5}$ | $5 \times 10^{-6}$ |
| $10^{-4}$ | $9.9995 \times 10^{-5}$ | $10^{-4}$ | $5 \times 10^{-5}$ |
| $10^{-3}$ | $9.995 \times 10^{-4}$ | $10^{-3}$ | $5 \times 10^{-4}$ |

## Large-$\xi$ regime — slope of $-\log R_1$ vs $\xi$ should be $1$

| $\xi$ |  $-\ln R_1$ measured |  $\xi$ (target) | rel. error |
|---:   |---:                  |---:             |---:        |
|  10   | $10.0000$            | $10$            | $\le 10^{-4}$ |
|  20   | $20.0000$            | $20$            | $\le 10^{-9}$ |
|  50   | $50.0000$            | $50$            | $\le 10^{-15}$ |
| 100   | $100.0000$           | $100$           | $\le 10^{-15}$ |

Reproduce:

```bash
cargo test --release --test exp_xi_asymptotic -- --nocapture
```
