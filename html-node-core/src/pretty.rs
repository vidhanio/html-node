use std::fmt::{self, Display, Formatter};

use crate::Node;

/// A wrapper around [`Node`] that is always pretty printed.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pretty(pub Node);

impl Pretty {
    /// Extract the inner node.
    #[must_use]
    pub fn into_inner(self) -> Node {
        self.0
    }

    /// Borrow the inner node.
    #[must_use]
    pub const fn as_inner(&self) -> &Node {
        &self.0
    }
}

impl Display for Pretty {
    /// Format as a pretty printed HTML node.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#}", self.0)
    }
}

impl<N> From<N> for Pretty
where
    N: Into<Node>,
{
    /// Create a new pretty wrapper around the given node.
    fn from(node: N) -> Self {
        Self(node.into())
    }
}
