**@FDR** — Find and Read Line

Searches for a character string and loads variables with information about the search. Use a @RLN (Read Line Next) or @RDL (Read Line) statement afterward to read the line containing the found string.

**Format**

```
@FDR,c,d[,r,l,q,lab] o cc ltyp,p [vrpt,vlno,vcab,vdrw] .
```

**Parameters**

`c,d,r` - Report in which to find a line to be read with `@RDL` or `@RLN`.
`l` - Line number at which to start the search.
`q` - Number of lines to search. Default = all lines.
`lab` - Label to branch to if no finds are made. Use `LIN1` to continue on the next line instead of a label number.
`o` - Options field.
`cc` - Column-character positions or names of the fields to search.
`ltyp` - Line type to search. If using the `A` option, leave this subfield blank but include the comma.
`p` - Find parameters.
`vrpt` - Variable to capture the report number where the first find is made.
`vlno` - Variable to capture the line number where the first find is made.
`vcab` - Variable to capture the cabinet number where the first find is made.
`vdrw` - Variable to capture the drawer number where the first find is made.

---
