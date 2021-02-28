//! [`AssocExt`] is a trait extension that allows you to work with [`Vec<(K, V)>`][Vec] as if it
//! were a map.
//! It provides APIs for querying, adding, and removing key-value pairs, as well as an [`Entry`]
//! API.
//! It places no constraints on the keys other than [`PartialEq`].
//!
//! ```rust
//! use assoc::AssocExt;
//!
//! #[derive(PartialEq, Debug)]
//! enum MyKey {
//!     A,
//!     B,
//!     C,
//! }
//!
//! let mut map = vec![(MyKey::A, 1), (MyKey::B, 2)];
//! map.entry(MyKey::C).or_insert(3);
//! assert_eq!(map.get(&MyKey::C), Some(&3));
//! map.entry(MyKey::A).and_modify(|e| *e += 1).or_insert(9);
//! assert_eq!(map.get(&MyKey::A), Some(&2));
//! ```
//!
//! ## Why?
//!
//! [`std::collections`] offers two maps: [`HashMap`] and [`BTreeMap`].
//! `HashMap` requires that its keys implement [`Hash`] and [`Eq`] while `BTreeMap` requires that
//! its keys implement [`Ord`] and [`Eq`].
//! `Hash` and `Ord` enable efficient implementations of mapping data structures; however, they are not
//! necessary for general mapping data structures.
//! `std::collections` falls short when working with keys that cannot implement `Hash` or `Ord`.
//!
//! For example, the following snippet fails to compile because `Hash` is not implemented for `MyKey`:
//!
//! ```compile_fail
//! use std::collections::HashMap;
//!
//! #[derive(PartialEq, Eq, Debug)]
//! enum MyKey {
//!     A,
//!     B,
//!     C,
//! }
//!
//! let map = vec![(MyKey::A, 1), (MyKey::B, 2)]
//!     .into_iter()
//!     .collect::<HashMap<MyKey, u64>>();
//! ```
//!
//! Strictly speaking, a map's keys should implement `Eq`, which is why this crate provides a
//! [`AssocStrictExt`] as well.
//! This trait extension behaves like `AssocExt` but requires `K: Eq`.
//!
//! [`HashMap`]: std::collections::HashMap
//! [`BTreeMap`]: std::collections::BTreeMap
//! [`Entry`]: vec::Entry
pub mod vec;

pub use vec::{AssocExt, AssocStrictExt};
