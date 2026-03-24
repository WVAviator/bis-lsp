**@GTO** - Go To Location

Branches unconditionally to another location in the run, or to another run control report in the same cabinet and drawer.

**Format**

```
@GTO {lab | END[,n,n,n] | LIN[+]n | LIN-n | RPX r} .
```

**Parameters**

`lab` - Label at which to continue execution. Specify the label number or a variable containing the label number.
`END` - Terminate the run and display the output area.
`END,n,n,n` - Same as `END`, but passes up to three integer status codes back to the calling run.
`LIN[+]n` - Jump forward `n` lines from the current line. Specify `n` as an integer or a variable containing an integer.
`LIN-n` - Jump backward `n` lines from the current line. Specify `n` as an integer or a variable containing the negative integer.
`RPX r` - Jump to run control report `r` in the same cabinet and drawer as the current run.

---
