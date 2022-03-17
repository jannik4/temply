mod util;

use temply::Template;

#[test]
fn test_simple() {
    #[derive(Debug, Template)]
    #[dedent]
    #[template_inline = "Hello World!"]
    struct MyTemplate;

    assert_render!(MyTemplate, "Hello World!");
}

#[test]
fn test_scope() {
    #[derive(Debug, Template)]
    #[dedent]
    #[template_inline = r#"{% scope %}
    Hello World!
    1234
{% endscope %}"#]
    struct MyTemplate;

    assert_render!(MyTemplate, "Hello World!\n1234");
}

#[test]
fn test_scope_1() {
    #[derive(Debug, Template)]
    #[dedent]
    #[template_inline = r#"{% scope %}Hello World
    1234
{% endscope %}"#]
    struct MyTemplate;

    assert_render!(MyTemplate, "Hello World\n1234");
}

#[test]
fn test_scope_start_1() {
    #[derive(Debug, Template)]
    #[dedent]
    #[template_inline = r#"{% scope %}{{ self.0 }}
    1234
{% endscope %}"#]
    struct MyTemplate(u32);

    assert_render!(MyTemplate(42), "42\n1234");
}

#[test]
fn test_scope_start_2() {
    #[derive(Debug, Template)]
    #[dedent]
    #[template_inline = r#"{% scope %}{% if self.0 == 42 %}Hello{% endif %}
    1234
{% endscope %}"#]
    struct MyTemplate(u32);

    assert_render!(MyTemplate(42), "Hello\n1234");
}

#[test]
fn test_scope_no_dedent_inner() {
    #[derive(Debug, Template)]
    #[dedent]
    #[template_inline = r#"{% scope %}
{{ self.0 }}
    1234
{% endscope %}"#]
    struct MyTemplate(u32);

    assert_render!(MyTemplate(42), "42\n    1234");
}

#[test]
fn test_scope_no_dedent_end() {
    #[derive(Debug, Template)]
    #[dedent]
    #[template_inline = r#"{% scope %}
    {{ self.0 }}
    1234
abc{% endscope %}"#]
    struct MyTemplate(u32);

    assert_render!(MyTemplate(42), "42\n    1234\nabc");
}

#[test]
fn test_scope_nested() {
    #[derive(Debug, Template)]
    #[dedent]
    #[template_inline = r#"{% scope %}
    Hello World!
    {% scope %}
        xxxx
    {% endscope %}
    1234
{% endscope %}"#]
    struct MyTemplate;

    assert_render!(MyTemplate, "Hello World!\nxxxx\n1234");
}

#[test]
fn test_scope_nested_indented() {
    #[derive(Debug, Template)]
    #[dedent]
    #[template_inline = r#"{% scope %}
    Hello World!
        {% scope %}
            xxxx
            yyyy
        {% endscope %}
    1234
{% endscope %}"#]
    struct MyTemplate;

    assert_render!(MyTemplate, "Hello World!\n    xxxx\n    yyyy\n1234");
}

#[test]
fn test_for() {
    #[derive(Debug, Template)]
    #[dedent]
    #[template_inline = r#"{% for i in 0..4 %}
    {{ i }}
{% endfor %}"#]
    struct MyTemplate;

    assert_render!(MyTemplate, "0\n1\n2\n3");
}

#[test]
fn test_for_and_if_else() {
    #[derive(Debug, Template)]
    #[dedent]
    #[template_inline = r#"{% for i in 0..4 %}
    {% if i % 2 == 0 %}
        {{ i }}
                            {% else %}
                      -
    {% endif %}
{% endfor %}"#]
    struct MyTemplate;

    assert_render!(MyTemplate, "0\n-\n2\n-");
}

#[test]
fn test_for_and_text_and_if() {
    #[derive(Debug, Template)]
    #[dedent]
    #[template_inline = r#"{% for i in 0..4 %}
    {{ i }}{% if i != 3 %},{% endif %}
{% endfor %}"#]
    struct MyTemplate;

    assert_render!(MyTemplate, "0,\n1,\n2,\n3");
}

#[test]
fn test_match() {
    #[derive(Debug, Template)]
    #[dedent]
    #[template_inline = r#"{% match self.0 %}
    {% where Some(x) %}
        X = {{x}};
        ...
    {% endwhere %}
    {% where None %}
        no value
    {% endwhere %}
{% endmatch %}"#]
    struct MyTemplate(Option<i32>);

    assert_render!(MyTemplate(Some(42)), "X = 42;\n...");
    assert_render!(MyTemplate(None), "no value");
}

#[test]
fn test_match_indented() {
    #[derive(Debug, Template)]
    #[dedent]
    #[template_inline = r#"{% match self.0 %}
    {% where Some(x) %}
        X = {{x}};
            ...
    {% endwhere %}
    {% where None %}
        no value
    {% endwhere %}
{% endmatch %}"#]
    struct MyTemplate(Option<i32>);

    assert_render!(MyTemplate(Some(42)), "X = 42;\n    ...");
    assert_render!(MyTemplate(None), "no value");
}

#[test]
fn test_macro() {
    #[derive(Debug, Template)]
    #[dedent]
    #[template_inline = r#"
        {% macro render_expr |expr: &Expr| %}
            {% match expr %}
                {% where Expr::Add(a, b) %}
                    add(
                        {% call render_expr (&*a) %},
                        {% call render_expr (&*b) %}
                    )
                {% endwhere %}
                {% where Expr::Sign(a) %}
                    -(
                        {% call render_expr (&*a) %}
                    )
                {% endwhere %}
                {% where Expr::Num(n) %}
                    {{ n }}
                {% endwhere %}
            {% endmatch %}
        {% endmacro %}
Res: {
    {% call render_expr(&self.0) %}
}
"#]
    struct MyTemplate(Expr);

    #[derive(Debug)]
    enum Expr {
        Add(Box<Expr>, Box<Expr>),
        Sign(Box<Expr>),
        Num(u32),
    }

    assert_render!(
        MyTemplate(Expr::Add(
            Box::new(Expr::Sign(Box::new(Expr::Num(3)))),
            Box::new(Expr::Num(4))
        )),
        r#"
        
Res: {
    add(
        -(
            3
        ),
        4
    )
}
"#
    );
}
