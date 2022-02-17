mod util;

use temply::Template;

#[test]
fn test_misc() {
    #[derive(Debug, Template)]
    #[template = "../tests/templates/misc.template"]
    struct MyTemplate {
        name: &'static str,
    }

    impl MyTemplate {
        fn count(&self) -> usize {
            5
        }
    }

    assert_render!(
        MyTemplate { name: "test1234" },
        include_str!("./templates/misc.rendered")
    );
}
