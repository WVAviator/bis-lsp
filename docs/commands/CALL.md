**@CALL** — Call Subroutine

Save the contents of all currently defined variables and pass control to an internal or external subroutine.

**Format**

```
@CALL[,c,d,r] {lab|function} ([p,...,p]) .    Unregistered subroutine call
@CALL,"name" {lab|function} ([p,...,p])  .    Registered subroutine call
```

**Parameters**

`c,d,r` - Report containing the external subroutine.
`"name"` - Name of the registered subroutine. The subroutine name must be enclosed in double quotes.
`lab` - Label where the subroutine begins.
`function` - Name of the Javascript entry point function to be executed.
`p,...,p` - Parameters to pass to the subroutine.

Parameter Types

`v` — variable; system returns the new value from the subroutine.
`*v` — variable; system retains the original value from the calling run.
`'string'` — literal character string.
`RNM(num)` — sends and receives a rename report where `num` is a rename number from `-1` to `-MAXRNM$`.
`RNMSND(num)` — only sends a rename report where `num` is a rename number from `-1` to `-MAXRNM$` or `0` to specify a void report.
`RNMRCV(num)` — only receives a rename report where `num` is a rename number from `-1` to `-MAXRNM$`. The caller sends a void report identifier to the subroutine.
