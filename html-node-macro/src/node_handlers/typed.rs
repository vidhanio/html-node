use std::collections::{HashMap, HashSet};

use proc_macro2::{Ident, Punct, Span, TokenStream as TokenStream2};
use proc_macro2_diagnostics::{Diagnostic, SpanDiagnosticExt};
use quote::{quote, ToTokens};
use rstml::node::{KeyedAttribute, NodeElement, NodeName, NodeNameFragment};
use syn::{
    punctuated::{Pair, Punctuated},
    spanned::Spanned,
    ExprPath, Type,
};

use super::{handle_element_inner, node_name_to_literal};

enum AttrType {
    Component,
    TypeChecked {
        key: TokenStream2,
        value: Option<TokenStream2>,
    },
    Extension {
        ty: Option<Type>,
        key: TokenStream2,
        value: TokenStream2,
    },
}

#[allow(clippy::too_many_lines)]
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

            (
                AttrType::TypeChecked {
                    key: TokenStream2::new(),
                    value: None,
                },
                Some(diagnostic),
            )
        },
        |attr| handle_attribute(attr, extensions),
        |element, attributes, children| {
            let name = element.name();

            let (
                component,
                (type_checked_keys, type_checked_values),
                (other_keys, other_values),
                extensions,
            ) = attributes.into_iter().fold(
                (
                    false,
                    (Vec::new(), Vec::new()),
                    (Vec::new(), Vec::new()),
                    HashMap::<_, (Vec<_>, Vec<_>)>::new(),
                ),
                |(mut component, mut type_checked, mut other, mut extension), attribute| {
                    match attribute {
                        AttrType::Component => component = true,
                        AttrType::TypeChecked { key, value } => {
                            type_checked.0.push(key);
                            type_checked.1.push(value);
                        }
                        AttrType::Extension {
                            ty: type_,
                            key,
                            value,
                        } => {
                            if let Some(type_) = type_ {
                                let extensions_vecs = extension.entry(type_).or_default();

                                extensions_vecs.0.push(key);
                                extensions_vecs.1.push(value);
                            } else {
                                other.0.push(key);
                                other.1.push(value);
                            }
                        }
                    }

                    (component, type_checked, other, extension)
                },
            );

            let extensions = extensions.into_iter().map(|(type_, (keys, values))| {
                quote! {
                    ::html_node::typed::TypedAttributes::into_attributes(
                        #[allow(clippy::needless_update, clippy::unnecessary_struct_initialization)]
                        #type_ {
                            #(#keys: #values,)*
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
                    v.append(&mut ::std::vec![#((#other_keys, #other_values),)*]);

                    v
                }
            };

            let default = if component {
                TokenStream2::new()
            } else {
                quote! {
                    ..::std::default::Default::default()
                }
            };

            let type_checked_values = if component {
                Box::new(type_checked_values.into_iter().map(|value| {
                    quote! {
                        #value
                    }
                })) as Box<dyn Iterator<Item = _>>
            } else {
                Box::new(type_checked_values.into_iter().map(|value| {
                    quote! {
                        ::html_node::typed::Attribute::Present(
                            #value
                        )
                    }
                })) as Box<dyn Iterator<Item = _>>
            };

            quote! {
                {
                    type ElementAttributes = <#name as ::html_node::typed::TypedElement>::Attributes;
                    <#name as ::html_node::typed::TypedElement>::into_node(
                        <#name as ::html_node::typed::TypedElement>::from_attributes(
                            #[allow(clippy::needless_update, clippy::unnecessary_struct_initialization)]
                            ElementAttributes {
                                #(#type_checked_keys: #type_checked_values,)*
                                #default
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

fn handle_attribute(
    attribute: &KeyedAttribute,
    extensions: &HashMap<Ident, Option<Type>>,
) -> (AttrType, Option<Diagnostic>) {
    let attr = match &attribute.key {
        NodeName::Block(block) => Err(block
            .span()
            .error("block attribute keys are not supported for typed elements")),
        NodeName::Path(path) => handle_path_attribute(path),
        NodeName::Punctuated(punctuated) => {
            handle_punctuated_attribute(&attribute.key, punctuated, extensions)
        }
    };

    let attr = match attr {
        Ok(attr) => attr,
        Err(diagnostic) => {
            return (
                AttrType::TypeChecked {
                    key: TokenStream2::new(),
                    value: None,
                },
                Some(diagnostic),
            )
        }
    };

    let attribute = match attr {
        AttrType::Component => AttrType::Component,
        AttrType::TypeChecked { key, .. } => {
            let value = attribute
                .value()
                .map(|value| quote!(::std::convert::Into::into(#value)));

            AttrType::TypeChecked { key, value }
        }
        AttrType::Extension { ty, key, .. } => {
            if let Some(ty) = ty {
                let value = attribute.value().map_or_else(
                    || quote!(::html_node::typed::Attribute::Empty),
                    |value| {
                        quote! {
                        ::html_node::typed::Attribute::Present(
                        ::std::convert::Into::into(#value)
                        )
                        }
                    },
                );

                AttrType::Extension {
                    ty: Some(ty),
                    key,
                    value,
                }
            } else {
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

                AttrType::Extension {
                    ty: None,
                    key,
                    value,
                }
            }
        }
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

        if ident == &Ident::new("component", Span::call_site()) {
            Ok(AttrType::Component)
        } else {
            let ident = Ident::new_raw(&ident.to_string(), path.span());

            Ok(AttrType::TypeChecked {
                key: ident.to_token_stream(),
                value: None,
            })
        }
    }
}

fn handle_punctuated_attribute(
    node_name: &NodeName,
    punctuated: &Punctuated<NodeNameFragment, Punct>,
    extensions: &HashMap<Ident, Option<Type>>,
) -> Result<AttrType, Diagnostic> {
    if let Some(Pair::Punctuated(n, p)) = punctuated.pairs().next() {
        let name = n.to_string();
        if p.as_char() == '-' {
            extensions
                .get(&Ident::new(&name, Span::call_site()))
                .map_or_else(
                    || {
                        hyphenated_to_underscored(punctuated).map(|name| AttrType::TypeChecked {
                            key: Ident::new_raw(&name, punctuated.span()).to_token_stream(),
                            value: None,
                        })
                    },
                    |type_| {
                        type_.as_ref().map_or_else(
                            || {
                                let literal = node_name_to_literal(node_name);
                                Ok(AttrType::Extension {
                                    ty: None,
                                    key: quote! {
                                        ::std::convert::Into::<::std::string::String>::into(#literal)
                                    },
                                    value: TokenStream2::new(),
                                })
                            },
                            |type_| {
                                hyphenated_to_underscored(punctuated).map(|name| {
                                    AttrType::Extension {
                                        ty: Some(type_.clone()),
                                        key: Ident::new_raw(&name, punctuated.span())
                                            .to_token_stream(),
                                        value: TokenStream2::new(),
                                    }
                                })
                            },
                        )
                    },
                )
        } else {
            Err(punctuated
                .span()
                .error("empty punctuated keys are not supported"))
        }
    } else if let Some(Pair::End(ident)) = punctuated.pairs().next() {
        Ok(AttrType::TypeChecked {
            key: ident.to_token_stream(),
            value: None,
        })
    } else {
        Err(punctuated
            .span()
            .error("empty punctuated keys are not supported"))
    }
}

fn hyphenated_to_underscored(
    punctuated: &Punctuated<NodeNameFragment, Punct>,
) -> Result<String, Diagnostic> {
    punctuated
        .pairs()
        .map(|pair| match pair {
            Pair::Punctuated(ident, punct) => {
                if punct.as_char() == '-' {
                    Ok(format!("{ident}_"))
                } else {
                    Err(punct
                        .span()
                        .error("only hyphens can be converted to underscores in attribute names"))
                }
            }
            Pair::End(ident) => Ok(ident.to_string()),
        })
        .collect()
}
