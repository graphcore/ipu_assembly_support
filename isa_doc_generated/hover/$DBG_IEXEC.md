If the target *context* (given by `$DBG_IOWNER`) is a *Worker* context:

-   If the target context is *quiescent*, writes to this register will
    cause the value written to be be injected as an instruction into the
    instruction stream of the target *context*. Such writes will also
    cause the busy flag to be set to 0b1.
-   If the target *context* is not *quiescent*, writes to this register
    will result in the setting of to 0b1. The contents of will not be
    modified.

If the target context is the *Supervisor* context:

-   Writes to this register will cause the value written to be injected
    as an instruction into the instruction stream of the Supervisor
    context under the following conditions:
    -   The Supervisor context is in theExcepted state
    -   The Supervisor context is in any other *quiescent* state and
        RBRK is enabled for the Supervisor context
-   Otherwise, writes to this register will result in the setting of to
    0b1. The contents of will not be modified.

Values written to must constitute correctly formed, complete instruction
opcode values, including all field information. Note that the following
instructions cannot be injected into either the Supervisor or any Worker
context. Any attempt to inject any such instruction will result in being
set to 0b1 (the contents of will not be modified):

Injected instructions cannot raise BREAK *exception event*s. IBRKs,
DBRKs and RBRKs do not apply to injected instructions. Since `sync` and
instructions cannot be injected, break_on_sync and PBRKs also don\'t
apply. Similarly, post-execution exceptions ( which include exchange
parity errors, internal exchange receive errors, external exchange
receive errors and and memory parity/ECC errors raised by other
contexts) also don\'t apply to injected instructions. However, if an
injected instruction would result in a FAULT *exception event* (not
caused by any of the exceptions listed above) and the *context*:

-   Is not in theExcepted state, the appropriate (unrecoverable)
    *exception event* will be raised in the normal manner.
-   Is in theExcepted state, will be set to 0b1 (and the exception
    logging state may or may not be updated - implementation_specifics)

Once an injected instruction has retired will be cleared to 0b0.

## Note

Writes to will be ignored if any of the error flags in of the `$TDI_STS`
are set. Those flags can be cleared via `$TDI_CLR`.
