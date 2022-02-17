#[macro_export]
macro_rules! assert_render {
    ($template:expr, $expected:expr) => {
        let mut buffer = String::new();
        $template.render(&mut buffer).unwrap();
        assert_eq!(buffer, $expected);
    };
}
