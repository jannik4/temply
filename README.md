# temply

[![Build Status](https://github.com/jannik4/temply/workflows/CI/badge.svg)](https://github.com/jannik4/temply/actions)
[![crates.io](https://img.shields.io/crates/v/temply.svg)](https://crates.io/crates/temply)
[![docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/temply)
[![codecov](https://codecov.io/gh/jannik4/temply/branch/main/graph/badge.svg?token=WaLFTbiYBW)](https://codecov.io/gh/jannik4/temply)

temply is a simple, opinionated template engine. The syntax is derived from [Jinja](https://jinja.palletsprojects.com/). Templates can be defined inline or in an external file and are validated at compile time.

**Warning**: temply currently does not handle html-escaping and is therefore not suitable for html templates. You may be looking for [askama](https://crates.io/crates/askama) or [ructe](https://crates.io/crates/ructe).

## Example

```rust
use temply::Template;

#[derive(Debug, Template)]
#[template_inline = "Hello {{ name }}!"]
struct MyTemplate<'a> {
    name: &'a str
}

fn main() {
    // Init template
    let template = MyTemplate { name: "World" };

    // Render
    let mut buffer = String::new();
    template.render(&mut buffer).unwrap();

    assert_eq!(buffer, "Hello World!");
}
```
