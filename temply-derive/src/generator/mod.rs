use std::path::Path;

use crate::parser::ast;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, Generics, Ident};

pub fn generate(
    name: &Ident,
    data: &Data,
    generics: &Generics,
    path: Option<&Path>,
    ast: ast::Ast<'_>,
) -> TokenStream {
    // Recompile if template changes
    let recompile_on_change = match path {
        Some(path) => {
            let path = path.to_str().unwrap();
            Some(quote! {
                const _: &str = include_str!(#path); // Recompile if template changes
            })
        }
        None => None,
    };

    // Generics
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Destruct self
    let destruct_self = generate_destruct_self(&name, data);

    // Ast
    let ast = generate_ast(ast);

    quote! {
        #recompile_on_change

        impl #impl_generics ::temply::Template for #name #ty_generics #where_clause {
            fn render(&self, mut __buffer: impl ::std::fmt::Write) -> ::std::fmt::Result {
                let __buffer = &mut __buffer;
                #destruct_self
                #ast
                Ok(())
            }
        }
    }
}

fn generate_destruct_self(name: &Ident, data: &Data) -> Option<TokenStream> {
    let data = match data {
        Data::Struct(data) => data,
        Data::Enum(_) | Data::Union(_) => return None,
    };
    let fields_named = match &data.fields {
        syn::Fields::Named(fields_named) => fields_named,
        syn::Fields::Unnamed(_) | syn::Fields::Unit => return None,
    };
    let names = fields_named
        .named
        .iter()
        .map(|field| field.ident.as_ref().unwrap());

    Some(quote! {
        #[allow(unused)]
        let #name{ #(#names,)* } = self;
    })
}

fn generate_ast(ast: ast::Ast<'_>) -> TokenStream {
    let items = ast.items.into_iter().map(generate_item);

    quote! {
        #(#items)*
    }
}

fn generate_item(item: ast::Item<'_>) -> TokenStream {
    match item {
        ast::Item::Text(text) => {
            let text = text_to_string(text);
            quote! {
                ::std::write!(__buffer, "{}", #text)?;
            }
        }
        ast::Item::Comment(_) => quote! {},
        ast::Item::Expr(expr, format) => {
            let expr = expr.parse::<TokenStream>().unwrap();
            quote! {
                ::std::write!(__buffer, #format, #expr)?;
            }
        }
        ast::Item::Let(let_) => {
            let let_ = let_.parse::<TokenStream>().unwrap();
            quote! { #let_; }
        }
        ast::Item::Scope(body) => {
            let body = generate_ast(body);
            quote! { { #body } }
        }
        ast::Item::For { for_, pre, body } => {
            let for_ = for_.parse::<TokenStream>().unwrap();
            let body = generate_ast(body);
            match pre {
                Some(pre_text) => {
                    let pre_text = text_to_string(pre_text);
                    quote! {
                        {
                            let mut __first = true;
                            #for_ {
                                if !__first {
                                    ::std::write!(__buffer, "{}", #pre_text)?;
                                }
                                __first = false;
                                #body
                            }
                        }
                    }
                }
                None => quote! { #for_ { #body } },
            }
        }
        ast::Item::If {
            if_,
            else_ifs,
            else_,
        } => {
            let if_body = generate_ast(if_.1);
            let if_ = if_.0.parse::<TokenStream>().unwrap();
            let else_ifs = else_ifs.into_iter().map(|(else_if, body)| {
                let else_if = else_if.parse::<TokenStream>().unwrap();
                let body = generate_ast(body);
                quote! { #else_if { #body } }
            });
            let else_ = match else_ {
                Some(body) => {
                    let body = generate_ast(body);
                    Some(quote! { else { #body } })
                }
                None => None,
            };
            quote! {
                #if_ {
                    #if_body
                }
                #(#else_ifs)*
                #else_
            }
        }
        ast::Item::Match { match_, wheres } => {
            let match_ = match_.parse::<TokenStream>().unwrap();
            let cases = wheres.into_iter().map(|(arm, body)| {
                let arm = arm.parse::<TokenStream>().unwrap();
                let body = generate_ast(body);
                quote! {
                    #arm => { #body }
                }
            });
            quote! {
                #match_ {
                    #(#cases)*
                }
            }
        }
        ast::Item::Macro { name, params, body } => {
            let struct_name = Ident::new(&format!("__closure_{}", name), Span::call_site());
            let struct_name_var = Ident::new(&format!("__closure_{}_var", name), Span::call_site());

            let params = params
                .iter()
                .map(|param| param.parse::<TokenStream>().unwrap())
                .collect::<Vec<_>>();
            let generics = (0..params.len())
                .map(|idx| Ident::new(&format!("T{}", idx), Span::call_site()))
                .collect::<Vec<_>>();

            let body = generate_ast(body);

            quote! {
                struct #struct_name<'c, #(#generics),*> {
                    f: &'c dyn Fn(&#struct_name<'c, #(#generics),*>, &mut dyn ::std::fmt::Write, #(#generics),*) -> ::std::fmt::Result
                }
                let #struct_name_var = #struct_name {
                    f: &|#struct_name_var, __buffer, #(#params),*| {
                        #body
                        Ok(())
                    }
                };
            }
        }
        ast::Item::Call { name, args, ind } => {
            let struct_name_var = Ident::new(&format!("__closure_{}_var", name), Span::call_site());
            let args = args
                .iter()
                .map(|arg| arg.parse::<TokenStream>().unwrap())
                .collect::<Vec<_>>();
            let write = if ind == 0 {
                quote! { __buffer }
            } else {
                let indentation = (0..ind).map(|_| ' ').collect::<String>();
                quote! { &mut ::temply::__intern::indent::Indenter::new(__buffer, #indentation) }
            };

            quote! {
                (#struct_name_var.f)(&#struct_name_var, #write, #(#args),*)?;
            }
        }
    }
}

fn text_to_string(text: ast::Text<'_>) -> String {
    let mut buffer = String::new();
    for line in text.lines {
        buffer += line.content;
        buffer += line.new_line;
    }
    buffer += text.trailing;
    buffer
}
