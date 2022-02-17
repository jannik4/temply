mod util;

use temply::Template;

#[test]
fn test_shadowing_local() {
    #[derive(Debug, Template)]
    #[template_inline = "{% let x = 12 %}{{ x }}, {% let x = 5 %}{{ x }}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "12, 5");
}

#[test]
fn test_shadowing_field() {
    #[derive(Debug, Template)]
    #[template_inline = "{{ x }}, {% let x = 5 %}{{ x }}"]
    struct MyTemplate {
        x: i32,
    }

    assert_render!(MyTemplate { x: 12 }, "12, 5");
}

#[test]
fn test_shadowing_in_scope() {
    #[derive(Debug, Template)]
    #[template_inline = "{{ x }}, {% scope %}{% let x = 5 %}{{ x }}{% endscope %}, {{ x }}"]
    struct MyTemplate {
        x: i32,
    }

    assert_render!(MyTemplate { x: 12 }, "12, 5, 12");
}
