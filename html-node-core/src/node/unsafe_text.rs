use std::fmt::{self, Display, Formatter};

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
    /// Unescaped text.
    ///
    /// This string is **not** HTML encoded!
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl<T> From<T> for UnsafeText
where
    T: Into<String>,
{
    /// Create a new unsafe text element from anything
    /// that can be converted into a string.
    fn from(text: T) -> Self {
        Self { text: text.into() }
    }
}
