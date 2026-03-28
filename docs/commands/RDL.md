**@RDL** (Read Line)

Reads a report line or segments of a line and loads variables with the data.

**Format**

```
@RDL,c,d,r,l[,lab] cc vdata .
```

**Parameters**

`c,d,r` - Report from which to read the line.
`l` - Line number to read.
`lab` - Label to go to if the line or report does not exist.
`cc` - Column-character positions or field names to read.
`vdata` - Variables to capture the data.

---
