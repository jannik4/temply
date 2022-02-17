mod util;

use temply::Template;

#[test]
fn test_simple() {
    #[derive(Debug, Template)]
    #[template_inline = "Hello World!"]
    struct MyTemplate;

    assert_render!(MyTemplate, "Hello World!");
}

#[test]
fn test_new_lines() {
    #[derive(Debug, Template)]
    #[template_inline = "\n\nHello World\n!\n\n"]
    struct MyTemplate;

    assert_render!(MyTemplate, "\n\nHello World\n!\n\n");
}

#[test]
fn test_rust_code() {
    #[derive(Debug, Template)]
    #[template_inline = "let fn = 0; match 5 { true => false, for => () }"]
    struct MyTemplate;

    assert_render!(
        MyTemplate,
        "let fn = 0; match 5 { true => false, for => () }"
    );
}

#[test]
fn test_format_params() {
    #[derive(Debug, Template)]
    #[template_inline = "{0} {} {:?} {:#?}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "{0} {} {:?} {:#?}");
}
