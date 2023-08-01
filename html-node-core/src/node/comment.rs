use std::fmt::{self, Display, Formatter};

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
    /// Format as an HTML comment.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<!-- {} -->", self.comment)
    }
}

impl<C> From<C> for Comment
where
    C: Into<String>,
{
    /// Create a new comment from anything that can be converted into a string.
    fn from(comment: C) -> Self {
        Self {
            comment: comment.into(),
        }
    }
}
