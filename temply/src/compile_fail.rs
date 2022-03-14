//! # Use variable outside of scope
//!
//! ```compile_fail
//! use temply::Template;
//!
//! #[derive(Debug, Template)]
//! #[template_inline = r#"
//! {% scope %}
//!     {% let x = 12; %}
//!     {{ x }}
//! {% endscope %}
//! {{ x }}
//! "#]
//! struct MyTemplate;
//! ```
//!
//! # Missing match arm
//!
//! ```compile_fail
//! use temply::Template;
//!
//! #[derive(Debug, Template)]
//! #[template_inline = r#"
//! {% match Some(12) %}
//!     {% where Some(x) %}{{ x }}{% endwhere %}
//!     {# {% where None %}No value{% endwhere %} #}
//! {% endmatch %}
//! "#]
//! struct MyTemplate;
//! ```
//!
//! # Unclosed comment
//!
//! ```compile_fail
//! use temply::Template;
//!
//! #[derive(Debug, Template)]
//! #[template_inline = r#"
//! {# My comment
//! "#]
//! struct MyTemplate;
//! ```
//!
//! # Unclosed nested comment
//!
//! ```compile_fail
//! use temply::Template;
//!
//! #[derive(Debug, Template)]
//! #[template_inline = r#"
//! {# My nested {# comment #}
//! "#]
//! struct MyTemplate;
//! ```
//!
//! # Macro mut env
//!
//! ```compile_fail
//! use temply::Template;
//!
//! #[derive(Debug, Template)]
//! #[template_inline = r#"
//! {% let mut x = 12; %}
//! {% macro f || %}
//!     {% let _ = { x += 1; }; %}
//! {% endmacro %}
//! "#]
//! struct MyTemplate;
//! ```
