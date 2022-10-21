* `ld2x64pace $aDst0:Dst0+1, $aDst1:Dst1+1, $mAddr0:Addr0+1+=, $mStride0, Strimm2x2`

Naturally aligned dual 64-bit load, with dual independent
post-incrementing addresses.

Destination register-file: ARF only

Effective addresses:

:   -   2 independent load addresses
    -   provided directly from MRF as a register pair
    -   lower register provides 1st load address
    -   upper register provides 2nd load address

Data format:

:   -   Results are 2 unmodified 64-bit values stored in 2 naturally
        aligned register pairs.

Note that a TEXCPT_INVALID_OP exception will occur if the two
destination register pairs are not distinct.
