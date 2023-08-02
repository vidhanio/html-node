use std::fmt::{self, Display, Formatter};

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
    /// <!DOCTYPE syntax>
    /// ```
    pub syntax: String,
}

impl Display for Doctype {
    /// Format as an HTML doctype element.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<!DOCTYPE {}>", self.syntax)
    }
}

impl<S> From<S> for Doctype
where
    S: Into<String>,
{
    /// Create a new doctype element with a syntax attribute set
    /// from anything that can be converted into a string.
    fn from(syntax: S) -> Self {
        Self {
            syntax: syntax.into(),
        }
    }
}
