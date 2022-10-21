Reset value is dependent on `$TILE_ID`. See timpl_tile_incomingmuxreset.

## Note

A *exception event* will be raised by this *Tile* if any of the
following conditions are met:

-   a write to this CSR (by the local *Tile*) collides with a write
    initiated by the neighbour Tile via its `$INCOMING_MUXPAIR`
    register.
-   the neighbour *Tile* instance attempts to write a value within the
    external exchange range via its `$INCOMING_MUXPAIR` register.
-   the neighbour *Tile* instance attempts to write a value within the
    internal exchange range (via its `$INCOMING_MUXPAIR` register)
    when the current value is within the external exchange range.
