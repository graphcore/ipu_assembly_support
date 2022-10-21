* `ld128step $aDst0:Dst0+3, $mBase0, $mDelta0+=, simm8`
* `ld128step $aDst0:Dst0+3, $mBase0, $mDelta0+=, $mStride0`

Naturally aligned 128-bit load from interleaved memory region with
scaled post-incrementing address.

Destination register-file: ARF only

Effective address formed from:

:   -   Base address (`$m` register)
    -   Unsigned address delta (`$m` register)

Data format:

:   -   Result is an unmodified 128-bit value stored in a naturally
        aligned register-quad.

Address auto-increment:

:   -   The unsigned address delta MRF register operand is
        post-incremented by the signed immediate or stride register
        (after the value has been scaled to atom size).
