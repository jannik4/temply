mod util;

use temply::Template;

#[test]
fn test_simple() {
    #[derive(Debug, Template)]
    #[template_inline = "{% for i in 0..4 %}{{i * 2}},{% endfor %}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "0,2,4,6,");
}

#[test]
fn test_ws() {
    #[derive(Debug, Template)]
    #[template_inline = "    {% for i in 0..4 %}\n  {{i * 2}}{% endfor %}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "    0\n  2\n  4\n  6");
}

#[test]
fn test_nested() {
    #[derive(Debug, Template)]
    #[template_inline = "{% for i in 0..4 %}[{% for j in 0..4 %}{{i * j}},{% endfor %}],{% endfor %}"]
    struct MyTemplate;

    assert_render!(MyTemplate, "[0,0,0,0,],[0,1,2,3,],[0,2,4,6,],[0,3,6,9,],");
}
