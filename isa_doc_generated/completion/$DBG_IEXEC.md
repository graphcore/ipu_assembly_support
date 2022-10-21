# $DBG_IEXEC

Writes to this register must be correctly formed, complete instruction
opcode values, including all field information. Writes to this register
will cause the value written to be injected as an instruction into the
instruction stream of the context indicated by `$DBG_IOWNER`, assuming
that context is in a suitable state.
