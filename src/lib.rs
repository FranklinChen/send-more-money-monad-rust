//! Solve SEND+MORE=MONEY, using Vec<_> as monad with guard.
//!
//! Inspired by mjd's [blog post](http://blog.plover.com/prog/monad-search-2.html)

// Requires nightly.
#![feature(test)]

extern crate test;

pub mod utils;

pub mod types;
pub mod constants;

pub mod imperative;

pub mod vec_monad;
pub mod monadic;

#[macro_use]
pub mod monad_macro;

pub mod monadic_syntax;
