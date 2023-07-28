//! Typed HTML nodes.
//!
//! # Examples
//!
//! ```rust
//! use html_node::typed::{self, elements::*};
//!                           // ^^^^^^^^^^^
//!                           // required to bring type definitions
//!                           // of all basic html elements into
//!                           // the current scope.
//!                           // (can also use `elements::div`, etc.)
//!
//! // defines a custom element named `CustomElement`, with the specified attributes.
//! // underscores in attributes get converted to and from hyphens in the
//! // `typed::html!` macro and rendering.
//!
//! // note that global attributes like `id` will be pre-defined when
//! // using the `typed::element!` macro.
//!
//! #[derive(Clone, Debug)]
//! struct Location {
//!     x: i32,
//!     y: i32,
//! }
//!
//! impl std::fmt::Display for Location {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         write!(f, "{},{}", self.x, self.y)
//!     }
//! }
//!
//! typed::element! {
//!     CustomElement("custom-element") {
//!         custom_attr, // implictly typed as a `String`
//!         location: Location,
//!     }
//! }
//!
//! // creates a normal `Node`, but checks types at compile-time!
//! let html = typed::html! {
//!     <div id="container">
//!         <CustomElement id="el" custom-attr="test" location=Location { x: 1, y: 2 } />
//!     </div>
//! };
//!
//! assert_eq!(
//!     html.to_string(),
//!     r#"<div id="container"><custom-element id="el" custom-attr="test" location="1,2"></custom-element></div>"#,
//! );

#[allow(clippy::module_name_repetitions)]
pub use html_node_core::typed::{elements, TypedAttributes, TypedElement};
/// Make a typed set of HTML attributes.
///
/// Used internally by [`element!`].
pub use html_node_core::typed_attributes as attributes;
/// Make a typed element.
///
/// # Examples
///
/// ## Fully Generated (With Custom Name)
///
/// ```rust
/// use html_node::typed;
///
/// typed::element! {
///     CustomElement("custom-element") {
///         custom_attr,
///     }
/// }
///
/// // note that global attributes like `id` will be pre-defined when
/// // using the `typed::element!` macro.
/// assert_eq!(
///     typed::html!(<CustomElement id="el" custom-attr="test" />).to_string(),
///     r#"<custom-element id="el" custom-attr="test"></custom-element>"#,
/// );
/// ```
///
/// ## Fully Generated (With Default Name)
///
/// ```rust
/// use html_node::typed;
///
/// typed::element! {
///     CustomElement {
///         custom_attr,
///     }
/// }
///
/// assert_eq!(
///     typed::html!(<CustomElement id="el" custom-attr="test" />).to_string(),
///     r#"<CustomElement id="el" custom-attr="test"></CustomElement>"#,
/// );
/// ```
///
/// ## Generated With Custom Attributes Name
///
/// ```rust
/// use html_node::typed::{self, TypedAttributes};
///
/// typed::element! {
///     CustomElement [CustomElementAttributesDifferent] {
///         custom_attr,
///     }
/// }
///
/// assert_eq!(
///     typed::html!(<CustomElement id="el" custom-attr="test" />).to_string(),
///     r#"<CustomElement id="el" custom-attr="test"></CustomElement>"#,
/// );
/// ```
///
/// ## Generated With Custom Attributes
///
/// ```rust
/// use html_node::typed::{self, TypedAttributes};
///
/// #[derive(Debug, Clone, Default)]
/// struct CustomElementAttributes {
///     custom_attr: Option<Option<String>>,
/// }
///
/// impl TypedAttributes for CustomElementAttributes {
///     fn into_attributes(self) -> Vec<(String, Option<String>)> {
///         vec![self.custom_attr.map(|v| ("custom-attr".into(), v))]
///             .into_iter()
///             .flatten()
///             .collect()
///     }
/// }
///
/// typed::element! {
///     CustomElement [CustomElementAttributes]
/// }
///
/// // note that global attributes like `id` will not be allowed here
/// // because they are not defined in `CustomElementAttributes`.
/// assert_eq!(
///     typed::html!(<CustomElement custom-attr="test" />).to_string(),
///     r#"<CustomElement custom-attr="test"></CustomElement>"#,
/// );
/// ```
pub use html_node_core::typed_element as element;
/// Make many typed elements.
///
/// This uses the same syntax as [`element!`], but repeated and seperated
/// by semicolons (`;`).
pub use html_node_core::typed_elements as elements;
/// Make a typed HTML node.
///
/// # Examples
///
/// ## Passing Type-Checking
///
/// ```rust
/// use html_node::typed::{self, elements::*};
///
/// let html = typed::html! {
///     <div class="cool" id="hello-world" data-my-attr="hello" aria-label="world">
///         "Hello, world!"
///     </div>
/// };
///
/// let expected = "\
/// <div class=\"cool\" id=\"hello-world\" data-my-attr=\"hello\" aria-label=\"world\">\
///     Hello, world!\
/// </div>\
/// ";
///
/// assert_eq!(html.to_string(), expected);
/// ```
///
/// ## Failing Type-Checking
///
/// ```compile_fail
/// use html_node::typed::{self, elements::*};
///
/// let html = typed::html! {
///     // ERROR: struct `html_node::typed::elements::DivAttributes` has no field named `my_attr`
///     <div class="cool" id="hello-world" my-attr="hello">
///         {text!("Hello, world!")}
///     </div>
/// };
/// ```
pub use html_node_macro::typed_html as html;
