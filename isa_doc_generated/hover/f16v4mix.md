* `f16v4mix $aDst0:Dst0+1, $aSrc0:Src0+1, $aSrc1:Src1+1`

*Half-precision* 4-element vector **z** = *a***x** + *b***y**. The
scalar multiplicands *a* and *b* are provided by the internal state
element `$TAS`.

Results are stored within the accumulator state. Destination registers
are written with the previous accumulator state.
