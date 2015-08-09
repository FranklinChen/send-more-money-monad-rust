//! Monad syntax `mdo!` corresponding to Haskell [`do`](https://en.wikibooks.org/wiki/Haskell/do_notation) syntax.

/// mdo! block introduces:
///
/// - ... `=<<` ...
/// - `when` ...
///
/// It uses:
///
/// - `flat_map`
/// - `ret`
/// - `mzero`
///
/// Modified from [`rust-mdo`](https://github.com/TeXitoi/rust-mdo).
macro_rules! mdo {
    (
        let $p: pat = $e: expr ; $( $t: tt )*
    ) => (
        { let $p = $e ; mdo! { $( $t )* } }
    );

    (
        let $p: ident : $ty: ty = $e: expr ; $( $t: tt )*
    ) => (
        { let $p: $ty = $e ; mdo! { $( $t )* } }
    );

    (
        $p: pat =<< $e: expr ; $( $t: tt )*
    ) => (
        $e.flat_map( move |$p| mdo! { $( $t )* } )
    );

    (
        $p: ident : $ty: ty =<< $e: expr ; $( $t: tt )*
    ) => (
        $e.flat_map( |$p : $ty| mdo! { $( $t )* } )
    );

    (
        $e: expr ; $( $t: tt )*
    ) => (
        $e.flat_map( move |_| mdo! { $( $t )* })
    );

    (
        when $e: expr ; $( $t: tt )*
    ) => (
        if !$e { mzero() } else { mdo! { $( $t )* } }
    );

    (
        ret $f: expr
    ) => (
        $f
    )
}
