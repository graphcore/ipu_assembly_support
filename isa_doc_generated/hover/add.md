* `add $mDst0, $mSrc0, $mSrc1`
* `add $mDst0, $mSrc0, zimm16`
* `add $mDst0, $mSrc0, simm16`

Signed integer addition of 2 source register values, or 1 source
register and 1 immediate. Immediates may be *sign extended* or *zero
extended* to *word* width. No scaling of the source operands (register
or immediate) is performed.
