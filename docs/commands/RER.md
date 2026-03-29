**@RER** (Register Error Routine)

Registers a routine to be executed if the script encounters an error. Since all variables, temporary results, and the current output area remain unaltered and accessible to the error routine, the routine can be used to help debug the script.

**Format**

```
@RER[,c,d,r] lab .
@RER[,c,d,r] lab[,opts] . 2200 Only
```

**Parameters**

`c,d,r` - Report containing the external routine. Defaults to the current `c,d,r`.
`lab` - Label in the report where the error routine begins. Use `LIN1` to begin at the first line of the external run control report.
`opts` - (2200 Only) Diagnostic options.

---
