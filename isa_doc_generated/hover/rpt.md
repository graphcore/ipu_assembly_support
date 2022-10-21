* `rpt $mSrc0, zimm8`
* `rpt zimm12, zimm8`

*rpt* provides a zero-overhead loop facility, causing the subsequent
sequence of *Execution Bundle*s (the repeat-body) to be executed
repeatedly. The repeat-count can be provided as an immediate or as an
unsigned register source value. The size of the repeat-body is expressed
in whole Execution Bundles and provided by an immediate (with the
repeat-body size being (immediate + 1) *Execution Bundle*s). Note that
it is not possible to execute solo instructions within a repeat-body. A
*exception* will be raised in an attempt to execute a solo instruction.

If the repeat-count is zero initially, *rpt* will act as a branch over
the repeat-body. Otherwise, the subsequent repeat-body *Execution
Bundle*s will be executed repeat-count times.

Any instruction co-issued with *rpt* is executed only once, and is not
part of the repeat-body.

`control` and `system` instructions cannot be executed within the
repeat-body. A exception will be raised in an attempt to execute any
such instruction within the body of *rpt*.

Exceptions raised during the execution of the repeat-body will always be
treated as malign, regardless of the underlying exception type
(including Debug exceptions). When such exceptions arise, `$WSR.ERPT` is
set to 0b1 to indicate that the event is unrecoverable.
