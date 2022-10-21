Auto-incrementingTile Memory delta offset for internal exchange incoming
messages. The address is incremented by 4-bytes for each new *word*
received. The effective address for the next internal exchange incoming
message is `$INCOMING_BASE` + `$INCOMING_DELTA`. An explicit write to
this register from `put` always wins over an auto-increment.
