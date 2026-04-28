//! # bitrag-theorems
//!
//! Companion crate to `paper.md` in the repository root.  Implements the
//! closed-form pieces of Theorem B (F2 retrieval scaling law), Lemma T5
//! (tropical rank upper bound) and the helpers needed by the four added
//! experiments and by Corollary C.
//!
//! Everything in this crate is **deterministic and integer-only on the
//! input side**: every floating-point value comes from a closed-form
//! function of integer parameters, evaluated through `f64::from(i32)`
//! and `libm`-free arithmetic, so reproductions are byte-equal across
//! IEEE-754-conformant platforms.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod corpus;
pub mod jl_bound;
pub mod prng;
pub mod scaling;
pub mod tropical;

pub use jl_bound::{f2_jl_lower_bound_p, theorem_b_required_p};
pub use scaling::{recall_at_k, xi};
pub use tropical::k_max;
