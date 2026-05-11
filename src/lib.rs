//! Bitcoin user policy: a descriptor-agnostic policy model on top of `rust-miniscript`.
//!
//! Policies are modelled as a set of [`Path`]s, each carrying a key shape ([`Semantic`])
//! and an optional locktime gate ([`Locktime`]). A [`Policy`] compiles into a Taproot
//! multipath [`miniscript::Descriptor`] and round-trips back via [`Policy::from_descriptor`].

pub mod compile;
pub mod multipath;
pub mod nums;
pub mod parse;
pub mod path;
pub mod policy;
pub mod tree_builder;

pub use multipath::{MultipathError, get_multipath_index};
pub use nums::{bip341_nums, unspendable_internal_key};
pub use path::{
    CLTV_ALIGNMENT, Leaf, Locktime, MULTIPATH_SEMANTIC_FACTOR, OXpub, OXpubError, Path, Semantic,
    TapPosition, cltv_align, is_cltv_aligned,
};
pub use policy::{Policy, PolicyError, PolicyType};
pub use tree_builder::TreeBuilderError;
