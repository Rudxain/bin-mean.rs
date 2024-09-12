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
#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn sum(s: &[i8]) -> i8 {
    let mut acc: i8 = 0;

    let mut i: usize = 0;
    let mut overshoot: usize;
    while i < s.len() {
        let n = s[i];
        i += 1;
        i <<= 1;
        todo!();
    }
    acc
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
    fn test_sum() {
        let s = [2, 2];
        assert_eq!(sum(&s), s.iter().sum());
    }
    #[test]
    fn test_wrap() {
        let s = [2, 2];
        assert_eq!(
            wrapping_sum(&s),
            s.iter().fold(0, |a, b| a.wrapping_add(*b))
        );
    }
}
