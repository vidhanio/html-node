use std::fmt::{self, Display, Formatter};

use crate::Node;

/// A wrapper around [`Node`] that is always pretty printed.
#[derive(Debug, Default)]
pub struct Pretty(pub Node);

impl Display for Pretty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#}", self.0)
    }
}

impl<N> From<N> for Pretty
where
    N: Into<Node>,
{
    fn from(node: N) -> Self {
        Self(node.into())
    }
}
