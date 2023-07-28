#![allow(clippy::module_name_repetitions)]
#![allow(non_snake_case)]

pub mod elements;
#[doc(hidden)]
pub use paste::paste;

use crate::{Element, Node};

/// A typed HTML element.
pub trait TypedElement: Default {
    /// The name of the element.
    const NAME: &'static str;

    /// The attributes of the element.
    type Attributes: TypedAttributes;

    /// Create an element from its attributes.
    fn from_attributes(
        attributes: Self::Attributes,
        data_attributes: Vec<(String, Option<String>)>,
        aria_attributes: Vec<(String, Option<String>)>,
    ) -> Self;

    /// Convert the typed element into an [`Element`].
    fn into_element(self, children: Option<Vec<Node>>) -> Element;

    /// Convert the typed element into a [`Node`].
    ///
    /// By default, this is equivalent to calling [`TypedElement::into_element`]
    /// and then just wrapping it in a [`Node::Element`].
    fn into_node(self, children: Option<Vec<Node>>) -> Node {
        Node::Element(self.into_element(children))
    }
}

/// A typed set of HTML attributes.
pub trait TypedAttributes: Default {
    /// Convert the typed attributes into a set of attributes.
    fn into_attributes(self) -> Vec<(String, Option<String>)>;
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! typed_elements {
    ($vis:vis $($ElementName:ident $(($name:literal))? $([$AttributeName:ident])? $({ $($attribute:ident),* $(,)? })?;)*) => {
        $(
            $crate::typed_element!{
                $vis $ElementName $(($name))? $([$AttributeName])? $({ $($attribute),* })?
            }
        )*
    };
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! typed_element {
    ($vis:vis $ElementName:ident $(($name:literal))? $([$AttributeName:ident])? $({ $($attribute:ident),* $(,)? })?) => {
        $crate::typed_attributes!{
            $vis $ElementName $([$vis $AttributeName])? $({
                accesskey,
                autocapitalize,
                autofocus,
                class,
                contenteditable,
                dir,
                draggable,
                enterkeyhint,
                exportparts,
                hidden,
                id,
                inert,
                inputmode,
                is,
                itemid,
                itemprop,
                itemref,
                itemscope,
                itemtype,
                lang,
                nonce,
                part,
                popover,
                role,
                slot,
                spellcheck,
                style,
                tabindex,
                title,
                translate,
                virtualkeyboardpolicy,
                $($attribute),*
            })?
        }

        #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::default::Default)]
        #[allow(non_camel_case_types)]
        #[allow(missing_docs)]
        $vis struct $ElementName {
            $vis attributes: <Self as $crate::typed::TypedElement>::Attributes,
            $vis data_attributes: ::std::vec::Vec<(::std::string::String, ::std::option::Option<::std::string::String>)>,
            $vis aria_attributes: ::std::vec::Vec<(::std::string::String, ::std::option::Option<::std::string::String>)>,
        }

        impl $crate::typed::TypedElement for $ElementName {
            const NAME: &'static str = $crate::typed_element!(@NAME_STR $ElementName$(($name))?);
            type Attributes = $crate::typed_attributes!(@NAME $ElementName $([$AttributeName])?);

            fn from_attributes(
                attributes: Self::Attributes,
                data_attributes: ::std::vec::Vec<(::std::string::String, ::std::option::Option<::std::string::String>)>,
                aria_attributes: ::std::vec::Vec<(::std::string::String, ::std::option::Option<::std::string::String>)>,
            ) -> Self {
                Self { attributes, data_attributes, aria_attributes }
            }

            fn into_element(mut self, children: ::std::option::Option<::std::vec::Vec<$crate::Node>>) -> $crate::Element {
                let mut attributes = $crate::typed::TypedAttributes::into_attributes(self.attributes);
                attributes.append(&mut self.data_attributes);
                attributes.append(&mut self.aria_attributes);

                $crate::Element {
                    name: Self::NAME.into(),
                    attributes,
                    children,
                }
            }
        }
    };
    (@NAME_STR $ElementName:ident) => {
        stringify!($ElementName)
    };
    (@NAME_STR $ElementName:ident($name:literal)) => {
        $name
    };
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! typed_attributes {
    {
        $($vise:vis $ElementName:ident)? $([$visa:vis $AttributeName:ident])? {
            $($attribute:ident),* $(,)?
        }
    } => {
        $crate::typed_attributes!(@STRUCT $($vise $ElementName)? $([$visa $AttributeName])? { $($attribute),* });

        impl $crate::typed::TypedAttributes for $crate::typed_attributes!(@NAME $($ElementName)? $([$AttributeName])?) {
            fn into_attributes(self) -> ::std::vec::Vec<(::std::string::String, ::std::option::Option<::std::string::String>)> {
                [$((::std::stringify!($attribute), self.$attribute)),*]
                    .into_iter()
                    .flat_map(|(key, maybe_value)| {
                        maybe_value.map(|value| (key.strip_prefix("r#").unwrap_or(key).replace('_', "-"), value))
                    })
                    .collect()
            }
        }
    };
    ($($_vise:vis $_ElementName:ident)? $([$_visa:vis $_AttributeName:ident])?) => {};
    (@NAME $ElementName:ident) => {
        $crate::typed::paste!([< $ElementName:camel Attributes >])
    };
    (@NAME $ElementName:ident [$AttributeName:ident]) => {
        $AttributeName
    };
    {
        @STRUCT $vis:vis $ElementName:ident {
            $($attribute:ident),* $(,)?
        }
    } => {
        $crate::typed::paste! {
            #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::default::Default)]
            #[allow(missing_docs)]
            $vis struct [< $ElementName:camel Attributes >] {
                $($vis $attribute: ::std::option::Option<::std::option::Option<::std::string::String>>,)*
            }
        }
    };
    {
        @STRUCT $_vis:vis $ElementName:ident [$vis:vis $AttributeName:ident] {
            $($attribute:ident),* $(,)?
        }
    } => {
        #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::default::Default)]
        #[allow(missing_docs)]
        $vis struct $AttributeName {
            $($vis $attribute: ::std::option::Option<::std::option::Option<::std::string::String>>,)*
        }
    };
}
