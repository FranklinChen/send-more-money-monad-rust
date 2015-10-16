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

We implement a monad instance for `Iterator`.

Advantages over imperative approach:

- no need for `for` or `if` control structures
- we can return an `Iterator` to **incrementally ask for one solution at
  a time**, rather than return a whole `Vec` at once

Disadvantages:

- the code is unpleasant to type and look at
- there is risk of stack overflow, because Rust is not designed
  for this style of programming
- compilation of all the nested closures is very slow because of Rust's
monomorphism

### Macro-based monadic syntax

This is identical to the explicit monadic approach, except that we use
Rust's macro system to provide a monadic syntax looking like what
Haskell, Scala, and F# have built-in.

We take ideas from the macro library [`mdo`](https://github.com/TeXitoi/rust-mdo). It's
worth looking at the
[source code](https://github.com/TeXitoi/rust-mdo/blob/master/src/lib.rs)
to see what the `mdo!` macro does. (The reason we are not using the
library as it is, is that it creates overhead that results in stack overflow.)

Advantages:

- much cleaner to type and read

Disadvantages:

- potentially confusing to those who don't understand what the macros do

## Performance

The imperative solution is obviously a bit faster than the monadic
one, about 37% faster.

```
$ cargo bench
test bench_imperative_solutions     ... bench:  54,804,559 ns/iter (+/- 2,309,645)
test bench_monadic_solutions        ... bench:  74,392,854 ns/iter (+/- 3,183,683)
test bench_monadic_syntax_solutions ... bench:  75,067,433 ns/iter (+/- 4,109,869)
```
