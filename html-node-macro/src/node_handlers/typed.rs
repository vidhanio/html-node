use std::collections::HashSet;

use proc_macro2::{Ident, TokenStream as TokenStream2};
use proc_macro2_diagnostics::{Diagnostic, SpanDiagnosticExt};
use quote::{quote, ToTokens};
use rstml::node::{KeyedAttribute, NodeElement, NodeName};
use syn::{punctuated::Pair, spanned::Spanned, ExprPath};

use super::{handle_element_inner, node_name_to_literal};

enum AttrType {
    Normal(TokenStream2),
    Data(TokenStream2),
    Aria(TokenStream2),
}

pub fn handle_element(
    void_elements: &HashSet<&str>,
    element: &NodeElement,
) -> (TokenStream2, Vec<Diagnostic>) {
    handle_element_inner(
        |block| {
            let diagnostic = block
                .span()
                .error("typed elements don't support block attributes");

            (AttrType::Normal(TokenStream2::new()), Some(diagnostic))
        },
        handle_typed_attribute,
        |element, attributes, children| {
            let name = element.name();

            let (normals, datas, arias) = attributes.into_iter().fold(
                (Vec::new(), Vec::new(), Vec::new()),
                |(mut normals, mut datas, mut arias), attribute| {
                    match attribute {
                        AttrType::Normal(attribute) => normals.push(attribute),
                        AttrType::Data(attribute) => datas.push(attribute),
                        AttrType::Aria(attribute) => arias.push(attribute),
                    }

                    (normals, datas, arias)
                },
            );

            quote! {
                {
                    type ElementAttributes = <#name as ::html_node::typed::TypedElement>::Attributes;
                    <#name as ::html_node::typed::TypedElement>::into_node(
                        <#name as ::html_node::typed::TypedElement>::from_attributes(
                            #[allow(clippy::needless_update)]
                            ElementAttributes {
                                #(#normals,)*
                                ..::std::default::Default::default()
                            },
                            ::std::vec![#(#datas,)*],
                            ::std::vec![#(#arias,)*],
                        ),
                        #children
                    )
                }
            }
        },
        void_elements,
        true,
        element,
    )
}

fn handle_typed_attribute(attribute: &KeyedAttribute) -> (AttrType, Option<Diagnostic>) {
    let key = match &attribute.key {
        NodeName::Block(block) => Err(block
            .span()
            .error("block attribute keys are not supported for typed elements")),
        NodeName::Path(path) => handle_typed_path_attribute(path),
        NodeName::Punctuated(puncutated) => {
            if let Some(Pair::Punctuated(n, p)) = puncutated.pairs().next() {
                if p.as_char() == '-' {
                    let node_name = n.to_string();
                    if node_name == "data" {
                        let literal = node_name_to_literal(&attribute.key);
                        let literal =
                            quote!(::std::convert::Into::<::std::string::String>::into(#literal));
                        Ok(AttrType::Data(literal))
                    } else if node_name == "aria" {
                        let literal = node_name_to_literal(&attribute.key);
                        let literal =
                            quote!(::std::convert::Into::<::std::string::String>::into(#literal));
                        Ok(AttrType::Aria(literal))
                    } else {
                        let underscored_name = puncutated
                                .pairs()
                                .map(|pair| match pair {
                                    Pair::Punctuated(ident, punct) => {
                                        if punct.as_char() == '-' {
                                            Ok(format!("{ident}_"))
                                        } else {
                                            Err(punct.span().error("only hyphens can be converted to underscores in attribute names"))
                                        }
                                    }
                                    Pair::End(ident) => Ok(ident.to_string()),
                                })
                                .collect::<Result<String, _>>();

                        match underscored_name {
                            Ok(underscored_name) => {
                                let raw_ident =
                                    Ident::new_raw(&underscored_name, puncutated.span());
                                Ok(AttrType::Normal(raw_ident.to_token_stream()))
                            }
                            Err(diagnostic) => Err(diagnostic),
                        }
                    }
                } else {
                    Err(puncutated
                        .span()
                        .error("empty punctuated keys are not supported"))
                }
            } else if let Some(Pair::End(ident)) = puncutated.pairs().next() {
                Ok(AttrType::Normal(ident.to_token_stream()))
            } else {
                Err(puncutated
                    .span()
                    .error("empty punctuated keys are not supported"))
            }
        }
    };

    let key = match key {
        Ok(key) => key,
        Err(diagnostic) => return (AttrType::Normal(TokenStream2::new()), Some(diagnostic)),
    };

    let value = attribute.value().map_or_else(
        || quote!(::std::option::Option::None),
        |value| {
            quote! {
                ::std::option::Option::Some(
                    ::std::convert::Into::<::std::string::String>::into(#value),
                )
            }
        },
    );

    let attribute = match key {
        AttrType::Normal(key) => AttrType::Normal(quote! {
            #key: ::std::option::Option::Some(#value)
        }),
        AttrType::Data(key) => AttrType::Data(quote! {
            (#key, #value)
        }),
        AttrType::Aria(key) => AttrType::Aria(quote! {
            (#key, #value)
        }),
    };

    (attribute, None)
}

fn handle_typed_path_attribute(path: &ExprPath) -> Result<AttrType, Diagnostic> {
    if !path.attrs.is_empty() {
        Err(path
            .span()
            .error("typed elements don't support attributes on path keys"))
    } else if path.qself.is_some() {
        Err(path
            .span()
            .error("typed elements don't support qualified self on path keys"))
    } else if path.path.leading_colon.is_some() {
        Err(path
            .span()
            .error("typed elements don't support leading colons on path keys"))
    } else if path.path.segments.len() != 1 {
        Err(path
            .span()
            .error("typed elements don't support multiple segments on path keys"))
    } else {
        let segment = &path.path.segments[0];

        let ident = &segment.ident;

        let ident = Ident::new_raw(&ident.to_string(), path.span());

        Ok(AttrType::Normal(ident.to_token_stream()))
    }
}
