Every non-final iterator item ends with a line feed:

```rust
use lines_inclusive::LinesInclusive;
let lines: Vec<_> = "ABC\nDEF\nGHI".lines_inclusive().collect();
assert_eq!(lines, ["ABC\n", "DEF\n", "GHI"]);
```

If the text has a single trailing line feed, that line feed is part of the final iterator item:

```rust
use lines_inclusive::LinesInclusive;
let lines: Vec<_> = "ABC\nDEF\nGHI\n".lines_inclusive().collect();
assert_eq!(lines, ["ABC\n", "DEF\n", "GHI\n"]);
```

If the text has multiple extra trailing line feeds, these line feeds are their own iterator items:

```rust
use lines_inclusive::LinesInclusive;
let lines: Vec<_> = "ABC\nDEF\nGHI\n\n\n".lines_inclusive().collect();
assert_eq!(lines, ["ABC\n", "DEF\n", "GHI\n", "\n", "\n"]);
```

Handling of carriage returns:

```rust
use lines_inclusive::LinesInclusive;
let lines: Vec<_> = "ABC\r\nDEF\n\rGHI".lines_inclusive().collect();
assert_eq!(lines, ["ABC\r\n", "DEF\n", "\rGHI"]);
```

Empty lines are turned into lines of single line feed:

```rust
use lines_inclusive::LinesInclusive;
let lines: Vec<_> = "ABC\nDEF\n\nGHI\n".lines_inclusive().collect();
assert_eq!(lines, ["ABC\n", "DEF\n", "\n", "GHI\n"]);
```
