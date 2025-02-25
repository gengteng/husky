//! Core functionality for the Sycamore UI framework.
//!
//! This crate should not be used directly. Instead, use the `sycamore` crate which re-exports this
//! crate.
//!
//! # Feature Flags
//!
//! - `hydrate` - Enables the hydration API.

pub mod component;
pub mod generic_node;
#[cfg(feature = "hydrate")]
pub mod hydrate;
pub mod noderef_signal;
pub mod render;
pub mod view;
