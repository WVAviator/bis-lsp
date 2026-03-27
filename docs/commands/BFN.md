**@BFN** — Binary Find

Find character strings quickly in one or more large reports or a drawer or to count occurrences of character strings.

Each field or partial field being searched must be sorted in ascending order.

**Format**

```
@BFN,c,d[,r,l,lab] o cc ltyp,p [vrpt,vlno] .
```

**Parameters**

`c,d,r` - Report or drawer to scan.
`l` - Number of the data line on which to start the first sample, to speed up the search.
`lab` - Label to go to if no finds are made. If you do not specify a label and no finds are made, the run continues.
`o` - Options field.
`cc` - Column-character positions or names of the fields to scan.
`ltyp` - Line type to scan. If you specify the A option, leave this subfield blank but enter the comma.
`p` - Character string to find.
`vrpt` - Variable to capture the number of the report where the find was made.
`vlno` - Variable to capture the number of the line where the find was made.

---
