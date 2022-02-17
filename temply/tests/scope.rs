mod util;

use temply::Template;

#[test]
fn test_simple() {
    #[derive(Debug, Template)]
    #[template_inline = "{% scope %}Hello World!{% endscope %}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "Hello World!");
}

#[test]
fn test_ws() {
    #[derive(Debug, Template)]
    #[template_inline = "{% scope %}\n \t Hello World! {{ 42 }} {% endscope %}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "Hello World! 42");
}

#[test]
fn test_nested() {
    #[derive(Debug, Template)]
    #[template_inline = "{% scope %}{% scope %}Hello{% endscope %} World!{% endscope %}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "Hello World!");
}
