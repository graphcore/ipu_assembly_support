* `ldz16 $mDst0, $mBase0, $mDelta0, $mOff0`
* `ldz16 $mDst0, $mBase0, $mDelta0, zimm12`

Load and zero-extend a single, naturally aligned 16-bit quantity
fromTile Memory.

Destination register-file: MRF only

Effective address formed from:

:   -   Base address (`$m` register)
    -   Unsigned address delta (`$m` register)
    -   Unsigned scaled offset (`$m` register or immediate)

Data format:

:   -   Result is a 32-bit value formed by zero-extending the 16-bit
        data value.
