# [RLE](https://en.wikipedia.org/wiki/Run-length_encoding) × [BS](https://en.wikipedia.org/wiki/Binary_search) = ⚡🚀

> [!warning]
> This is an unimplemented and experimental crate!

This is a specialized algorithm. It requires the list to be sorted. The number of unique values should be (at most) `log(len)` for sum-like fns (addition, bitwise XOR, arithmetic-mean), and `len / 2` for geometric-mean (or any other expensive reduction). This is just for the overhead to be worthwhile.

The algorithm exploits the following lemmas (theorems?):
- `a + a = 2a`, `a + a + a ... = n*a`
- `a * a = a^2`, `a * a * a ... = a^n`
- Multiplication is faster than repeated addition
- [Bin exponentiation](https://en.wikipedia.org/wiki/Exponentiation_by_squaring) is faster than repeated multiplication
- Iterating over all elements of a list is `O(n)`, but finding values in a sorted list can be as fast as `O(log(n))`

Even though the implementation will only be defined for `u8`s, I hope that the rough proof-of-concept can help optimize many other programs that deal with sorted values of any type.
