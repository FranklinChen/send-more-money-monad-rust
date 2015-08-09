//! `Iterator` as a monad.
//!
//! Based on [`rust-mdo`](https://github.com/TeXitoi/rust-mdo/blob/master/src/lib.rs)

use std::option;

/// Corresponds to Haskell [`return`](https://hackage.haskell.org/package/base-4.8.1.0/docs/Control-Monad.html#v:return)
#[inline]
pub fn ret<T>(x: T) -> option::IntoIter<T> {
    Some(x).into_iter()
}

/// Corresponds to Haskell [`mzero`](https://hackage.haskell.org/package/base-4.8.1.0/docs/Control-Monad.html#v:mzero)
#[inline]
pub fn mzero<T>() -> option::IntoIter<T> {
    None.into_iter()
}

/// Corresponds to Haskell [`guard`](https://hackage.haskell.org/package/base-4.8.1.0/docs/Control-Monad.html#v:guard)
#[inline]
pub fn when<F, T>(b: bool, f: F) -> option::IntoIter<T>
    where F: FnOnce() -> option::IntoIter<T>
{
    if !b {
        mzero()
    } else {
        f()
    }
}
