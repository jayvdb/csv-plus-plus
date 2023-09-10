![github workflow](https://github.com/patrickomatic/csv-plus-plus/actions/workflows/rust.yml/badge.svg)

# csv++

csv++ is an extension of [CSV](https://en.wikipedia.org/wiki/Comma-separated_values) which allows
you to author spreadsheets in a text file then compile that to your target spreadsheet (Excel, 
Google Sheets or even back to CSV).

Since csv++ is a superset of CSV any CSV document is valid:

```csvpp
Sum,Column 1,Column 2,
=5*SUM(B2,C2),,,
```

However you can extract reusable variables and functions by making a code section at the top, 
separated from the cells with a `---`.  Here's the same spreadsheet but extracted out into
variables and functions:

```csvpp
# you can define variables with `:=`
multiplier := 5

# functions look like this and have a single expression as their body
fn my_fn(a, b)
  multiplier * SUM(a, b)

---
Sum             , Column 1, Column 2,
=my_fn(B2, C2)  ,         ,         ,
```

One more useful feature is the ability to bind variables to a cell.  You can use the `[[`/`]]`
modifier syntax on the cell you want to bind it on.

```csvpp
fn my_complex_fn(a, b)
  a * a + SQRT(b)

---
Complex function    , Column 1  , Column 2 ,
=my_complex_fn(a, b), [[var=a]] , [[var=b]],
```


## Expands

Another useful feature is to define a range of rows which expand out (either infinitely or by a
finite amount) in the compiled spreadsheet.  To specify one you use the row modifier syntax
which is similar to above, you just prefix it with `!`: `![[`/`]]`.

```csvpp
Product Name   , Quantity          , Price per Unit  , =SUM(D2:D12)
![[expand=10]] , [[var=quantity]]  , [[var=price]]   , =quantity * price
```

This will expand the second row and repeat it 10 times in the final spreadsheet.  If you wanted 
it to be repeated until the end of the spreadsheet just leave off the `=10` and specify it as 
`![[expand]]`.


## Variable Scoping

The variable scoping semantics are pretty unique because every function call is evaluated relative
to the cell where it is used.  As you've seen above you can use `[[var=...]]` to bind a variable
name to a given cell.  As an example of scoping semantics we'll use this csv++ template:

```csvpp
foo_from_code_section := 42

---
[[var=bar_outside_expand]] ,                         ,                 ,                     ,                       ,
![[expand=2]]bar           , [[var=bar_in_expand]]   , =bar_in_expand  , =bar_outside_expand , =foo_from_code_section,
```

which will compile to:

```csv
bar  ,     , =B2 , =A1 , =42
bar  ,     , =B3 , =A1 , =42
```

Breaking this down:

* `foo_from_code_section` - Is always `42` no matter where it is used.
* `bar_in_expand` - Since it is defined within an `![[expand]]`, it's value depends on the final
  row, which will be `B2` or `B3`
* `bar_outside_expand` - Will always be `A1`, pointing to the cell where it was defined.  There
  is no relative aspect to it since it's not defined in an `expand`.


## Builtin Functions & Variables

TODO

## Formatting

You can also specify basic cell formatting which will either apply for the entire row or just
for individual cells.  To apply formatting to individual cells use the `[[`/`]]` syntax:

```csvpp
[[format=bold/format=underline]]foo,[[fontsize=20]]bar,baz
```

and here is the same thing using short-hand:

```csvpp
[[f=b/f=u]]foo,[[fs=20]]bar,baz,
```

To apply formatting to the entire row you can use `![[`/`]]` at the beginning of the row

```csvpp
![[f=b/f=u]]foo,bar,baz,
```

For a full list of formatting features, take a look at the [language reference](docs/LANGUAGE_REFERENCE.md)

### Additional Reading

* [Installation](docs/INSTALL.md)
* [Language Reference](docs/LANGUAGE_REFERENCE.md)
* [All Features Example](tests/all_features.rs)
* [csv++ Examples](https://github.com/patrickomatic/csvpp-examples)
* [Changelog](docs/CHANGELOG.md)
* [Feature Support Matrix](docs/feature_matrix.csvpp)
