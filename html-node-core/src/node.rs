use std::fmt::{self, Display, Formatter};

mod comment;
mod doctype;
mod element;
mod fragment;
mod text;
mod unsafe_text;

pub use self::{
    comment::Comment, doctype::Doctype, element::Element, fragment::Fragment, text::Text,
    unsafe_text::UnsafeText,
};
use crate::Node;

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
