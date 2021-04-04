[![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]

# daylio

A simple Daylio diary parsing and utility library.

## Sample Usage

Parse a default configuration .csv file into entries.

```rust
let entries = daylio::parse("data.csv");
```

[ci]: https://github.com/Elinvynia/daylio/actions?query=workflow%3ARust
[ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/daylio/Rust/master?style=flat-square
[docs]: https://docs.rs/daylio
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
[crate-link]: https://crates.io/crates/daylio
[crate-version]: https://img.shields.io/crates/v/daylio.svg?style=flat-square

