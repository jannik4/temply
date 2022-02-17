mod util;

use temply::Template;

#[test]
fn test_file() {
    #[derive(Debug, Template)]
    #[template = "../tests/templates/file.template"]
    struct MyTemplate {
        name: &'static str,
    }

    assert_render!(
        MyTemplate { name: "World" },
        include_str!("./templates/file.rendered")
    );
}
