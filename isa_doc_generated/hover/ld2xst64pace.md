* `ld2xst64pace $aDst0:Dst0+3, $aSrc0:Src0+1, $mAddr0:Addr0+1+=, $mStride0, Strimm3x2`

Naturally aligned dual 64-bit load and simultaneous 64-bit store, with 3
independent post-incrementing addresses.

Destination register-file: ARF only

Source register-file: ARF only

Effective addresses:

:   -   3 independent addresses provided directly from MRF, packed into
        a register pair

Data format:

:   -   Load results are 2 unmodified 64-bit values stored in a
        naturally aligned register quad.
