//! The `walrus` WebAssembly transformations library.

#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

mod arena_set;
pub mod const_value;
pub mod dot;
mod emit;
mod encode;
pub mod error;
pub mod ir;
mod map;
pub mod module;
mod parse;
pub mod passes;
pub mod ty;
