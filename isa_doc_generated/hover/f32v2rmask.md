* `f32v2rmask $aDst0:Dst0+1, $aSrc0:Src0+1, $aSrc1`

*Single-precision* floating-point vector random mask.

The result is a masked version of the input vector, with each element of
the input being individually masked with the probability specified by
the bottom 17-bits of the 2nd input operand:

-   if \$aSrc1\[16\] == 1, no masking is applied (the result is a copy
    of the input vector)
-   else if \$aSrc1\[16:0\] == 0, the result is a zero vector
-   otherwise each element is individually unmasked with probability
    \$frac{\$aSrc1\[15:0\]}{65536}\$

PRNG is used by this instruction to generate 2 x 16-bit random values
from the discrete uniform distribution.
