* `ldst64pace $aDst0:Dst0+1, $aSrc0:Src0+1, $mAddr0:Addr0+1+=, $mStride0, Strimm2x2`

Naturally aligned 64-bit load and simultaneous 64-bit store, with dual
independent post-incrementing addresses.

Destination register-file: ARF only

Source register-file: ARF only

Effective addresses:

:   -   independent load and store addresses
    -   provided directly from MRF as a register pair
    -   lower register provides load address
    -   store address is split across the upper-bits of both registers
        (see `tapack`)

Data format:

:   -   Load result is an unmodified 64-bit value stored in a naturally
        aligned register pair.
