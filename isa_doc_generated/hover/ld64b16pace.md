* `ld64b16pace $aDst0:Dst0+1, $aDst1, $mAddr0:Addr0+1+=, $mStride0, Strimm2x2`

Naturally aligned 64-bit and broadcast 16-bit load, with dual
independent post-incrementing addresses.

Destination register-file: ARF only

Effective addresses:

:   -   2 independent full load addresses
    -   provided directly from MRF as a register pair
    -   lower register provides 1st load address
    -   upper register provides 2nd load address

Data format:

:   -   Results are:
        -   1 unmodified 64-bit value stored in a naturally aligned
            register pair
        -   1 16-bit value broadcast (duplicated) into a single ARF
            register

Note that a TEXCPT_INVALID_OP exception will occur if the two
destination register pairs are not distinct.
