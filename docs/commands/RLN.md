**@RLN** (Read Line Next)

Continues reading from the report last accessed by a `@RDL` (Read Line) or `@FDR` (Find and Read Line) statement.

**Format**

```
@RLN[,l,lab] cc vdata .
```

**Parameters**

`l` - Line number to read. Default = next line number.
`lab` - Label to go to if the specified line does not exist.
`cc` - Column-character positions or field names to read.
`vdata` - Variables to capture the data. Maximum = 80 variables.

---
