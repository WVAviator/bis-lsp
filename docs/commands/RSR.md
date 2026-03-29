**@RSR** — Run Subroutine

Execute an external or internal subroutine starting at a specified label. The system transfers control to a subroutine within the same run control report (an **internal subroutine**) or to one in another report (an **external subroutine**).

**Format**

```
@RSR{,c,d,r lab | lab} .
```

**Parameters**

`c,d,r` - Report containing the external subroutine. Omit for internal subroutines.
`lab` - Label where the subroutine starts.

---
