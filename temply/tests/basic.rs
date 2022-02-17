mod util;

use temply::Template;

#[test]
fn test_simple() {
    #[derive(Debug, Template)]
    #[template_inline = "Hello World!"]
    struct MyTemplate;

    assert_render!(MyTemplate, "Hello World!");
}

#[test]
fn test_raw_string() {
    #[derive(Debug, Template)]
    #[template_inline = r###"Hello World!"###]
    struct MyTemplate;

    assert_render!(MyTemplate, "Hello World!");
}

#[test]
fn test_unit_struct() {
    #[derive(Debug, Template)]
    #[template_inline = "Unit struct"]
    struct MyTemplate;

    assert_render!(MyTemplate, "Unit struct");
}

#[test]
fn test_tuple_struct() {
    #[derive(Debug, Template)]
    #[template_inline = "Tuple struct {{ self.0 }}"]
    struct MyTemplate(i32);

    assert_render!(MyTemplate(-3), "Tuple struct -3");
}

#[test]
fn test_c_struct() {
    #[derive(Debug, Template)]
    #[template_inline = "C struct {{ value }} {{ self.x }}"]
    struct MyTemplate {
        value: i32,
        x: u8,
    }

    assert_render!(MyTemplate { value: -3, x: 1 }, "C struct -3 1");
}

#[test]
fn test_enum() {
    #[derive(Debug, Template)]
    #[template_inline = "Enum ..."]
    enum MyTemplate {
        A(i32),
        #[allow(unused)]
        B,
        #[allow(unused)]
        C {
            val: i32,
        },
    }

    assert_render!(MyTemplate::A(42), "Enum ...");
}

#[test]
fn test_method() {
    #[derive(Debug, Template)]
    #[template_inline = "x(true) = {{ self.x(true) }}"]
    struct MyTemplate;

    impl MyTemplate {
        fn x(&self, v: bool) -> bool {
            v && !v
        }
    }

    assert_render!(MyTemplate, "x(true) = false");
}

#[test]
fn test_function() {
    #[derive(Debug, Template)]
    #[template_inline = "4 + 3 = {{ add(4, 3) }}"]
    struct MyTemplate;

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    assert_render!(MyTemplate, "4 + 3 = 7");
}

#[test]
fn test_lifetime() {
    #[derive(Debug, Template)]
    #[template_inline = "{{ self.0 }}"]
    struct MyTemplate<'a>(&'a str);

    assert_render!(MyTemplate("test"), "test");
}

#[test]
fn test_generics() {
    #[derive(Debug, Template)]
    #[template_inline = "{{ self.0 }}"]
    struct MyTemplate<T>(&'static str, T);

    assert_render!(MyTemplate("test", true), "test");
}

#[test]
fn test_lifetime_and_generics() {
    #[derive(Debug, Template)]
    #[template_inline = "{{ self.0 }}"]
    struct MyTemplate<'a, T, U>(&'a str, T, U);

    assert_render!(MyTemplate("test", true, 0), "test");
}
