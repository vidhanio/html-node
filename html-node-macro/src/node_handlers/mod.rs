#[cfg(feature = "typed")]
mod typed;

use std::collections::{HashMap, HashSet};

use proc_macro2::{Ident, Literal, TokenStream as TokenStream2};
use proc_macro2_diagnostics::{Diagnostic, SpanDiagnosticExt};
use quote::{quote, ToTokens};
use rstml::node::{
    KeyedAttribute, NodeAttribute, NodeBlock, NodeComment, NodeDoctype, NodeElement, NodeFragment,
    NodeName, NodeText, RawText,
};
use syn::{spanned::Spanned, Expr, ExprCast, Type};

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
    extensions: Option<&HashMap<Ident, Option<Type>>>,
    fragment: &NodeFragment,
) -> (TokenStream2, Vec<Diagnostic>) {
    let (inner_nodes, inner_diagnostics) =
        tokenize_nodes(void_elements, extensions, &fragment.children);

    let children = quote!(::std::vec![#(#inner_nodes),*]);

    (
        quote! {
            ::html_node::Node::Fragment(
                ::html_node::Fragment {
                    children: #children,
                }
            )
        },
        inner_diagnostics,
    )
}

pub fn handle_element(
    void_elements: &HashSet<&str>,
    extensions: Option<&HashMap<Ident, Option<Type>>>,
    element: &NodeElement,
) -> (TokenStream2, Vec<Diagnostic>) {
    extensions.map_or_else(
        || handle_element_untyped(void_elements, element),
        |extensions| typed::handle_element(void_elements, extensions, element),
    )
}

pub fn handle_element_untyped(
    void_elements: &HashSet<&str>,
    element: &NodeElement,
) -> (TokenStream2, Vec<Diagnostic>) {
    handle_element_inner(
        |block| {
            let attribute_tokens = quote! {
                (
                    ::std::convert::Into::<::std::string::String>::into(
                        #[allow(unused_braces)]
                        #block,
                    ),
                    ::std::option::Option::None,
                )
            };

            (attribute_tokens, None)
        },
        |attribute| {
            let key = node_name_to_literal(&attribute.key);

            let key = quote!(::std::convert::Into::<::std::string::String>::into(#key));

            let value = attribute.value().map(|value| match value {
                Expr::Cast(ExprCast { expr, ty, .. }) => (&**expr, Some(ty)),
                _ => (value, None),
            });

            let attribute_tokens = value.map_or_else(
                || quote!((#key, ::std::option::Option::None)),
                |(value, ty)| ty.map_or_else(
                    || quote!{
                        (
                            #key,
                            ::std::option::Option::Some(::std::string::ToString::to_string(&#value)),
                        )
                    },
                    |ty| quote!{
                        (
                            #key,
                            ::std::option::Option::<#ty>::from(#value).map(|v| ::std::string::ToString::to_string(&v)),
                        )
                    },
                ),
            );

            (attribute_tokens, None)
        },
        |element, attributes, children| {
            let name = node_name_to_literal(element.name());

            quote! {
                ::html_node::Node::Element(
                    ::html_node::Element {
                        name: ::std::convert::Into::<::std::string::String>::into(#name),
                        attributes: ::std::vec![#(#attributes),*],
                        children: #children,
                    }
                )
            }
        },
        void_elements,
        None,
        element,
    )
}

fn handle_element_inner<T>(
    handle_block: impl Fn(&NodeBlock) -> (T, Option<Diagnostic>),
    handle_keyed: impl Fn(&KeyedAttribute) -> (T, Option<Diagnostic>),
    to_element: impl Fn(&NodeElement, Vec<T>, TokenStream2) -> TokenStream2,
    void_elements: &HashSet<&str>,
    extensions: Option<&HashMap<Ident, Option<Type>>>,
    element: &NodeElement,
) -> (TokenStream2, Vec<Diagnostic>) {
    let (attributes, attribute_diagnostics) = element
        .attributes()
        .iter()
        .map(|attribute| match attribute {
            NodeAttribute::Block(block) => handle_block(block),
            NodeAttribute::Attribute(attribute) => handle_keyed(attribute),
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let is_void_element = void_elements.contains(element.open_tag.name.to_string().as_str());

    let (children, void_diagnostics) = if is_void_element {
        let diagnostic = if element.children.is_empty() {
            vec![]
        } else {
            vec![element
                .span()
                .warning("void elements' children will be ignored")]
        };

        (quote!(::std::option::Option::None), diagnostic)
    } else {
        let (inner_nodes, inner_diagnostics) =
            tokenize_nodes(void_elements, extensions, &element.children);

        (
            quote!(::std::option::Option::Some(::std::vec![#(#inner_nodes),*])),
            inner_diagnostics,
        )
    };

    let element = to_element(element, attributes, children);

    let diagnostics = attribute_diagnostics
        .into_iter()
        .flatten()
        .chain(void_diagnostics)
        .collect::<Vec<_>>();

    (element, diagnostics)
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

fn node_name_to_literal(node_name: &NodeName) -> TokenStream2 {
    match node_name {
        NodeName::Block(block) => quote!(#[allow(unused_braces)] #block),
        other_node_name => {
            let mut literal = Literal::string(&other_node_name.to_string());
            literal.set_span(other_node_name.span());
            literal.to_token_stream()
        }
    }
}

#[cfg(not(feature = "typed"))]
mod typed {
    use std::collections::{HashMap, HashSet};

    use rstml::node::NodeElement;
    use syn::{Ident, Type};

    pub fn handle_element(
        _void_elements: &HashSet<&str>,
        _extensions: &HashMap<Ident, Option<Type>>,
        _element: &NodeElement,
    ) -> ! {
        unreachable!("`typed::handle_element` should be unreachable without the `typed` feature")
    }
}
