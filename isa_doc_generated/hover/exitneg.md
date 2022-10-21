* `exitneg $mSrc0`

Terminate current execution of a *Worker* thread and return a Boolean
exit status to the *Supervisor* thread. This instruction passes control
from a *Worker* thread to the *Supervisor* thread. The currently
allocated thread execution slot is returned to the *Supervisor*, which
may reassign the execution slot to another task.
