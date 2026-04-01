# **@LNP** - Put Line

Copies lines from a temporary yank buffer to a report.

- Inserts the specified lines into the report, truncating or padding with spaces to match the line length of the current report.
- The report expands to receive the added lines.

**Format**

```
@LNP,c,d,r,lb4[,b] .
```

**Parameters**

`c,d,r` - Report to which lines are to be copied.
`lb4` - Line number preceding the line where the buffer lines will be placed.
`b` - Buffer label from 1–100 as specified by the `@LNY`, `@LNA`, or `@LND` statement. Default = unlabeled buffer.

---
