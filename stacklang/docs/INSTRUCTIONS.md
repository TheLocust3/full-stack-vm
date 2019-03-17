# Instructions

### push

Syntax: (push {VALUE})

Examples

- (push 10)

### add

Syntax: (add)

Examples

- (add)

### sub

Syntax: (sub)

Examples

- (sub)

### if0

Syntax: (if0 {PROGRAM} {PROGRAM})

Examples

- (if0 ((push 1) (push 2) (add)) ((push 2) (push 1) (sub)))

### call

Syntax: call

Examples

- (call)

### lam

Syntax: (lam {VARIABLE} {PROGRAM})

Examples

- (lam x ((push 10) (push x) (add)))
