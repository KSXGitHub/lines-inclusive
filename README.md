# lines-inclusive

Split a string into multiple lines, every line may end with `\n`

## Example

```rust
use lines_inclusive::LinesInclusive;

let lines: Vec<_> = "ABC\nDEF\nGHI".lines_inclusive().collect();
assert_eq!(lines, ["ABC\n", "DEF\n", "GHI"]);
```

## License

[MIT](https://github.com/KSXGitHub/lines-inclusive/blob/master/LICENSE.md) © [Hoàng Văn Khải](https://github.com/KSXGitHub/)
