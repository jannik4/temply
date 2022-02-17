#![deny(rust_2018_idioms)]

mod generator;
mod lexer;
mod parser;

use proc_macro2::TokenStream;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{Data, DeriveInput, Generics, Ident, Lit, Meta};

/// Derive the `Template` trait.
///
/// The template can be specified with either `#[template]` or `#[template_inline]`.
///
/// # Examples
///
/// ```ignore
/// # use temply::Template;
/// #[derive(Debug, Template)]
/// #[template = "./hello.template"] // Path is relative to the src folder
/// struct MyTemplate<'a> { name: &'a str }
/// ```
///
/// ```ignore
/// # use temply::Template;
/// #[derive(Debug, Template)]
/// #[template_inline = "Hello {{ name }}!"]
/// struct MyTemplate<'a> { name: &'a str }
/// ```
#[proc_macro_derive(Template, attributes(template, template_inline))]
pub fn derive_template(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse derive
    let (name, data, generics, source) = parse_derive(input.into());

    // Get source
    let (source, path) = match source {
        Source::File(path) => (
            fs::read_to_string(&path).expect("failed to read template from file"),
            Some(path),
        ),
        Source::Inline(source) => (source, None),
    };

    // Lex and parse
    let tokens = lexer::lex(&source);
    let ast = match parser::parse(&source, &tokens) {
        Ok(ast) => ast,
        Err(error) => panic!("failed to parse template: {}", error.format(&source)),
    };

    // Generate
    generator::generate(&name, &data, &generics, path.as_deref(), ast).into()
}

#[derive(Debug)]
enum Source {
    File(PathBuf),
    Inline(String),
}

fn parse_derive(input: TokenStream) -> (Ident, Data, Generics, Source) {
    let ast = syn::parse2::<DeriveInput>(input).unwrap();

    let root_path =
        Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string())).join("src/");
    let sources = ast
        .attrs
        .iter()
        .filter_map(|attr| match attr.parse_meta() {
            Ok(Meta::NameValue(name_value)) => {
                if name_value.path.is_ident("template") {
                    match name_value.lit {
                        Lit::Str(str) => Some(Source::File(root_path.join(str.value()))),
                        _ => panic!("template must be a string"),
                    }
                } else if name_value.path.is_ident("template_inline") {
                    match name_value.lit {
                        Lit::Str(str) => Some(Source::Inline(str.value())),
                        _ => panic!("template_inline must be a string"),
                    }
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    let source = if sources.len() == 1 {
        sources.into_iter().next().unwrap()
    } else {
        panic!("found zero or more than one template source");
    };

    (ast.ident, ast.data, ast.generics, source)
}
