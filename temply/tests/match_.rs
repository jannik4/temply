mod util;

use temply::Template;

#[test]
fn test_simple() {
    #[derive(Debug, Template)]
    #[template_inline = "{% match self.0 %}{% where Some(x) %}{{ x }}{% endwhere %}{% where None %}No value{% endwhere %}{% endmatch %}"]
    struct MyTemplate(Option<i32>);

    assert_render!(MyTemplate(Some(42)), "42");
    assert_render!(MyTemplate(None), "No value");
}

#[test]
fn test_ws() {
    #[derive(Debug, Template)]
    #[template_inline = r#"{% match self.0 %}         
    {% where Some(x) %}          {{ x }}   
           
    
         {% endwhere %}
    {% where _ %}No value{% endwhere %}
                {% endmatch %}"#]
    struct MyTemplate(Option<i32>);

    assert_render!(MyTemplate(Some(42)), "42");
    assert_render!(MyTemplate(None), "No value");
}

#[test]
fn test_if_guard() {
    #[derive(Debug, Template)]
    #[template_inline = r#"{% match self.0 %}
    {% where Some(x) if x > 50 %}{{ x }}!!!{% endwhere %}
    {% where Some(x) %}{{ x }}{% endwhere %}
    {% where _ %}No value{% endwhere %}
{% endmatch %}"#]
    struct MyTemplate(Option<i32>);

    assert_render!(MyTemplate(Some(80)), "80!!!");
    assert_render!(MyTemplate(Some(20)), "20");
    assert_render!(MyTemplate(None), "No value");
}

#[test]
fn test_nested() {
    #[derive(Debug, Template)]
    #[template_inline = r#"{% match self.0 %}
    {% where Some(x) %}
        {% match x %}
            {% where x %}
                {{ x }}
            {% endwhere %}
        {% endmatch %}
    {% endwhere %}
    {% where _ %}
        No value
    {% endwhere %}
{% endmatch %}"#]
    struct MyTemplate(Option<i32>);

    assert_render!(MyTemplate(Some(42)), "42");
    assert_render!(MyTemplate(None), "No value");
}
