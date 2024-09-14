# [RLE](https://en.wikipedia.org/wiki/Run-length_encoding) × [BS](https://en.wikipedia.org/wiki/Binary_search) = ⚡🚀

> [!warning]
> This is an unimplemented and experimental crate!

This is a specialized algorithm: It requires the list to be sorted.

For the overhead to be worthwhile, the number of unique values should be (at most) `log(len)` (maybe `√len`?) for sum-like fns (addition, bitwise XOR, arithmetic-mean), and `len / 2` for geometric-mean (or any other expensive reduction).

The algorithm exploits the following lemmas (theorems?):
- `a + a = 2a`, `a + a + a ... = n*a`
- `a * a = a^2`, `a * a * a ... = a^n`
- Multiplication is faster than repeated addition
- [Bin exponentiation](https://en.wikipedia.org/wiki/Exponentiation_by_squaring) is faster than repeated multiplication
- Iterating over all elements of a list is `O(n)`, but finding values in a sorted list can be as fast as `O(log(n))`

Even though the implementation will only be defined for `u8`s, I hope that the rough proof-of-concept can help optimize many other programs that deal with sorted values of any type. Such as a [specialized compressor](https://discord.com/channels/244230771232079873/504289362880495616/1284115409146220657), or a floating-point approximation (similar values are considered as identical repeats, for performance)

## Informal algorithm
The following is a non-rigorous description (or prescription?). Ambiguities are here because this is just a rough idea of what's on my mind:

Since the list is assumed to be sorted, and the operators are assumed to be commutative and associative, the algorithm can start at any index:
- If we choose exponential-search, `i := 0`, `tmp := ls[i]`, `i++`.
- If we choose binary-search, `i := ls.len // 2`.

For simplicity, let's choose exp-s. We now enter the main (outer) loop.

If `ls[i*2] > tmp`, then we've overshoot, so we do a bin-s to find the last instance of `tmp` within `ls` (we know it must be between `i` and `i*2`).

Once we find the index of the last `tmp`, assign it to `i_tmp`. Now, `i_tmp - i + 1` is the exact number of `occurrences` of `tmp`.

If we're doing a summation, now we can simply `accumulator += tmp * occurrences` (assuming no overflow occurs)

We can repeat this process for every unique value in `ls`.

Another detail that can optimize this further, is keeping track of any "secondary" new values we encounter while bin-searching `tmp`. For example:

`ls := [0,0,1,1,1,1]`
`tmp == 0`, overshoot to index 3. Now we can remember that there are at least 2 instances of `1`, _even before we set it as our target_, just because we happen to visit it while searching for `0`. ~That may require a hash-map, as we may encounter multiple unique values in our way to find `tmp`. Since my impl only deals with `u8`, we don't need a `HashMap<u8, usize>`, a good-ol' `[usize; u8::MAX as usize + 1]` will do.~ This optimization only applies if the secondary value shares a boundary with our target, that is, there _aren't multiple boundaries_ (such as \[0,0,1,1,2,2,3,3,4,4])
