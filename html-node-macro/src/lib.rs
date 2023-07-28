#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]

mod node_handlers;

use std::collections::HashSet;

use node_handlers::{
    handle_block, handle_comment, handle_doctype, handle_element, handle_fragment, handle_raw_text,
    handle_text,
};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2_diagnostics::Diagnostic;
use quote::quote;
use rstml::{node::Node, Parser, ParserConfig};

#[proc_macro]
pub fn html(tokens: TokenStream) -> TokenStream {
    html_inner(tokens, false)
}

#[cfg(feature = "typed")]
#[proc_macro]
pub fn typed_html(tokens: TokenStream) -> TokenStream {
    html_inner(tokens, true)
}

fn html_inner(tokens: TokenStream, typed: bool) -> TokenStream {
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
        tokenize_nodes(&void_elements, typed, &parsed_nodes);

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
    typed: bool,
    nodes: &[Node],
) -> (Vec<TokenStream2>, Vec<Diagnostic>) {
    let (token_streams, diagnostics) = nodes
        .iter()
        .map(|node| match node {
            Node::Comment(comment) => (handle_comment(comment), vec![]),
            Node::Doctype(doctype) => (handle_doctype(doctype), vec![]),
            Node::Fragment(fragment) => handle_fragment(void_elements, typed, fragment),
            Node::Element(element) => handle_element(void_elements, typed, element),
            Node::Block(block) => (handle_block(block), vec![]),
            Node::Text(text) => (handle_text(text), vec![]),
            Node::RawText(text) => (handle_raw_text(text), vec![]),
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let diagnostics = diagnostics.into_iter().flatten().collect();

    (token_streams, diagnostics)
}
