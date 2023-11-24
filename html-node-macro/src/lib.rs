#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod node_handlers;

use std::collections::{HashMap, HashSet};

use node_handlers::{
    handle_block, handle_comment, handle_doctype, handle_element, handle_fragment, handle_raw_text,
    handle_text,
};
use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use proc_macro2_diagnostics::Diagnostic;
use quote::quote;
use rstml::{node::Node, Parser, ParserConfig};
use syn::Type;

#[proc_macro]
pub fn html(tokens: TokenStream) -> TokenStream {
    html_inner(tokens.into(), None)
}

#[cfg(feature = "typed")]
#[proc_macro]
pub fn typed_html(tokens: TokenStream) -> TokenStream {
    use syn::{punctuated::Punctuated, token::Paren, Token};

    #[derive(syn_derive::Parse)]
    struct ColonAndType {
        _colon_token: syn::Token![:],
        ty: Type,
    }

    #[derive(syn_derive::Parse)]
    enum MaybeColonAndType {
        #[parse(peek = Token![:])]
        ColonAndType(ColonAndType),
        Nothing,
    }

    #[derive(syn_derive::Parse)]
    struct Extension {
        prefix: Ident,
        colon_and_type: MaybeColonAndType,
    }

    #[derive(syn_derive::Parse)]
    struct Extensions {
        #[syn(parenthesized)]
        #[allow(dead_code)]
        paren_token: Paren,

        #[syn(in = paren_token)]
        #[parse(Punctuated::parse_terminated)]
        extensions: Punctuated<Extension, syn::Token![,]>,
    }

    #[derive(syn_derive::Parse)]
    enum MaybeExtensions {
        #[parse(peek = Paren)]
        Extensions(Extensions),
        Nothing,
    }

    #[derive(syn_derive::Parse)]
    struct TypedHtmlOptions {
        extensions: MaybeExtensions,
        tokens: TokenStream2,
    }

    let options = syn::parse_macro_input!(tokens as TypedHtmlOptions);

    let mut extensions = match options.extensions {
        MaybeExtensions::Extensions(extensions) => extensions
            .extensions
            .into_iter()
            .map(|extension| match extension.colon_and_type {
                MaybeColonAndType::ColonAndType(ColonAndType { ty, .. }) => {
                    (extension.prefix, Some(ty))
                }
                MaybeColonAndType::Nothing => (extension.prefix, None),
            })
            .collect::<HashMap<_, _>>(),
        MaybeExtensions::Nothing => HashMap::new(),
    };

    extensions.insert(Ident::new("data", proc_macro2::Span::call_site()), None);
    extensions.insert(Ident::new("aria", proc_macro2::Span::call_site()), None);

    html_inner(options.tokens, Some(&extensions))
}

fn html_inner(
    tokens: TokenStream2,
    extensions: Option<&HashMap<Ident, Option<Type>>>,
) -> TokenStream {
    // from: https://html.spec.whatwg.org/dev/syntax.html#void-elements
    let void_elements = [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source",
        "track", "wbr",
    ]
    .into_iter()
    .collect::<HashSet<_>>();

    // from: https://html.spec.whatwg.org/dev/syntax.html#raw-text-elements
    let raw_text_elements = ["script", "style"].into_iter().collect();

    let config = ParserConfig::new()
        .recover_block(true)
        .always_self_closed_elements(void_elements.clone())
        .raw_text_elements(raw_text_elements);

    let parser = Parser::new(config);
    let (parsed_nodes, parsing_diagnostics) = parser.parse_recoverable(tokens).split_vec();
    let (tokenized_nodes, tokenization_diagnostics) =
        tokenize_nodes(&void_elements, extensions, &parsed_nodes);

    let node = match &*tokenized_nodes {
        [node] => quote!(#node),
        nodes => {
            quote! {
                ::html_node::Node::Fragment(
                    ::html_node::Fragment {
                        children: ::std::vec![#(#nodes),*],
                    }
                )
            }
        }
    };

    let errors = parsing_diagnostics
        .into_iter()
        .chain(tokenization_diagnostics)
        .map(Diagnostic::emit_as_expr_tokens);

    quote! {
        {
            #(#errors;)*
            #node
        }
    }
    .into()
}

fn tokenize_nodes(
    void_elements: &HashSet<&str>,
    extensions: Option<&HashMap<Ident, Option<Type>>>,
    nodes: &[Node],
) -> (Vec<TokenStream2>, Vec<Diagnostic>) {
    let (token_streams, diagnostics) = nodes
        .iter()
        .map(|node| match node {
            Node::Comment(comment) => (handle_comment(comment), vec![]),
            Node::Doctype(doctype) => (handle_doctype(doctype), vec![]),
            Node::Fragment(fragment) => handle_fragment(void_elements, extensions, fragment),
            Node::Element(element) => handle_element(void_elements, extensions, element),
            Node::Block(block) => (handle_block(block), vec![]),
            Node::Text(text) => (handle_text(text), vec![]),
            Node::RawText(text) => (handle_raw_text(text), vec![]),
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let diagnostics = diagnostics.into_iter().flatten().collect();

    (token_streams, diagnostics)
}
