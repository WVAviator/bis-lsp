**@LDV** — Load Variable

Puts data into a new or existing variable, or reformats the contents of an existing variable.

**Formats**

```
@LDV[,o] v=vld[,v=vld,...,v=vld] .
@LDV,o v[,v,...,v] .
@LDV,Q rv=iv,n[(delim),rv=iv,n(delim),...] .
```

**Parameter:**

`o` — Option controlling how data is loaded. _(optional)_
`v` — Variable to load. Include type and size to define or redefine it.
`vld` — Data to put into the variable: literal, constant, variable, reserved word, or combination. _(Format 1)_
`Q` — Load variables based on delimited content. _(Format 3)_
`rv` — Receiving variable. _(Format 3)_
`iv` — Issuing variable (source of delimited data). _(Format 3)_
`n` — Number of delimiters in `iv` to skip before loading. Must be a literal. _(Format 3)_
`(delim)` — Delimiter character. Default: tab. Parentheses required if specified. _(Format 3, optional)_

See also: `@JUV` (Justify Variable) for numeric reformatting options.
