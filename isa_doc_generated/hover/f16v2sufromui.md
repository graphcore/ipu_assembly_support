* `f16v2sufromui $aDst0, $aSrc0`

Symmetric, unbiased conversion from 2-element vector of unsigned 16-bit
integers to 2-element *half-precision* vector.

Each of the *half-precision* results lies within the range
\$\[-frac{1}{2}, frac{1}{2}\]\$ but can never be exactly 0. The minimum
result magnitude is \$frac{1}{2\^{17}}\$ (and therefore results can lie
within the denormalised number range for *half-precision*).

Note that this instruction can be combined with `urand32`/`urand64` to
produce random, uniformly distributed floating-point values.
