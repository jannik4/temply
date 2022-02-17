mod util;

use temply::Template;

#[test]
fn test_simple() {
    #[derive(Debug, Template)]
    #[template_inline = r#"{{ "Hello World!" }}"#]
    struct MyTemplate;

    assert_render!(MyTemplate, "Hello World!");
}

#[test]
fn test_ws() {
    #[derive(Debug, Template)]
    #[template_inline = r#"{{ "\n Hello\nWorld!\t\t" }}"#]
    struct MyTemplate;

    assert_render!(MyTemplate, "\n Hello\nWorld!\t\t");
}

#[test]
fn test_method() {
    #[derive(Debug, Template)]
    #[template_inline = "x: {{ self.x() }}"]
    struct MyTemplate;

    impl MyTemplate {
        fn x(&self) -> i32 {
            12
        }
    }

    assert_render!(MyTemplate, "x: 12");
}

#[test]
fn test_destruct() {
    #[derive(Debug, Template)]
    #[template_inline = "x: {{ x }}, y: {{ self.y }}"]
    struct MyTemplate {
        x: i32,
        y: i32,
    }

    assert_render!(MyTemplate { x: 12, y: 3 }, "x: 12, y: 3");
}

#[test]
fn test_complex_expr() {
    #[derive(Debug, Template)]
    #[template_inline = "x: {{ x.map(|x| x * 4).unwrap_or(0) * 2 }}"]
    struct MyTemplate {
        x: Option<i32>,
    }

    assert_render!(MyTemplate { x: Some(3) }, "x: 24");
}

#[test]
fn test_format() {
    #[derive(Debug, Template)]
    #[template_inline = "{{ x@{:?} }}\n{{ self@{:#?} }}"]
    struct MyTemplate {
        x: Option<i32>,
    }

    assert_render!(
        MyTemplate { x: Some(3) },
        "Some(3)\nMyTemplate {\n    x: Some(\n        3,\n    ),\n}"
    );
}
