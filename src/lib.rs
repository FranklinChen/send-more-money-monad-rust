//! Solve SEND+MORE=MONEY, using `Iterator` as monad with guard.
//!
//! Inspired by mjd's [blog post](http://blog.plover.com/prog/monad-search-2.html)

// Requires nightly.
#![feature(test)]

extern crate test;

// For monadic syntax.
#[macro_use]
pub mod monad_macro;

pub mod utils;

pub mod types;
pub mod constants;

pub mod imperative;

pub mod iter_monad;
pub mod monadic;
pub mod monadic_syntax;
