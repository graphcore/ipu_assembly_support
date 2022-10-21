* `exitz $mSrc0`

Terminate current execution of a *Worker* thread and return a Boolean
exit status to the *Supervisor* thread. This instruction passes control
from a *Worker* thread to the *Supervisor* thread. The currently
allocated thread execution slot is returned to the *Supervisor*, which
may reassign the execution slot to another task.

## Note

This instruction considers the floating-point *single-precision* value
-0.0 to not be zero (+0.0)
