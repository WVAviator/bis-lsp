**@RNM** (Rename)

Establishes a new reference for a report or result using names `-0` through `-16` in any order. Useful for temporarily saving multiple results — if a result is not renamed, the system releases it when a new `-0` result is created.

**Format**

```
@RNM[,c,d,r] -n .
```

**Parameters**

`c,d,r` - Report to rename. If not specified, the current `-0` result is renamed.
`-n` - New name for the report or result. Valid range: `-0` through `-16`.

---
