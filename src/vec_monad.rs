//! `Vec` as a monad.

/// Corresponds to Haskell [`return`](https://hackage.haskell.org/package/base-4.8.1.0/docs/Control-Monad.html#v:return)
#[inline]
pub fn mreturn<T>(x: T) -> Vec<T> {
    vec![x]
}

/// TODO

/// Either bail out completely or continue on.
///
/// Corresponds to Haskell [`guard`](https://hackage.haskell.org/package/base-4.8.1.0/docs/Control-Monad.html#v:guard)
#[inline]
pub fn guard<F, T>(b: bool, f: F) -> Vec<T>
    where F: FnOnce() -> Vec<T>
{
    if !b {
        mzero()
    } else {
        f()
    }
}

/// Corresponds to Haskell [`mzero`](https://hackage.haskell.org/package/base-4.8.1.0/docs/Control-Monad.html#v:mzero)
#[inline]
pub fn mzero<T>() -> Vec<T> {
    vec![]
}
