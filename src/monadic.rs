//! Monadic implementation.

use types::Solution;
use utils::{to_number, is_in};

// Use the `Iterator` monad.
use iter_monad::{ret, when};

/// Compute all solutions to `SEND+MORE=MONEY`, monadically.
///
/// Use `Iterator` as monad instead of using `for` loops,
/// and replace `if` with the function `when`.
///
/// Note that we really compute an `Iterator`, and could return that,
/// but for simplicity, we collect into a `Vec` for consistency with
/// the imperative API.
///
/// `flat_map` corresponds to Haskell's [`>>=`](https://hackage.haskell.org/package/base-4.8.1.0/docs/Control-Monad.html#v:-62--62--61-) operator as `flat_map`.
///
/// # Examples:
///
/// ```
/// # use send_more_money::constants::DIGITS;
/// # use send_more_money::monadic::solutions;
/// assert_eq!(solutions(&DIGITS), vec![(9567, 1085, 10652)]);
/// ```
pub fn solutions(digits: &[usize]) -> Vec<Solution> {
    let iter = digits.iter().cloned()
        .filter(move |&z| !is_in(z, &[0])).flat_map(move |s|
    digits.iter().cloned()
        .filter(move |&z| !is_in(z, &[s])).flat_map(move |e|
    digits.iter().cloned()
        .filter(move |&z| !is_in(z, &[s,e])).flat_map(move |n|
    digits.iter().cloned()
        .filter(move |&z| !is_in(z, &[s,e,n])).flat_map(move |d| {

    let send = to_number(&[s,e,n,d]);

    digits.iter().cloned()
        .filter(move |&z| !is_in(z, &[0,s,e,n,d])).flat_map(move |m|
    digits.iter().cloned()
        .filter(move |&z| !is_in(z, &[s,e,n,d,m])).flat_map(move |o|
    digits.iter().cloned()
        .filter(move |&z| !is_in(z, &[s,e,n,d,m,o])).flat_map(move |r| {

    let more = to_number(&[m,o,r,e]);

    digits.iter().cloned()
        .filter(move |&z| !is_in(z, &[s,e,n,d,m,o,r])).flat_map(move |y| {

    let money = to_number(&[m,o,n,e,y]);

    when(send + more == money, ||

    ret((send, more, money))

    )})})))}))));

    iter.collect()
}
