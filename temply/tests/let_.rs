mod util;

use temply::Template;

#[test]
fn test_simple() {
    #[derive(Debug, Template)]
    #[template_inline = "{% let x = 12 %}{{ x }}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "12");
}

#[test]
fn test_semicolon() {
    #[derive(Debug, Template)]
    #[template_inline = "{% let x = 12 %}{{ x }},{% let y = 3; %}{{ y }}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "12,3");
}

#[test]
fn test_multiple() {
    #[derive(Debug, Template)]
    #[template_inline = "{% let x = 1 %}{% let y = 2 %}{{ x + y }}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "3");
}

#[test]
fn test_mut() {
    #[derive(Debug, Template)]
    #[template_inline = "{% let mut x = 1 %}{{ {let copy = x; x += 1; copy} }}, {{ x }}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "1, 2");
}

#[test]
fn test_pattern() {
    #[derive(Debug, Template)]
    #[template_inline = "{% let (x, mut y) = (12, 13) %}{{ x + y}}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "25");
}

#[test]
fn test_shadowing() {
    #[derive(Debug, Template)]
    #[template_inline = "{% let x = 12 %}{{ x }}, {% let x = 5 %}{{ x }}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "12, 5");
}

#[test]
fn test_access_self() {
    #[derive(Debug, Template)]
    #[template_inline = "{% let x = self.double(4) %}{{ x }}"]
    struct MyTemplate;

    impl MyTemplate {
        fn double(&self, x: i32) -> i32 {
            x * 2
        }
    }

    assert_render!(MyTemplate, "8");
}
