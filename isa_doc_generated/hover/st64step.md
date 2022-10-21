* `st64step $aSrc0:Src0+1, $mBase0, $mDelta0+=, simm8`
* `st64step $aSrc0:Src0+1, $mBase0, $mDelta0+=, $mStride0`

Naturally aligned 64-bit store with scaled post-incrementing address.

Source register-file: ARF only (a naturally aligned register-pair)

Effective address formed from:

:   -   Base address (`$m` register)
    -   Unsigned address delta (`$m` register)

Address auto-increment:

:   -   The unsigned address delta MRF register operand is
        post-incremented by the signed immediate or stride register
        (after the value has been scaled to atom size).
