* `cmpslt $mDst0, $mSrc0, $mSrc1`
* `cmpslt $mDst0, $mSrc0, simm16`

*Less than* comparison of two **signed** source values. Destination
register is set to 1 if the first source operand is less than the
second. Otherwise the destination register is set to 0. The comparison
operation is **signed**.
