This CSR location represents a side-effecting write alias. A write to
this address:

-   updates `$INCOMING_MUX` with the value written
-   also results in a delayed write to the `$INCOMING_MUX` register of
    the paired *Tile* instance. The value written to the `$INCOMING_MUX`
    register of the paired Tile instance is that appropriate for an
    incoming 64-bit exchange from the specified Tile Id (see
    timpl_tpair_writepairedincoming_mux). See timpl_exchange_parameters
    for the delay value.
-   will result in a exception being raised by both *Tile* instances if
    the attempted update of the paired *Tile\'s* `$INCOMING_MUX`
    register coincides with a write to `$INCOMING_MUX` performed by that
    *Tile* instance.

## Note

Writes to this register may cause *exception event*s to be raised by the
paired *Tile* instance (see `$INCOMING_MUXPAIR`).
