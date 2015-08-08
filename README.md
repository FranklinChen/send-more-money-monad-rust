# Solve `SEND+MORE=MONEY` puzzle monadically in Rust

[![Build Status](https://travis-ci.org/FranklinChen/send-more-money-rust.png)](https://travis-ci.org/FranklinChen/send-more-money-rust)

Rust code corresponding to the Haskell, Perl, Python code in mjd's [blog post](http://blog.plover.com/prog/monad-search-2.html).

## Comparing different approaches

### Natural imperative approach

The imperative approach uses no fancy continuations or monads, but
instead relies on

- deeply nested `for` loops
- imperative `if` guards without `else`
- a single mutable result vector to append to in the innermost loop

This is a very natural approach for this particular problem and with
its hardcoded words. If the words were read in at runtime, the whole
approach falls apart because the number of `for` loops and `if` guards
would vary depending on the input.

### Explicit monadic approach

We implement a monad instance for `Vec`.

Advantages over imperative approach:

- no need for `for` or `if` control structures

Disadvantages:

- the code is unpleasant to type and look at
- compilation of all the nested closures is slow

### Macro-based monadic syntax

This is identical to the explicit monadic approach, except that we use
Rust's macro system to provide a monadic syntax looking like what
Haskell, Scala, and F# have built-in.

Advantages:

- much cleaner to type and read

Disadvantages:

- potentially confusing to those who don't understand what the macros do

## Performance

The imperative solution is obviously a bit faster than the monadic
one.

```
$ cargo bench
test imperative::test::bench_solutions     ... bench:  51,329,560 ns/iter (+/- 5,085,420)
test monadic::test::bench_solutions        ... bench:  81,393,211 ns/iter (+/- 20,765,764)
test monadic_syntax::test::bench_solutions ... bench:  85,343,092
ns/iter (+/- 7,851,590)
```
