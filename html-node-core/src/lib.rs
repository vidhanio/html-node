//! The core crate for [`html-node`](https://docs.rs/html-node).

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

/// HTTP Server integrations.
mod http;

/// [`Node`] variant definitions.
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
    pub const EMPTY: Self = Self::Fragment(Fragment::EMPTY);

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

    /// Borrow the children of the node, if it is an element (with children) or
    /// a fragment.
    #[must_use]
    pub fn as_children(&self) -> Option<&[Self]> {
        match self {
            Self::Fragment(fragment) => Some(&fragment.children),
            Self::Element(element) => element.children.as_deref(),
            _ => None,
        }
    }

    /// Iterate over the children of the node.
    pub fn children_iter(&self) -> impl Iterator<Item = &Self> {
        self.as_children().unwrap_or_default().iter()
    }

    /// The children of the node, if it is an element (with children) or
    /// a fragment.
    #[must_use]
    pub fn children(self) -> Option<Vec<Self>> {
        match self {
            Self::Fragment(fragment) => Some(fragment.children),
            Self::Element(element) => element.children,
            _ => None,
        }
    }

    /// Iterate over the children of the node, consuming it.
    pub fn into_children(self) -> impl Iterator<Item = Self> {
        self.children().unwrap_or_default().into_iter()
    }

    /// Try to get this node as a [`Comment`], if it is one.
    #[must_use]
    pub const fn as_comment(&self) -> Option<&Comment> {
        if let Self::Comment(comment) = self {
            Some(comment)
        } else {
            None
        }
    }

    /// Try to get this node as a [`Doctype`], if it is one.
    #[must_use]
    pub const fn as_doctype(&self) -> Option<&Doctype> {
        if let Self::Doctype(doctype) = self {
            Some(doctype)
        } else {
            None
        }
    }

    /// Try to get this node as a [`Fragment`], if it is one.
    #[must_use]
    pub const fn as_fragment(&self) -> Option<&Fragment> {
        if let Self::Fragment(fragment) = self {
            Some(fragment)
        } else {
            None
        }
    }

    /// Try to get this node as an [`Element`], if it is one.
    #[must_use]
    pub const fn as_element(&self) -> Option<&Element> {
        if let Self::Element(element) = self {
            Some(element)
        } else {
            None
        }
    }

    /// Try to get this node as a [`Text`], if it is one.
    #[must_use]
    pub const fn as_text(&self) -> Option<&Text> {
        if let Self::Text(text) = self {
            Some(text)
        } else {
            None
        }
    }

    /// Try to get this node as an [`UnsafeText`], if it is one.
    #[must_use]
    pub const fn as_unsafe_text(&self) -> Option<&UnsafeText> {
        if let Self::UnsafeText(text) = self {
            Some(text)
        } else {
            None
        }
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
