* `ldz16step $mDst0, $mBase0, $mDelta0+=, simm8`
* `ldz16step $mDst0, $mBase0, $mDelta0+=, $mStride0`

Zero-extending, naturally aligned 16-bit load with scaled
post-incrementing address.

Destination register-file: MRF only

Effective address formed from:

:   -   Base address (`$m` register)
    -   Unsigned address delta (`$m` register)

Data format:

:   -   Result is a 32-bit value formed by zero-extending the 16-bit
        data value.

Address auto-increment:

:   -   The unsigned address delta MRF register operand is
        post-incremented by the signed immediate or stride register
        (after the value has been scaled to atom size).
