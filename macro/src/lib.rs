#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]

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

fn tokenize_nodes(
    void_elements: &HashSet<&str>,
    nodes: &[Node],
) -> (Vec<TokenStream2>, Vec<Diagnostic>) {
    let (token_streams, diagnostics) = nodes
        .iter()
        .map(|node| match node {
            Node::Comment(comment) => (handle_comment(comment), None),
            Node::Doctype(doctype) => (handle_doctype(doctype), None),
            Node::Fragment(fragment) => handle_fragment(void_elements, fragment),
            Node::Element(element) => handle_element(void_elements, element),
            Node::Block(block) => (handle_block(block), None),
            Node::Text(text) => (handle_text(text), None),
            Node::RawText(text) => (handle_raw_text(text), None),
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let diagnostics = diagnostics.into_iter().flatten().flatten().collect();

    (token_streams, diagnostics)
}

#[proc_macro]
pub fn html(tokens: TokenStream) -> TokenStream {
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
    let (tokenized_nodes, tokenization_diagnostics) = tokenize_nodes(&void_elements, &parsed_nodes);

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
