# Integration tests

## How to write a test for `Runtime::evaluate()`

Add a JavaScript file `scripts/test_name.js` or `modules/test_name.mjs` and put JavaScript
code:

```javascript
print(undefined); ///=undefined
```

The `print()` function will be registered into a runtime used for the test.  When it's called, its
first argument is stored into `Validator::actual_values` for later validation.

Expected values are given in special line comments starting with `///=`.
[`evalute.js`](./evaluate.js) collects the subsequent values in the special line comments as
expected values in ascending order of lines.  The expected values are used as template parameters
for [`evaluate.rs.njk`](./evalute.rs.njk) and rendered into `evalute.rs`.

An uncaught exception is expressed in a special line comment starting with `///!`. `evaluate.js`
also collects its value.

For testing `async` functions, use `///#<index>=<expected-value>` instead.  This set the expected
value at the specified position in the list of expected values.

When you modify a test, you **MUST** run `make codegen` in order to create or update `evalute.rs`.
Then, `make test`.
