* `lds8 $mDst0, $mBase0, $mDelta0, $mOff0`
* `lds8 $mDst0, $mBase0, $mDelta0, zimm12`

Load and sign-extend a single, 8-bit quantity fromTile Memory.

Destination register-file: MRF only

Effective address formed from:

:   -   Base address (`$m` register)
    -   Unsigned address delta (`$m` register)
    -   Unsigned scaled offset (`$m` register or immediate)

Data format:

:   -   Result is a 32-bit value formed by sign-extending the 8-bit
        loaded data value.
