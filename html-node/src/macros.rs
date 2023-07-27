/// Creates a [`Node::Comment`][crate::Node::Comment].
#[macro_export]
macro_rules! comment {
    ($($tt:tt)*) => {
        ::html_node::Node::Comment(::html_node::Comment {
            comment: ::std::format!($($tt)*),
        })
    };
}

/// Creates a [`Node::Text`][crate::Node::Text].
#[macro_export]
macro_rules! text {
    ($($tt:tt)*) => {
        ::html_node::Node::Text(::html_node::Text {
            text: ::std::format!($($tt)*),
        })
    };
}

/// Creates a [`Node::UnsafeText`][crate::Node::UnsafeText].
///
/// # Warning
///
/// [`Node::UnsafeText`][crate::Node::UnsafeText] is not escaped when rendered,
/// and as such, can allow for XSS attacks. Use with caution!
#[macro_export]
macro_rules! unsafe_text {
    ($($tt:tt)*) => {
        ::html_node::Node::UnsafeText(::html_node::UnsafeText {
            text: ::std::format!($($tt)*),
        })
    };
}
