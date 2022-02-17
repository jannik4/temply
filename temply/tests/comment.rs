mod util;

use temply::Template;

#[test]
fn test_simple() {
    #[derive(Debug, Template)]
    #[template_inline = "{# My comment #}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "");
}

#[test]
fn test_ws() {
    #[derive(Debug, Template)]
    #[template_inline = "\n{#      \n My comment #}   \n \n\n"]
    struct MyTemplate;

    assert_render!(MyTemplate, "\n   \n \n\n");
}

#[test]
fn test_nested() {
    #[derive(Debug, Template)]
    #[template_inline = "Hello {# My {# comment #} #}World!"]
    struct MyTemplate;

    assert_render!(MyTemplate, "Hello World!");
}

#[test]
fn test_multi_line() {
    #[derive(Debug, Template)]
    #[template_inline = r#"Hello {# My
        
        1234      
        comment #}World!"#]
    struct MyTemplate;

    assert_render!(MyTemplate, "Hello World!");
}

#[test]
fn test_items() {
    #[derive(Debug, Template)]
    #[template_inline = "Hello {# My comment {{ self.x() }} {% if x %} {{{{{}}}}} #}World!"]
    struct MyTemplate;

    assert_render!(MyTemplate, "Hello World!");
}
