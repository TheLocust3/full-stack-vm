# Overview

A simple stack based language that will be used to create a lisp-style language

## Examples

Example of calling a lambda:

```
(push (num 1))
(push (thunk ((lam x ((push x))))))
(call)
```

Results in the stack being ((num 1))
