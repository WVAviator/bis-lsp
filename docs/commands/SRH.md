**@SRH** - Search Reports

Finds a character string in specified fields of one or more reports in a drawer, creating a result containing all lines on which the string is found.

**Format**

```
@SRH,c,d[,r,l,q,lab] o cc ltyp,p [vlines,vls,vrpt] .
```

**Parameters**

`c,d,r` - Report or drawer to search (statement form).
`l` - Line number at which to start the search.
`q` - Number of lines to search.
`lab` - Label to go to if no finds are made.
`o` - Options field.
`cc` - Column-character positions or names of the fields to search.
`ltyp` - Line type to search, or blank for all line types.
`p` - Character string to find.
`vlines` - Variable to capture the number of lines found.
`vls` - Variable to capture the number of lines searched.
`vrpt` - Variable to capture the report number where the find was made.
