* `stm32step $mSrc0, $mBase0+=, $mStride0`

Naturally aligned 32-bit store from MRF with scaled post-incrementing
address.

Source register-file: MRF only

Effective address formed from:

:   -   Base address (`$m` register)

Address auto-increment:

:   -   The base address register operand is post-incremented by the
        signed stride register value (after the value has been scaled to
        atom size).
