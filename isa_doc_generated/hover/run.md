* `run $mEntry0, $mVBase0, zimm16`

Launch a worker *thread*.

Allocate execution time and *context* state to the *thread* whose entry
point is given by the register operand \$mEntry0, using the vertex
address calculated by summing:

1.  the register operand \$mVBase0
2.  the 16-bit immediate offset zimm16 × 4
3.  the constant TMEM_REGION0_BASE_ADDR

(i.e. the address formed by adding the register value \$mVBase0 to the
scaled immediate offset zimm16 is relative to TMEM_REGION0_BASE_ADDR)

*Exception event*s will be raised for any of the following conditions:

-   \$mEntry0 is not 4-byte aligned
-   \$mVBase0 is not 4-byte aligned
-   \$mEntry0 is not a valid, executable address
