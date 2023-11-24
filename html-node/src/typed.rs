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
//! // defines a custom element named `CustomElement`, with the specified attributes.
//! // underscores in attributes get converted to and from hyphens in the
//! // `typed::html!` macro and rendering.
//!
//! // note that global attributes like `id` will be pre-defined when
//! // using the `typed::element!` macro.
//!
//! typed::element! {
//!     CustomElement("custom-element") {
//!         custom_attr, // implictly typed as a `String`
//!         location: Location,
//!     }
//! }
//!
//! typed::attributes! {
//!     [TestAttrs] {
//!         test_val: i32,
//!     }
//! }
//!
//! // creates a normal `Node`, but checks types at compile-time!
//! let html = typed::html! { (test: TestAttrs, any)
//!                        // ^^^^^^^^^^^^^^^^^^^^^^ these are extension attributes.
//!                        // they are not required, but allow you to specify extra attributes
//!                        // which will be available within this macro invocation.
//!                        // those of the form `attr-prefix: Type` will be type checked, and
//!                        // those with just `attr-prefix` will be considered "catch-all" prefixes
//!                        // allowing any attribute with that prefix to be specified.
//!                        // `data-*` and `aria-*` are predefined as catch-all prefixes.
//!     <div id="container">
//!         <CustomElement test-val=42 any-whatever data-cool=true id="el" custom-attr="test" location=Location { x: 1, y: 2 } />
//!     </div>
//! };
//!
//! assert_eq!(
//!     html.to_string(),
//!     "\
//!         <div id=\"container\">\
//!             <custom-element id=\"el\" custom-attr=\"test\" location=\"1,2\" test-val=\"42\" any-whatever data-cool=\"true\">\
//!             </custom-element>\
//!         </div>\
//!     ",
//! );

#[allow(clippy::module_name_repetitions)]
pub use html_node_core::typed::{elements, Attribute, TypedAttributes, TypedElement};
/// Make a typed set of HTML attributes.
///
/// Used internally by [`element!`].
pub use html_node_core::typed_attributes as attributes;
/// Make a typed HTML node.
///
/// # Examples
///
/// ## Passing Type-Checking
///
/// ```rust
/// use html_node::typed::{self, elements::*};
///
/// typed::component! {
///     CustomBody {
///         r: u8,
///         g: u8,
///         b: u8,
///         width: i32,
///     };
///
///     |CustomBodyAttributes { r, g, b, width }, _, children| typed::html! {
///         <div style={format!("background-color: rgb({r}, {g}, {b}); width: {width}px;")}>
///             { children }
///         </div>
///     }
/// }
///
/// let html = typed::html! {
///     <CustomBody component r=255 g=0 b=0 width=100>"Hello, world!"</CustomBody>
/// };
///
/// let expected = "\
/// <div style=\"background-color: rgb(255, 0, 0); width: 100px;\">\
///     Hello, world!\
/// </div>\
/// ";
///
/// assert_eq!(html.to_string(), expected);
/// ```
pub use html_node_core::typed_component as component;
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
/// use html_node::typed::{self, Attribute, TypedAttributes};
///
/// #[derive(Debug, Clone, Default)]
/// struct CustomElementAttributes {
///     custom_attr: Attribute<String>,
/// }
///
/// impl TypedAttributes for CustomElementAttributes {
///     fn into_attributes(self) -> Vec<(String, Option<String>)> {
///         vec![self.custom_attr.into_option().map(|v| ("custom-attr".into(), v))]
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
