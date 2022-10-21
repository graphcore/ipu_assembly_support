* `runall $mEntry0, $mVBase0, zimm16`

Allocate execution time and context state to a batch of worker
*thread*s:

-   The total number of *thread*s launched is equal to the number of
    hardware *worker* contexts ()
>
-   All *thread*s use the same entry point (\$mEntry0)
>
-   The value of `$VERTEX_BASE` assigned to the first allocated
    *worker* is provided by the register \$mVBase0
>
-   The value of `$VERTEX_BASE` assigned to every other *worker* is:
>
    > -   \$mVBase0 + (*n* × zimm16 × 4) (\$n in {\[1, CTXT_WORKERS -
    >     1\]}\$)

*Exception event*s will be raised for any of the following conditions:

-   \$mEntry0 is not 4-byte aligned
-   \$mVBase0 is not 4-byte aligned
-   \$mEntry0 is not a valid, executable address
-   There are any active Worker contexts

If there are active Worker contexts, `$SSR.RAERR` will also be set to
0b1 and all active Workers will raise a exception during the retirement
of their next instruction.
