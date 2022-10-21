* `ldb16 $aDst0, $mBase0, $mDelta0, $mOff0`
* `ldb16 $aDst0, $mBase0, $mDelta0, zimm12`

Load and broadcast a single, naturally aligned 16-bit quantity fromTile
Memory.

Destination register-file: ARF only

Effective address formed from:

:   -   Base address (`$m` register)
    -   Unsigned address delta (`$m` register)
    -   Unsigned scaled offset (`$m` register or immediate)

Data format:

:   -   Result is a 32-bit value formed by broadcasting (duplicating)
        the 16-bit data value.
