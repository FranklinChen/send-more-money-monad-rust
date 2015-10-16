#![feature(test)]

extern crate test;
extern crate send_more_money;

use test::Bencher;

use send_more_money::imperative;
use send_more_money::monadic;
use send_more_money::monadic_syntax;

const DIGITS: &'static [usize] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

#[bench]
fn bench_imperative_solutions(b: &mut Bencher) {
    b.iter(|| imperative::solutions(DIGITS))
}

#[bench]
fn bench_monadic_solutions(b: &mut Bencher) {
    b.iter(|| monadic::solutions(DIGITS))
}

#[bench]
fn bench_monadic_syntax_solutions(b: &mut Bencher) {
    b.iter(|| monadic_syntax::solutions(DIGITS))
}
