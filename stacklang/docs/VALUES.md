# Values

### variable

A symbol that represents a bound variable

Examples

- (var x)
- (var y)
- (var test)

### number

A unsigned 64 bit integer

Examples:

- (num 10)
- (num 11)

### thunk

A full stacklang program that can be pushed onto the stack

Examples:

- (thunk ((push (num 10)) (push (num 11)) (add)))
