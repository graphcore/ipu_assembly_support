# $DBG_DATA

CSR alias. This register can be configured (via `$DBG_ECSR`) to behave
in two different ways:

1.  A general purpose 32-bit data register for passing data into and
    out ofTile via the debug bus.
2.  To present the current value of `$PC` for any context.
