* `f16v2gina $aDst0, $aSrc0, zimm12`

Get and initialise accumulators.

-   Read a pair of internal accumulator values as *half-precision*
    values. Stochastic rounding applies as configured by
    `$FP_CTL.ESR`.
>
-   Convert 2-element vector of *half-precision* input values to
    single precision and write to internal accumulator state.
>
-   The instruction immediate specifies which pair of accumulator
    registers are to be read and written:
>
    > a.  Read **\$AACC**\[0\] and **\$AACC**\[2\], write
    >     **\$AACC**\[12\] and **\$AACC**\[14\]
    > b.  Read **\$AACC**\[1\] and **\$AACC**\[3\], write
    >     **\$AACC**\[13\] and **\$AACC**\[15\]
    >
    > and if and only if the platform supports 2 AMP sets:
    >
    > c.  Read **\$AACC**\[16\] and **\$AACC**\[18\], write
    >     **\$AACC**\[28\] and **\$AACC**\[30\]
    > d.  Read **\$AACC**\[17\] and **\$AACC**\[19\], write
    >     **\$AACC**\[29\] and **\$AACC**\[31\]
>
-   Propagate internal accumulator state such that all accumulator
    registers may be read and written via a sequence of this
    instruction.

zimm12 immediate format:

![f16v2gina immediate
format](images/autogen/GINA_IMMFLAGS.*){.align-center}
