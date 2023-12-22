# json-parser

reference: https://qiita.com/togatoga/items/9d600e20325775f09547

## Usage

```rust
let json = r#"
{
    "num": 2.71828,
    "name": [true, false, null, 3.14],
    "other": {
        "num": 3.14,
        "name": "pi"
    }
}
"#;

let value = Parser::new(Lexer::new(json).tokenize().unwrap())
    .parse()
    .unwrap();
JsonPrinter::print_json(&value);
```

```json
{
  "name": [
    true,
    false,
    null,
    3.14,
  ],
  "num": 2.71828,
  "other": {
    "name": "pie",
    "num": 3.14,
  },
}
```