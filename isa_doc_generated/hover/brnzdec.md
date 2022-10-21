* `brnzdec $mSrcDst0, zimm19`

Conditional branch to absolute address with counter decrement. Branch
taken and counter value decremented by 1 if and only if counter register
value **is not 0**. Immediate provides word-addressed absolute
destination address.

## Note

This instruction considers the floating-point *single-precision* value
-0.0 to not be equal to zero (+0.0)
