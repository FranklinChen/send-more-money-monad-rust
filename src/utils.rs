//! Utilities for the project.

/// Convert iterator of base 10 digits into a number.
#[inline]
pub fn iter_to_number<'a, Iter>(iter: Iter) -> usize
    where Iter: Iterator<Item = &'a usize>
{
    iter.fold(0, |n, d| 10*n + d)
}

/// Convert slice of base 10 digits into a number.
///
/// # Examples:
///
/// ```
/// # use send_more_money::utils::to_number;
/// assert_eq!(to_number(&[1, 2, 3]), 123);
/// ```
#[inline]
pub fn to_number(digits: &[usize]) -> usize {
    iter_to_number(digits.iter())
}

/// Return whether x is in the slice.
///
/// # Examples:
///
/// ```
/// # use send_more_money::utils::is_in;
/// assert!(is_in(5usize, &[2, 4, 5, 7]));
/// ```
#[inline]
pub fn is_in<T: PartialEq>(x: T, slice: &[T]) -> bool {
    slice.iter().any(|a| *a == x)
}
