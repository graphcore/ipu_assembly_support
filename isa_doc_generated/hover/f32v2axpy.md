* `f32v2axpy $aDst0:Dst0+1, $aSrc0:Src0+1, $aSrc1:Src1+1`

*Single-precision* 2-element vector **z** = *a***x** + **y** The scalar
multiplicand *a* is provided by the internal state element `$TAS`.

Results are stored within the accumulator state. Destination registers
are written with the previous accumulator state.
