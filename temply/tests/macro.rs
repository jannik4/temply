mod util;

use temply::Template;

#[test]
fn test_simple() {
    #[derive(Debug, Template)]
    #[template_inline = "{% macro x |y| %}{{ y }}{% endmacro %}{% call x(42) %}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "42");
}

#[test]
fn test_zero_params() {
    #[derive(Debug, Template)]
    #[template_inline = "{% macro x || %}{{ 12 }}{% endmacro %}{% call x() %}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "12");
}

#[test]
fn test_multiple_params() {
    #[derive(Debug, Template)]
    #[template_inline = "{% macro x |y, z| %}{{ y + z }}{% endmacro %}{% call x(42, 3) %}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "45");
}

#[test]
fn test_trailing_label() {
    #[derive(Debug, Template)]
    #[template_inline = "{% macro x |y, z, | %}{{ y + z }}{% endmacro %}{% call x(42, 3,) %}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "45");
}

#[test]
fn test_recursive() {
    #[derive(Debug, Template)]
    #[template_inline = r#"{% macro fact |x| %}
    {% if x == 0 %}
        1
    {% else %}
        {{ x }} * {% call fact(x - 1) %}
    {% endif %}
{% endmacro %}{% call fact(5) %}"#]
    struct MyTemplate;

    assert_render!(MyTemplate, "5 * 4 * 3 * 2 * 1 * 1");
}

#[test]
fn test_param_pattern() {
    #[derive(Debug, Template)]
    #[template_inline = "{% macro x |(y, z)| %}{{ y + z }}{% endmacro %}{% call x(self.0) %}"]
    struct MyTemplate((u32, u32));

    assert_render!(MyTemplate((2, 3)), "5");
}

#[test]
fn test_param_typed() {
    #[derive(Debug, Template)]
    #[template_inline = "{% macro x |y: i32, z| %}{{ y + z }}{% endmacro %}{% call x(2 + 3, 5 - 2) %}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "8");
}

#[test]
fn test_arg_expr() {
    #[derive(Debug, Template)]
    #[template_inline = "{% macro x |y, z| %}{{ y + z }}{% endmacro %}{% call x(2 + 3, 5 - 2) %}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "8");
}

#[test]
fn test_arg_ref() {
    #[derive(Debug, Template)]
    #[template_inline = "{% macro x |y: &i32| %}{{ *y }}{% endmacro %}{% call x(&2) %}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "2");
}

#[test]
fn test_arg_mut_ref() {
    #[derive(Debug, Template)]
    #[template_inline = "{% macro x |y: &mut i32| %}{% let _ = { *y += 1; }; %}{{ *y }}{% endmacro %}{% call x(&mut 2) %}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "3");
}
