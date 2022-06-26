//! The prelude module automatically imports the most important functions and
//! structures into the Rust program.
//!
//! ## Prelude contents
//!
//! - crate::io::{[`read_file()`], [`write_file()`]}, functions for reading and writing files
//! - crate::merge::{[`merge()`], [`Key`], [`Settings`], [`Value`]}, the main function merge along with its relevant data structures
//! - crate::serde::{[`deserialize()`], [`serialize()`]}, serialization and deserialization functions to convert Strings into [`Settings`]

#[doc(no_inline)]
pub use crate::io::{read_file, write_file};

#[doc(no_inline)]
pub use crate::merge::{merge, Key, Settings, Value};

#[doc(no_inline)]
pub use crate::serde::{deserialize, serialize};
