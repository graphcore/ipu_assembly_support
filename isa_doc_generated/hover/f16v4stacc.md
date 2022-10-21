* `f16v4stacc $aDst0:Dst0+1, enumFlags`

Sort/shuffle (permute) through accumulators.

-   Perform \$AACC state propagation as specified by the immediate.
-   The destination register pair is written with 64-bits of result
    data from a combination of \$AACC registers. The precise
    combination is specified by the immediate.
