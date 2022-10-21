* `st32step $mSrc0, $mBase0, $mDelta0+=, simm8`
* `st32step $aSrc0, $mBase0, $mDelta0+=, simm8`
* `st32step $aSrc0, $mBase0, $mDelta0+=, $mStride0`

Naturally aligned 32-bit store with scaled post-incrementing address.

Source register-file: MRF or ARF

Effective address formed from:

:   -   Base address (`$m` register)
    -   Unsigned address delta (`$m` register)

Address auto-increment:

:   -   The unsigned address delta MRF register operand is
        post-incremented by the signed immediate or stride register
        (after the value has been scaled to atom size).
