//! The core crate for [`html-node`](https://docs.rs/html-node).

#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod http;

#[allow(missing_docs)]
#[cfg(feature = "typed")]
pub mod typed;

use std::fmt::{self, Display, Formatter};

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
}

impl<I, N> From<I> for Node
where
    I: IntoIterator<Item = N>,
    N: Into<Self>,
{
    fn from(iter: I) -> Self {
        Self::Fragment(Fragment {
            children: iter.into_iter().map(Into::into).collect(),
        })
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

impl Default for Node {
    fn default() -> Self {
        Self::EMPTY
    }
}

/// A comment.
///
/// ```html
/// <!-- I'm a comment! -->
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Comment {
    /// The text of the comment.
    ///
    /// ```html
    /// <!-- comment -->
    /// ```
    pub comment: String,
}

impl Display for Comment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<!-- {} -->", self.comment)
    }
}

impl<C> From<C> for Comment
where
    C: Into<String>,
{
    fn from(comment: C) -> Self {
        Self {
            comment: comment.into(),
        }
    }
}

/// A doctype.
///
/// ```html
/// <!DOCTYPE html>
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Doctype {
    /// The value of the doctype.
    ///
    /// ```html
    /// <!DOCTYPE synax>
    /// ```
    pub syntax: String,
}

impl Display for Doctype {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<!DOCTYPE {}>", self.syntax)
    }
}

impl<S> From<S> for Doctype
where
    S: Into<String>,
{
    fn from(syntax: S) -> Self {
        Self {
            syntax: syntax.into(),
        }
    }
}

/// A fragment.
///
/// ```html
/// <>
///     I'm in a fragment!
/// </>
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fragment {
    /// The children of the fragment.
    ///
    /// ```html
    /// <>
    ///     <!-- I'm a child! -->
    ///     <child>I'm another child!</child>
    /// </>
    pub children: Vec<Node>,
}

impl Display for Fragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write_children(f, &self.children, true)
    }
}

/// An element.
///
/// ```html
/// <div class="container">
///     I'm in an element!
/// </div>
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Element {
    /// The name of the element.
    ///
    /// ```html
    /// <name>
    /// ```
    pub name: String,

    /// The attributes of the element.
    ///
    /// ```html
    /// <div attribute="value">
    /// ```
    pub attributes: Vec<(String, Option<String>)>,

    /// The children of the element.
    ///
    /// ```html
    /// <div>
    ///     <!-- I'm a child! -->
    ///     <child>I'm another child!</child>
    /// </div>
    /// ```
    pub children: Option<Vec<Node>>,
}

impl Element {
    /// Create a new [`Element`] from a [`TypedElement`].
    #[cfg(feature = "typed")]
    pub fn from_typed<E: TypedElement>(element: E, children: Option<Vec<Node>>) -> Self {
        element.into_element(children)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<{}", self.name)?;

        for (key, value) in &self.attributes {
            write!(f, " {key}")?;

            if let Some(value) = value {
                let encoded_value = html_escape::encode_double_quoted_attribute(value);
                write!(f, r#"="{encoded_value}""#)?;
            }
        }
        write!(f, ">")?;

        if let Some(children) = &self.children {
            write_children(f, children, false)?;

            write!(f, "</{}>", self.name)?;
        };

        Ok(())
    }
}

/// A text node.
///
/// ```html
/// <div>
///     I'm a text node!
/// </div>
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Text {
    /// The text of the node.
    ///
    /// ```html
    /// <div>
    ///     text
    /// </div>
    pub text: String,
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let encoded_value = html_escape::encode_text_minimal(&self.text);
        write!(f, "{encoded_value}")
    }
}

impl<T> From<T> for Text
where
    T: Into<String>,
{
    fn from(text: T) -> Self {
        Self { text: text.into() }
    }
}

/// An unsafe text node.
///
/// # Warning
///
/// [`UnsafeText`] is not escaped when rendered, and as such, can allow
/// for XSS attacks. Use with caution!
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnsafeText {
    /// The text of the node.
    pub text: String,
}

impl Display for UnsafeText {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl<T> From<T> for UnsafeText
where
    T: Into<String>,
{
    fn from(text: T) -> Self {
        Self { text: text.into() }
    }
}

/// Writes the children of a node.
///
/// If the formatter is in alternate mode, then the children are put on their
/// own lines.
///
/// If alternate mode is enabled and `is_fragment` is false, then each line
/// is indented by 4 spaces.
fn write_children(f: &mut Formatter<'_>, children: &[Node], is_fragment: bool) -> fmt::Result {
    if f.alternate() {
        let mut children_iter = children.iter();

        if is_fragment {
            if let Some(first_child) = children_iter.next() {
                write!(f, "{first_child:#}")?;

                for child in children_iter {
                    write!(f, "\n{child:#}")?;
                }
            }
        } else {
            for child_str in children_iter.map(|child| format!("{child:#}")) {
                for line in child_str.lines() {
                    write!(f, "\n    {line}")?;
                }
            }

            // exit inner block
            writeln!(f)?;
        }
    } else {
        for child in children {
            child.fmt(f)?;
        }
    }
    Ok(())
}
