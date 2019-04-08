# Values

## Register

Valid values:

- `A`
- `B`
- `C`
- `D`
- `E`
- `F`
- `HL`

## Value

An unsigned 64bit number

Examples:

- `10`
- `1421`

## Address

An unsigned 64bit number surrounded by parenthesis. Represents an address in memory.

Examples:

- `(10)`
- `(342)`

## Address Register

A register surrounded by parenthesis. Represents an address in memory based on the value in the register.

Valid Values:

- `(A)`
- `(B)`
- `(C)`
- `(D)`
- `(E)`
- `(F)`
- `(HL)`

## Label

A string with no spaces starting with a colon. Represents a local in the uncompiled assembly program. Currently only used to call functions.

Examples:

- `:test`
- `:a-test-call`
- `:function`
