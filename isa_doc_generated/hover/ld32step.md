* `ld32step $mDst0, $mBase0, $mDelta0+=, simm8`
* `ld32step $mDst0, $mBase0, $mDelta0+=, $mStride0`
* `ld32step $aDst0, $mBase0, $mDelta0+=, simm8`
* `ld32step $aDst0, $mBase0, $mDelta0+=, $mStride0`

Naturally aligned single *word* load with scaled post-incrementing
address.

Destination register-file: MRF or ARF

Effective address formed from:

:   -   Base address (`$m` register)
    -   Unsigned address delta (`$m` register)

Data format:

:   -   Result is an unmodified 32-bit value.

Address auto-increment:

:   -   The unsigned address delta MRF register operand is
        post-incremented by the signed immediate or stride register
        (after the value has been scaled to atom size).
