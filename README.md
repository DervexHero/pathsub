# pathsub

Subtract one path from another. Returns subtraction difference not the relative path, unlike [pathdiff](https://crates.io/crates/pathdiff).

<div>
  <a href="https://crates.io/crates/pathsub"><img alt='Version badge' src='https://img.shields.io/crates/v/pathsub.svg'></a>
  <a href="https://crates.io/crates/pathsub"><img alt='Downloads badge' src='https://img.shields.io/crates/d/pathsub.svg'></a>
  <a href="https://crates.io/crates/pathsub"><img alt='License badge' src='https://img.shields.io/crates/l/pathsub.svg'></a>
</div>

## Example:

```rust
use pathsub::sub_paths;
use std::path::Path;

let a = Path::new("foo/bar/baz");
let b = Path::new("foo");

sub_paths(a, b); // Some("bar/baz")
```
