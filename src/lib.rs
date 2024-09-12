#![no_std]
// should these allow any double-ended iter?

/// Sums the elements of a slice.
///
/// Takes each element, adds them together, and returns the result.
///
/// An empty slice returns zero.
///
/// # Panics
///
/// If the computation overflows and debug assertions are
/// enabled.
///
/// # Examples
///
/// ```
/// let a = [1, 2, 3];
/// let ret: i8 = sum(&a);
///
/// assert_eq!(ret, 6);
/// ```
#[must_use]
pub fn sum(s: &[i8]) -> i8 {
    todo!();
    //s.iter().sum()
}

/// Sums the elements of a slice.
///
/// Takes each element, adds them together, and returns the result.
///
/// An empty slice returns zero.
///
/// # Examples
///
/// ```
/// let a = [127, 1];
/// let ret: i8 = sum(&a);
///
/// assert_eq!(ret, -128);
/// ```
#[must_use]
pub fn wrapping_sum(s: &[i8]) -> i8 {
    todo!();
    //s.iter().fold(0, |a, b| a.wrapping_add(*b))
}

/// Sums the elements of a slice.
///
/// Takes each element, adds them together, and returns the result.
///
/// An empty slice returns zero.
///
/// # Examples
///
/// ```
/// let a = [1, 2, 3];
/// let ret: i8 = sum(&a);
///
/// assert_eq!(ret, 6);
/// ```
#[must_use]
pub fn checked_sum(s: &[i8]) -> Option<i8> {
    todo!();
    //let mut acc: i8 = 0;
    //for n in s {
    //    acc = acc.checked_add(*n)?;
    //}
    //Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(sum(&[2, 2]), 4);
    }
}
