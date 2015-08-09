//! Monadic implementation: uses Rust macros to simplify
//! the look and feel of the code.

use types::Solution;
use utils::{to_number, is_in};

// Use the `Iterator` monad.
use iter_monad::{ret, mzero};

/// Compute all solutions to `SEND+MORE=MONEY`, monadically, using macros.
///
/// Note that we really compute an `Iterator`, and could return that,
/// but for simplicity, we collect into a `Vec` for consistency with
/// the imperative API.
///
/// # Examples:
///
/// ```
/// # use send_more_money::constants::DIGITS;
/// # use send_more_money::monadic_syntax::solutions;
/// assert_eq!(solutions(&DIGITS), vec![(9567, 1085, 10652)]);
/// ```
pub fn solutions(digits: &[usize]) -> Vec<Solution> {
    let result = mdo! {
        s =<< digits.iter().cloned()
            .filter(move |&z| !is_in(z, &[0]));
        e =<< digits.iter().cloned()
            .filter(move |&z| !is_in(z, &[s]));
        n =<< digits.iter().cloned()
            .filter(move |&z| !is_in(z, &[s,e]));
        d =<< digits.iter().cloned()
            .filter(move |&z| !is_in(z, &[s,e,n]));

        let send = to_number(&[s,e,n,d]);

        m =<< digits.iter().cloned()
            .filter(move |&z| !is_in(z, &[0,s,e,n,d]));
        o =<< digits.iter().cloned()
            .filter(move |&z| !is_in(z, &[s,e,n,d,m]));
        r =<< digits.iter().cloned()
            .filter(move |&z| !is_in(z, &[s,e,n,d,m,o]));

        let more = to_number(&[m,o,r,e]);

        y =<< digits.iter().cloned()
            .filter(move |&z| !is_in(z, &[s,e,n,d,m,o,r]));

        let money = to_number(&[m,o,n,e,y]);

        when send + more == money;
        ret ret((send, more, money))
    }.collect();

    result
}

#[cfg(test)]
mod test {
    use super::solutions;
    use test::Bencher;

    #[bench]
    fn bench_solutions(b: &mut Bencher) {
        b.iter(|| solutions(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]))
    }
}
