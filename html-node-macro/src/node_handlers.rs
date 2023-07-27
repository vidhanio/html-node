use std::collections::HashSet;

use proc_macro2::{Literal, TokenStream as TokenStream2};
use proc_macro2_diagnostics::{Diagnostic, SpanDiagnosticExt};
use quote::{quote, ToTokens};
use rstml::node::{
    NodeAttribute, NodeBlock, NodeComment, NodeDoctype, NodeElement, NodeFragment, NodeName,
    NodeText, RawText,
};
use syn::spanned::Spanned;

use crate::tokenize_nodes;

pub fn handle_comment(comment: &NodeComment) -> TokenStream2 {
    let comment = &comment.value;

    quote! {
        ::html_node::Node::Comment(
            ::html_node::Comment {
                comment: ::std::convert::Into::<::std::string::String>::into(#comment),
            }
        )
    }
}

pub fn handle_doctype(doctype: &NodeDoctype) -> TokenStream2 {
    let syntax = &doctype.value.to_token_stream_string();

    quote! {
        ::html_node::Node::Doctype(
            ::html_node::Doctype {
                syntax: ::std::convert::Into::<::std::string::String>::into(#syntax),
            }
        )
    }
}

pub fn handle_fragment(
    void_elements: &HashSet<&str>,
    fragment: &NodeFragment,
) -> (TokenStream2, Option<Vec<Diagnostic>>) {
    let (inner_nodes, inner_diagnostics) = tokenize_nodes(void_elements, &fragment.children);

    let children = quote!(::std::vec![#(#inner_nodes),*]);

    (
        quote! {
            ::html_node::Node::Fragment(
                ::html_node::Fragment {
                    children: #children,
                }
            )
        },
        Some(inner_diagnostics),
    )
}

pub fn handle_element(
    void_elements: &HashSet<&str>,
    element: &NodeElement,
) -> (TokenStream2, Option<Vec<Diagnostic>>) {
    let attributes = element
        .attributes()
        .iter()
        .map(|attribute| match attribute {
            NodeAttribute::Block(block) => quote! {
                (
                    ::std::convert::Into::<::std::string::String>::into(
                        #[allow(unused_braces)]
                        #block,
                    ),
                    ::std::option::Option::None,
                )
            },
            NodeAttribute::Attribute(attribute) => {
                let key = node_name_to_tokens(&attribute.key);

                let key = quote!(::std::convert::Into::<::std::string::String>::into(#key));

                attribute.value().map_or_else(
                    || quote!((#key, ::std::option::Option::None)),
                    |value| {
                        quote! {
                            (
                                #key,
                                ::std::option::Option::Some(
                                    ::std::convert::Into::<::std::string::String>::into(#value),
                                ),
                            )
                        }
                    },
                )
            }
        });

    let attributes = quote!(::std::vec![#(#attributes),*]);

    let is_void_element = void_elements.contains(element.open_tag.name.to_string().as_str());

    let (children, diagnostics) = if is_void_element {
        let diagnostic = (!element.children.is_empty()).then(|| {
            vec![element
                .span()
                .warning("void elements' children will be ignored")]
        });

        (quote!(::std::option::Option::None), diagnostic)
    } else {
        let (inner_nodes, inner_diagnostics) = tokenize_nodes(void_elements, &element.children);

        (
            quote!(::std::option::Option::Some(::std::vec![#(#inner_nodes),*])),
            Some(inner_diagnostics),
        )
    };

    let name = node_name_to_tokens(element.name());

    (
        quote! {
            ::html_node::Node::Element(
                ::html_node::Element {
                    name: ::std::convert::Into::<::std::string::String>::into(#name),
                    attributes: #attributes,
                    children: #children,
                }
            )
        },
        diagnostics,
    )
}

pub fn handle_block(block: &NodeBlock) -> TokenStream2 {
    quote! {
        ::std::convert::Into::<::html_node::Node>::into(#[allow(unused_braces)] #block)
    }
}

pub fn handle_text(text: &NodeText) -> TokenStream2 {
    let text = &text.value;

    quote! {
        ::html_node::Node::Text(
            ::html_node::Text {
                text: ::std::convert::Into::<::std::string::String>::into(#text),
            }
        )
    }
}

pub fn handle_raw_text(raw_text: &RawText) -> TokenStream2 {
    let tokens = raw_text.to_string_best();
    let mut text = Literal::string(&tokens);
    text.set_span(raw_text.span());

    quote! {
        ::html_node::Node::Text(
            ::html_node::Text {
                text: ::std::convert::Into::<::std::string::String>::into(#text),
            }
        )
    }
}

fn node_name_to_tokens(node_name: &NodeName) -> TokenStream2 {
    match node_name {
        NodeName::Block(block) => quote!(#[allow(unused_braces)] #block),
        string_node_name => {
            let mut literal = Literal::string(&string_node_name.to_string());
            literal.set_span(string_node_name.span());
            literal.to_token_stream()
        }
    }
}
