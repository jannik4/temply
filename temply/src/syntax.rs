//! Syntax for temply templates.
//!
//! # Use Template Fields and Methods
//!
//! Inside the template `self` can be accessed. Additionally, all fields can be accessed directly by
//! name if `Self` is a named struct.
//!
//! ```
//! use temply::Template;
//!
//! #[derive(Debug, Template)]
//! #[template_inline = "{{ x }} + 5 = {{ self.add(5) }}"]
//! struct MyTemplate { x: i32 }
//!
//! impl MyTemplate {
//!     fn add(&self, y: i32) -> i32 { self.x + y }
//! }
//! #
//! # fn main() {
//! #     let template = MyTemplate { x: 3 };
//! #     let mut buffer = String::new();
//! #     template.render(&mut buffer).unwrap();
//! #     assert_eq!(buffer, "3 + 5 = 8");
//! # }
//! ```
//!
//! # Whitespace Control
//!
//! By default, all whitespaces in the template code are preserved, except at the beginning and end
//! of blocks, where whitespace control is currently unspecified and subject to change.
//!
//! # Expression
//!
//! An expression is any valid Rust expression that implements [`Display`] and is delimited by `{{`
//! and `}}`. Alternatively to [`Display`] another format parameter can be used with `@<PARAM>`.
//!
//! ```
//! # use temply::Template;
//! #
//! # #[derive(Debug, Template)]
//! # #[template_inline = r#"
//! {{ x }}
//! {{ x + 2 }}
//! {{ true }}
//! {{ Some(12)@{:?} }}
//! # "#]
//! # struct MyTemplate {
//! #     x: i32
//! # }
//! ```
//!
//! # Let
//!
//! A let block is any valid Rust let statement delimited by `{%` and `%}`. Shadowing is allowed.
//! The semicolon at the end is optional.
//!
//! ```
//! # use temply::Template;
//! #
//! # #[derive(Debug, Template)]
//! # #[template_inline = r#"
//! {% let x = 12; %}
//! {% let mut y = 3 + 7 %}
//! {% let (x, mut z) = (10, 1) %}
//! # "#]
//! # struct MyTemplate;
//! ```
//!
//! # Scope
//!
//! A scope block is equivalent to a Rust block expression and can be used to limit the scope of a
//! let block. It starts with `{% scope %}` and ends with `{% endscope %}`.
//!
//! ```
//! # use temply::Template;
//! #
//! # #[derive(Debug, Template)]
//! # #[template_inline = r#"
//! {% scope %}
//!     {% let x = 12; %}
//!     {{ x }}
//! {% endscope %}
//! # "#]
//! # struct MyTemplate;
//! ```
//!
//! # For
//!
//! A for block is equivalent to a Rust for loop. It starts with `{% for <PAT> in <EXPR> %}` and
//! ends with `{% endfor %}`. Additionally, an optional `{% else %}` can be inserted. The else block
//! is executed when the for loop runs for zero iterations.
//!
//! ```
//! # use temply::Template;
//! #
//! # #[derive(Debug, Template)]
//! # #[template_inline = r#"
//! {% for i in (0..3).rev() %}
//!     {{ i }},
//! {% else %}
//!     Empty
//! {% endfor %}
//! # "#]
//! # struct MyTemplate;
//! ```
//!
//! # If
//!
//! The if block is equivalent to a Rust if statement. It starts with `{% if <EXPR> %}` and ends with
//! `{% endif %}`. Additionally, any number of `{% else if <EXPR> %}` and an optional `{% else %}`
//! can be inserted.
//!
//! ```
//! # use temply::Template;
//! #
//! # #[derive(Debug, Template)]
//! # #[template_inline = r#"
//! {% let x = 42 %}
//! {% if x == 1 %}
//!     x is equal to 1
//! {% else if x == 2 %}
//!     x is equal to 2
//! {% else %}
//!     x is equal to {{ x }}
//! {% endif %}
//! # "#]
//! # struct MyTemplate;
//! ```
//!
//! # Match
//!
//! A match block is equivalent to a Rust match statement. It starts with `{% match <EXPR> %}` and
//! ends with `{% endmatch %}`. Each match arm is specified by a where block:
//! `{% where <PAT> <GUARD>? %} ... {% endwhere %}`.
//!
//! ```
//! # use temply::Template;
//! #
//! # #[derive(Debug, Template)]
//! # #[template_inline = r#"
//! {% match Some(12) %}
//!     {% where Some(x) if x > 42 %}
//!         {{x}} is greater than 42
//!     {% endwhere %}
//!     {% where Some(x) %}
//!         {{x}} is not greater than 42
//!     {% endwhere %}
//!     {% where _ %}
//!         There is no value
//!     {% endwhere %}
//! {% endmatch %}
//! # "#]
//! # struct MyTemplate;
//! ```
//!
//! # Macro and Call
//!
//! A macro block is roughly equivalent to a Rust closure. It starts with
//! `{% macro <NAME> |<PARAMS>| %}` and ends with `{% endmacro %}`. The name has to be a valid Rust
//! identifier. Macros can only be called after they have been declared. Macros can call themselves
//! recursively.
//!
//! The call block can be used to call a macro. The syntax is `{% call <NAME> (<ARGS>) %}`.
//!
//! ```
//! # use temply::Template;
//! #
//! # #[derive(Debug, Template)]
//! # #[template_inline = r#"
//! {% macro fact |x| %}
//!     {% if x == 0 %}
//!         1
//!     {% else %}
//!         {{ x }} * {% call fact(x - 1) %}
//!     {% endif %}
//! {% endmacro %}
//! {% call fact(5) %}
//! # "#]
//! # struct MyTemplate;
//! ```
//!
//! # Comment
//!
//! A comment is any text delimited by `{#` and `#}`. Comments may be nested. Comments must always
//! be closed.
//!
//! ```
//! # use temply::Template;
//! #
//! # #[derive(Debug, Template)]
//! # #[template_inline = r#"
//! {# My comment #}
//! {# My {# nested #} comment #}
//! # "#]
//! # struct MyTemplate;
//! ```
//!
//! [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
