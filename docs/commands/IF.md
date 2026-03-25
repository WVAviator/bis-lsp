**@IF** — If Conditional

Tests the relationship between two or more values and specifies the statements to execute when the test condition is true or false.

**Format**

```
@IF[,o] val1 op val2 stmt1 [.] ; stmt2 . Basic format
@IF[,o] val1 op val2,val3[,val4,...,valn] stmt1 [.] ; stmt2 . Logical OR
@IF[,o] val1 op val2 & op val3 [& op val4 ... & op valn] stmt1 [.] ; stmt2 . Logical AND
@IF[,o] val1 op val2,(lab)[,val3,(lab),...,valn,(lab)] ; stmt1 . Computed GTO
```

**Parameters**

`o` - Options field.
`val1` - Data to compare. Specify as a variable, constant, literal, or reserved word.
`op` - Relational operator.
`val2` - Data to compare to `val1`. Specify as a variable, constant, literal, or reserved word.
`stmt1 [.]` - Statements to execute when the condition is true.
`stmt2 .` - Statements to execute when the condition is false.
`,val3,...,valn` - A comma (`,`) representing a logical OR, followed by additional comparison values.
`& op val3...& op valn` - An ampersand (`&`) representing a logical AND, followed by a relational operator and additional value.
`,(lab)` - Label at which to continue execution when the condition is true.

---
