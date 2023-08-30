use std::fmt::{self, Display, Formatter};

use super::write_children;
use crate::Node;

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

impl Fragment {
    /// A fragment with no children.
    pub const EMPTY: Self = Self {
        children: Vec::new(),
    };
}

impl Default for Fragment {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Display for Fragment {
    /// Format the fragment's childrent as HTML elements.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write_children(f, &self.children, true)
    }
}

impl<N> FromIterator<N> for Fragment
where
    N: Into<Node>,
{
    /// Create a new fragment from an iterator of anything that
    /// can be converted into a [`crate::Node`].
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = N>,
    {
        Self {
            children: iter.into_iter().map(Into::into).collect(),
        }
    }
}

impl<I, N> From<I> for Fragment
where
    I: IntoIterator<Item = N>,
    N: Into<Node>,
{
    /// Create a new fragment from any iterator of anything that
    /// can be converted into a [`crate::Node`].
    fn from(iter: I) -> Self {
        Self::from_iter(iter)
    }
}
