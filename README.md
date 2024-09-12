# [RLE](https://en.wikipedia.org/wiki/Run-length_encoding) x [BS](https://en.wikipedia.org/wiki/Binary_search) x Math-means

> [!warning]
> This is an unimplemented and experimental crate!

This is a specialized algorithm. It requires:
- The list to be sorted
- The number of unique values should be (at most) `log(len)` for arithmetic-mean, and `len / 2` for geometric-mean. This is just for the overhead to be worthwhile.

The algorithm exploits the following lemmas (theorems?):
- `a + a = 2a`, `a + a + a ... = n*a`
- `a * a = a^2`, `a * a * a ... = a^n`
- Multiplication is faster than repeated addition
- [Bin exponentiation](https://en.wikipedia.org/wiki/Exponentiation_by_squaring) is faster than repeated multiplication
- Iterating over all elements of a list is `O(n)`, but finding values in a sorted list can be as fast as `O(log(n))`
