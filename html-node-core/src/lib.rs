//! The core crate for [`html-node`](https://docs.rs/html-node).

#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

/// HTTP Server integrations.
mod http;

/// [`crate::Node`] variant definitions.
mod node;

/// Pretty printing utilities.
#[cfg(feature = "pretty")]
pub mod pretty;

/// Typed HTML Nodes.
#[cfg(feature = "typed")]
pub mod typed;

use std::fmt::{self, Display, Formatter};

pub use self::node::*;
#[cfg(feature = "typed")]
use self::typed::TypedElement;

/// An HTML node.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Node {
    /// A comment.
    ///
    /// ```html
    /// <!-- I'm a comment! -->
    /// ```
    Comment(Comment),

    /// A doctype.
    ///
    /// ```html
    /// <!DOCTYPE html>
    /// ```
    Doctype(Doctype),

    /// A fragment.
    ///
    /// ```html
    /// <>
    ///     I'm in a fragment!
    /// </>
    /// ```
    Fragment(Fragment),

    /// An element.
    ///
    /// ```html
    /// <div class="container">
    ///     I'm in an element!
    /// </div>
    /// ```
    Element(Element),

    /// A text node.
    ///
    /// ```html
    /// <div>
    ///     I'm a text node!
    /// </div>
    /// ```
    Text(Text),

    /// An unsafe text node.
    ///
    /// # Warning
    ///
    /// [`Node::UnsafeText`] is not escaped when rendered, and as such, can
    /// allow for XSS attacks. Use with caution!
    UnsafeText(UnsafeText),
}

impl Node {
    /// A [`Node::Fragment`] with no children.
    pub const EMPTY: Self = Self::Fragment(Fragment {
        children: Vec::new(),
    });

    /// Create a new [`Node`] from a [`TypedElement`].
    #[cfg(feature = "typed")]
    pub fn from_typed<E: TypedElement>(element: E, children: Option<Vec<Self>>) -> Self {
        element.into_node(children)
    }

    /// Wrap the node in a pretty-printing wrapper.
    #[cfg(feature = "pretty")]
    #[must_use]
    pub fn pretty(self) -> pretty::Pretty {
        self.into()
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Comment(comment) => comment.fmt(f),
            Self::Doctype(doctype) => doctype.fmt(f),
            Self::Fragment(fragment) => fragment.fmt(f),
            Self::Element(element) => element.fmt(f),
            Self::Text(text) => text.fmt(f),
            Self::UnsafeText(unsafe_text) => unsafe_text.fmt(f),
        }
    }
}

impl<I, N> From<I> for Node
where
    I: IntoIterator<Item = N>,
    N: Into<Self>,
{
    fn from(iter: I) -> Self {
        Self::Fragment(iter.into())
    }
}

impl From<Comment> for Node {
    fn from(comment: Comment) -> Self {
        Self::Comment(comment)
    }
}

impl From<Doctype> for Node {
    fn from(doctype: Doctype) -> Self {
        Self::Doctype(doctype)
    }
}

impl From<Fragment> for Node {
    fn from(fragment: Fragment) -> Self {
        Self::Fragment(fragment)
    }
}

impl From<Element> for Node {
    fn from(element: Element) -> Self {
        Self::Element(element)
    }
}

impl From<Text> for Node {
    fn from(text: Text) -> Self {
        Self::Text(text)
    }
}

impl From<UnsafeText> for Node {
    fn from(text: UnsafeText) -> Self {
        Self::UnsafeText(text)
    }
}
