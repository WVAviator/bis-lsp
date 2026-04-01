**@LNY** - Yank Line

Copies lines from a report into a temporary buffer.

When executed:

- The specified lines are copied into the buffer.
- If the buffer does not exist, it is created.
- If more lines are requested than the report contains, all lines from the starting point to the end of the report are copied.

**Format**

```
@LNY,c,d,r,l[,q,b] .
```

**Parameters**

`c,d,r` - Report from which lines are to be copied.
`l` - Line number of the first line to copy.
`q` - Number of lines to copy. Default = `1`.
`b` - Buffer label from `1` through `100`. Default = unlabeled buffer.

---
