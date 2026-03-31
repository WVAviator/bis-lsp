**@LCV** — Locate and Change Variable

Finds and optionally replaces data within a variable. You can:

- Locate a string
- Change a string
- Count the number of occurrences
- Compare two variables literally

**Format**

```
@LCV[,lab] o v tgtstr[/replstr vpos,voccs] .
```

**Parameters**

`lab` - Label to continue execution at when no finds are made, or when `n` occurrences are not found (see `Bn` option). Use the `N` option to instead branch to the label when a find _is_ made.
`o` - Option controlling how the variable is searched.
`v` - Variable to search. Any type is accepted; substrings and array members are supported.
`tgtstr` - The target string to locate within the variable. Can be a variable, literal, constant, or reserved word. The transparent character acts as a wildcard (default: space). Use the `Tx` option to change the transparent character when you need to locate spaces.
`replstr` - Replacement data (0 to maximum line length) to substitute for the target string. Omitting this performs a locate only. To remove the target string entirely, specify nothing after the `/` delimiter (e.g. `target/`).
`vpos` - Variable to capture the character position of the first (or nth, with `Bn`) occurrence of the target string within the variable.
`voccs` - Variable to capture the total number of occurrences of the target string. Useful when using the `B` option for counting.

---
