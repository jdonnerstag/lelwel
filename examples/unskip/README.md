# lelwel-unskip

A small parser exercising the **unskip** feature with actual input. Both
`Newline` and `Whitespace` are declared as skip tokens in
[the grammar](src/unskip.llw), but selected rules explicitly consume them.

For example, `single a` must end with either a newline or a comma. Newlines
between `a`, `b`, and `c` in `sequence` are still skipped; the newline after
`c` is consumed as a terminator. The `multiple` rule explicitly accepts
either a newline or horizontal whitespace after `a`.

[The functional tests](tests/unskip.rs) cover required and optional tokens,
zero-or-more and one-or-more repetitions, sequences, nullable rule calls,
multiple unskipped token kinds, ordinary skipping, consecutive statements,
missing required tokens, and skipped terminators inside a repeated rule
(`lines_block`). A terminator consumes a comma or newline, followed by any
additional newlines; therefore a run of blank lines is one terminator.
Successful parses must have no diagnostics, the expected terminator nodes, and
a lossless CST.

This also allows `single a\n\n` to parse without requiring call-site-specific
unskip behavior: both newlines belong to the same `terminator` node. In
`lines_block`, multiple consecutive newlines likewise form one terminator node,
rather than one node per newline.

Run from the repository root:

```sh
cargo test -p lelwel-unskip
```
