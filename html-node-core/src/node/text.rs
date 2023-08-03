use std::fmt::{self, Display, Formatter};

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
    /// Format as HTML encoded string.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let encoded_value = html_escape::encode_text_minimal(&self.text);
        write!(f, "{encoded_value}")
    }
}

impl<T> From<T> for Text
where
    T: Into<String>,
{
    /// Create a new text element from anything that can
    /// be converted into a string.
    fn from(text: T) -> Self {
        Self { text: text.into() }
    }
}
