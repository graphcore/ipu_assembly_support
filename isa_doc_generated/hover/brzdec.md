* `brzdec $mSrcDst0, zimm19`

::: spec
brzdec_instruction_0xaef46144e226b571

Conditional branch to absolute address with counter decrement. Branch
taken if and only if counter register value **is 0**. Counter value
decremented by 1 regardless of whether branch taken or not taken.
Immediate provides word-addressed absolute destination address.

## Note

This instruction considers the floating-point *single-precision* value
-0.0 to not be equal to zero (+0.0)
