* `ld128putcs zimm8`

Load a naturally aligned 128-bit quantity, from a memory region with an
*interleave factor* of at least 2 and write the value to the common
compute configuration space. The load address is provided by
`$CCCSLOAD`, which is automatically post-incremented by 16.
