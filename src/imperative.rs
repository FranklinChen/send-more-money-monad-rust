//! The intuitive imperative implementation.

use types::Solution;
use utils::{to_number, is_in};

/// Compute all solutions to `SEND+MORE=MONEY`, imperatively.
///
/// Use nested `for` loops with `if` guards. In the inner loop,
/// mutate a `Vec` of solutions seen so far. At the end, return the
/// `Vec` of all solutions.
///
/// # Examples:
///
/// ```
/// # use send_more_money::constants::DIGITS;
/// # use send_more_money::monadic::solutions;
/// assert_eq!(solutions(&DIGITS), vec![(9567, 1085, 10652)]);
/// ```
pub fn solutions(digits: &[usize]) -> Vec<Solution> {
    let mut result = vec![];

    for &s in digits { if !is_in(s, &[0]) {
    for &e in digits { if !is_in(e, &[s]) {
    for &n in digits { if !is_in(n, &[s,e]) {
    for &d in digits { if !is_in(d, &[s,e,n]) {
      let send = to_number(&[s,e,n,d]);
    for &m in digits { if !is_in(m, &[0,s,e,n,d]) {
    for &o in digits { if !is_in(o, &[s,e,n,d,m]) {
    for &r in digits { if !is_in(r, &[s,e,n,d,m,o]) {
      let more = to_number(&[m,o,r,e]);
    for &y in digits { if !is_in(y, &[s,e,n,d,m,o,r]) {
      let money = to_number(&[m,o,n,e,y]);
      if send + more == money {
          // Found one answer.
          //
          // A drawback of the imperative implementation is that we
          // have to put the answer somewhere in external mutable
          // state, because we are deep in a call stack and cannot
          // just yield one value and then resume later.
          result.push((send, more, money));
      }
    }}}}}}}}}}}}}}}}

    result
}
