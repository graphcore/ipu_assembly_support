* `f16v4istacc $aDst0:Dst0+1, $aSrc0:Src0+1, $aSrc1:Src1+1, enumFlags`

Sort/shuffle (permute) through accumulators, with new input.

-   Present 128-bits of register operand source data to be
    sorted/shuffled (other otherwise permuted) using the \$AACC state.
    The precise behaviour is dependent on the value of the immediate.
-   Perform \$AACC state propagation as specified by the immediate.
-   The destination register pair is written with 64-bits of result
    data from a combination of \$AACC registers. The precise
    combination is specified by the immediate.
