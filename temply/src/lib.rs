#![deny(rust_2018_idioms)]
//! temply is a simple, opinionated template engine. The syntax is derived from
//! [Jinja](https://jinja.palletsprojects.com/). Templates can be defined inline or in an external
//! file and are validated at compile time.
//!
//! The syntax is documented in the [`syntax module`](./syntax/index.html).
//!
//! **Warning**: temply currently does not handle html-escaping and is therefore not suitable for
//! html templates. You may be looking for [askama](https://crates.io/crates/askama) or
//! [ructe](https://crates.io/crates/ructe).
//!
//! # Example
//!
//! ```
//! use temply::Template;
//!
//! #[derive(Debug, Template)]
//! #[template_inline = "Hello {{ name }}!"]
//! struct MyTemplate<'a> {
//!     name: &'a str
//! }
//!
//! fn main() {
//!     // Init template
//!     let template = MyTemplate { name: "World" };
//!
//!     // Render
//!     let mut buffer = String::new();
//!     template.render(&mut buffer).unwrap();
//!
//!     assert_eq!(buffer, "Hello World!");
//! }
//! ```

pub mod syntax;

use std::fmt;

#[cfg(feature = "derive")]
pub use temply_derive::Template;

/// The template trait. Usually this is implemented by deriving
/// [`Template`](./derive.Template.html).
pub trait Template {
    fn render(&self, buffer: impl fmt::Write) -> fmt::Result;
}

// Compile fail tests
#[cfg(any(test, doctest))]
mod compile_fail;
