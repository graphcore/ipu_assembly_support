* `lds8step $mDst0, $mBase0, $mDelta0+=, simm8`
* `lds8step $mDst0, $mBase0, $mDelta0+=, $mStride0`

Sign-extending 8-bit load with post-incrementing address.

Destination register-file: MRF only

Effective address formed from:

:   -   Base address (`$m` register)
    -   Unsigned address delta (`$m` register)

Data format:

:   -   Result is a 32-bit value formed by sign-extending the 8-bit data
        value.

Address auto-increment:

:   -   The unsigned address delta MRF register operand is
        post-incremented by the signed immediate or stride register.
