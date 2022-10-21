Writes to this register cause the following assignments to the \$PRNG
state:

-   `$PRNG_0_0` =value
-   `$PRNG_0_1` = \~value
-   `$PRNG_1_0` = (value \<\< 13) \| (\~value \>\> 19)
-   `$PRNG_1_1` = (\~value \<\< 13) \| (value \>\> 19)

Reads return 0.
