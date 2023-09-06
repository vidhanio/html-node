use std::fmt::{self, Display, Formatter};

use super::write_children;
use crate::Node;

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

impl Display for Element {
    /// Format as an HTML element.
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

impl<N> From<N> for Element
where
    N: Into<String>,
{
    /// Create an HTML element directly from a string.
    ///
    /// This [`Element`] has no attributes and no children.
    fn from(name: N) -> Self {
        Self {
            name: name.into(),
            attributes: Vec::new(),
            children: None,
        }
    }
}
