mod util;

use temply::Template;

#[test]
fn test_simple() {
    #[derive(Debug, Template)]
    #[template_inline = "{% if self.0 %}Hello World!{% endif %}"]
    struct MyTemplate(bool);

    assert_render!(MyTemplate(true), "Hello World!");
    assert_render!(MyTemplate(false), "");
}

#[test]
fn test_else() {
    #[derive(Debug, Template)]
    #[template_inline = "Hello{% if self.0 %}{{ \" \" }}World!{% else %}!{% endif %}"]
    struct MyTemplate(bool);

    assert_render!(MyTemplate(true), "Hello World!");
    assert_render!(MyTemplate(false), "Hello!");
}

#[test]
fn test_ws() {
    #[derive(Debug, Template)]
    #[template_inline = "Hello{% if self.0 %} \n {{ \" \" }}World!{% else %} !  {% endif %}"]
    struct MyTemplate(bool);

    assert_render!(MyTemplate(true), "Hello World!");
    assert_render!(MyTemplate(false), "Hello!");
}

#[test]
fn test_if_else() {
    #[derive(Debug, Template)]
    #[template_inline = "{% if self.0 == 0 %}a{% else if self.0 == 1 %}b{% else if self.0 == 2 %}c{% else %}other{% endif %}"]
    struct MyTemplate(i32);

    assert_render!(MyTemplate(0), "a");
    assert_render!(MyTemplate(1), "b");
    assert_render!(MyTemplate(2), "c");
    assert_render!(MyTemplate(3), "other");
}

#[test]
fn test_nested() {
    #[derive(Debug, Template)]
    #[template_inline = r#"{% if self.0 == 0 %}
    {% if self.1 %}+a{% else %}-a{% endif %}
{% else if self.0 == 1 %}
    {% if self.1 %}+b{% else %}-b{% endif %}
{% else if self.0 == 2 %}
    {% if self.1 %}+c{% else %}-c{% endif %}
{% else %}
    {% if self.1 %}+other{% else %}-other{% endif %}
{% endif %}"#]
    struct MyTemplate(i32, bool);

    assert_render!(MyTemplate(0, true), "+a");
    assert_render!(MyTemplate(0, false), "-a");
    assert_render!(MyTemplate(1, true), "+b");
    assert_render!(MyTemplate(1, false), "-b");
    assert_render!(MyTemplate(2, true), "+c");
    assert_render!(MyTemplate(2, false), "-c");
    assert_render!(MyTemplate(3, true), "+other");
    assert_render!(MyTemplate(3, false), "-other");
}
