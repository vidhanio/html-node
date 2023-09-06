use std::collections::{HashMap, HashSet};

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use proc_macro2_diagnostics::{Diagnostic, SpanDiagnosticExt};
use quote::{quote, ToTokens};
use rstml::node::{KeyedAttribute, NodeElement, NodeName};
use syn::{punctuated::Pair, spanned::Spanned, ExprPath, Type};

use super::{handle_element_inner, node_name_to_literal};

enum AttrType {
    TypeChecked(TokenStream2),
    Extension(Option<Type>, TokenStream2),
}

pub fn handle_element(
    void_elements: &HashSet<&str>,
    extensions: &HashMap<Ident, Option<Type>>,
    element: &NodeElement,
) -> (TokenStream2, Vec<Diagnostic>) {
    handle_element_inner(
        |block| {
            let diagnostic = block
                .span()
                .error("typed elements don't support block attributes");

            (AttrType::TypeChecked(TokenStream2::new()), Some(diagnostic))
        },
        |attr| handle_attribute(attr, extensions),
        |element, attributes, children| {
            let name = element.name();

            let (type_checked, other, extensions) = attributes.into_iter().fold(
                (Vec::new(), Vec::new(), HashMap::new()),
                |(mut normals, mut others, mut extensions), attribute| {
                    match attribute {
                        AttrType::TypeChecked(attribute) => normals.push(attribute),
                        AttrType::Extension(type_, attribute) => match type_ {
                            Some(type_) => extensions
                                .entry(type_)
                                .or_insert_with(Vec::new)
                                .push(attribute),
                            None => others.push(attribute),
                        },
                    }

                    (normals, others, extensions)
                },
            );

            let extensions = extensions.into_iter().map(|(type_, attributes)| {
                quote! {
                    ::html_node::typed::TypedAttributes::into_attributes(
                        #[allow(clippy::needless_update, clippy::unnecessary_struct_initialization)]
                        #type_ {
                            #(#attributes,)*
                            ..::std::default::Default::default()
                        }
                    )
                }
            });

            let extensions = quote! {
                {
                    let mut v = ::std::vec::Vec::new();
                    #(
                        v.append(&mut #extensions);
                    )*
                    v.append(&mut ::std::vec![#(#other,)*]);

                    v
                }
            };

            quote! {
                {
                    type ElementAttributes = <#name as ::html_node::typed::TypedElement>::Attributes;
                    <#name as ::html_node::typed::TypedElement>::into_node(
                        <#name as ::html_node::typed::TypedElement>::from_attributes(
                            #[allow(clippy::needless_update, clippy::unnecessary_struct_initialization)]
                            ElementAttributes {
                                #(#type_checked,)*
                                ..::std::default::Default::default()
                            },
                            #extensions,
                        ),
                        #children
                    )
                }
            }
        },
        void_elements,
        Some(extensions),
        element,
    )
}

#[allow(clippy::too_many_lines)]
fn handle_attribute(
    attribute: &KeyedAttribute,
    extensions: &HashMap<Ident, Option<Type>>,
) -> (AttrType, Option<Diagnostic>) {
    let key = match &attribute.key {
        NodeName::Block(block) => Err(block
            .span()
            .error("block attribute keys are not supported for typed elements")),
        NodeName::Path(path) => handle_path_attribute(path),
        NodeName::Punctuated(puncutated) => {
            if let Some(Pair::Punctuated(n, p)) = puncutated.pairs().next() {
                let name = n.to_string();
                if p.as_char() == '-' {
                    extensions.get(&Ident::new(&name, Span::call_site())).map_or_else(|| {
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
                                Ok(AttrType::TypeChecked(raw_ident.to_token_stream()))
                            }
                            Err(diagnostic) => Err(diagnostic),
                        }
                    }, |type_| type_.as_ref().map_or_else(|| {
                                let literal = node_name_to_literal(&attribute.key);
                                let literal = quote!(::std::convert::Into::<::std::string::String>::into(#literal));
                                Ok(AttrType::Extension(None, literal))
                            }, |type_| {
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
                                        Ok(AttrType::Extension(
                                            Some(type_.clone()),
                                            raw_ident.to_token_stream(),
                                        ))
                                    }
                                    Err(diagnostic) => Err(diagnostic),
                                }
                            }))
                } else {
                    Err(puncutated
                        .span()
                        .error("empty punctuated keys are not supported"))
                }
            } else if let Some(Pair::End(ident)) = puncutated.pairs().next() {
                Ok(AttrType::TypeChecked(ident.to_token_stream()))
            } else {
                Err(puncutated
                    .span()
                    .error("empty punctuated keys are not supported"))
            }
        }
    };

    let key = match key {
        Ok(key) => key,
        Err(diagnostic) => return (AttrType::TypeChecked(TokenStream2::new()), Some(diagnostic)),
    };

    let attribute = match key {
        AttrType::TypeChecked(key) => {
            let value = attribute.value().map_or_else(
                || quote!(::std::option::Option::None),
                |value| {
                    quote! {
                        ::std::option::Option::Some(
                            ::std::convert::Into::into(#value),
                        )
                    }
                },
            );

            AttrType::TypeChecked(quote! {
                #key: ::std::option::Option::Some(#value)
            })
        }
        AttrType::Extension(type_, key) => type_.map_or_else(
            || {
                let value = attribute.value().map_or_else(
                    || quote!(::std::option::Option::None),
                    |value| {
                        quote! {
                            ::std::option::Option::Some(
                                ::std::string::ToString::to_string(&#value),
                            )
                        }
                    },
                );

                AttrType::Extension(
                    None,
                    quote! {
                        (#key, #value)
                    },
                )
            },
            |type_| {
                let value = attribute.value().map_or_else(
                    || quote!(::std::option::Option::None),
                    |value| {
                        quote! {
                            ::std::option::Option::Some(
                                ::std::convert::Into::into(#value),
                            )
                        }
                    },
                );

                AttrType::Extension(
                    Some(type_),
                    quote! {
                        #key: ::std::option::Option::Some(#value)
                    },
                )
            },
        ),
    };

    (attribute, None)
}

fn handle_path_attribute(path: &ExprPath) -> Result<AttrType, Diagnostic> {
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

        Ok(AttrType::TypeChecked(ident.to_token_stream()))
    }
}
