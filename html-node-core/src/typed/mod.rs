#![allow(clippy::module_name_repetitions)]
#![allow(non_snake_case)]

pub mod elements;
#[doc(hidden)]
pub use paste::paste;

use crate::Node;

/// A typed HTML element.
pub trait TypedElement {
    /// The attributes of the element.
    type Attributes;

    /// Create an element from its attributes.
    fn from_attributes(
        attributes: Self::Attributes,
        other_attributes: Vec<(String, Option<String>)>,
    ) -> Self;

    /// Convert the typed element into a [`Node`].
    fn into_node(self, children: Option<Vec<Node>>) -> Node;
}

/// A typed set of HTML attributes.
pub trait TypedAttributes {
    /// Convert the typed attributes into a set of attributes.
    fn into_attributes(self) -> Vec<(String, Option<String>)>;
}

/// A typed attribute.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Attribute<T> {
    /// The attribute is present and has a value.
    ///
    /// ```html
    /// <div id="test"></div>
    /// ```
    Present(T),

    /// The attribute is present but has no value.
    ///
    /// ```html
    /// <div hidden></div>
    /// ```
    Empty,

    /// The attribute is not present.
    #[default]
    Missing,
}

impl<T> Attribute<T> {
    /// Convert the attribute into a double layered [`Option`].
    pub fn into_option(self) -> Option<Option<T>> {
        match self {
            Self::Present(value) => Some(Some(value)),
            Self::Empty => Some(None),
            Self::Missing => None,
        }
    }
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
    ($vis:vis $ElementName:ident $(($name:literal))? $([$AttributeName:ident])? $({ $($attribute:ident $(: $atype:ty)?),* $(,)? })?) => {
        $crate::typed_attributes!{
            ($vis $ElementName) $([$vis $AttributeName])? $({
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
                $($attribute $(: $atype)?),*
            })?
        }

        #[derive(::std::fmt::Debug, ::std::default::Default)]
        #[allow(non_camel_case_types)]
        #[allow(missing_docs)]
        $vis struct $ElementName {
            $vis attributes: <Self as $crate::typed::TypedElement>::Attributes,
            $vis other_attributes: ::std::vec::Vec<(::std::string::String, ::std::option::Option<::std::string::String>)>,
        }

        impl $crate::typed::TypedElement for $ElementName {
            type Attributes = $crate::typed_attributes!(@NAME ($ElementName) $([$AttributeName])?);

            fn from_attributes(
                attributes: Self::Attributes,
                other_attributes: ::std::vec::Vec<(::std::string::String, ::std::option::Option<::std::string::String>)>,
            ) -> Self {
                Self { attributes, other_attributes }
            }

            fn into_node(mut self, children: ::std::option::Option<::std::vec::Vec<$crate::Node>>) -> $crate::Node {
                let mut attributes = $crate::typed::TypedAttributes::into_attributes(self.attributes);
                attributes.append(&mut self.other_attributes);

                $crate::Node::Element(
                    $crate::Element {
                        name: ::std::convert::From::from($crate::typed_element!(@NAME_STR $ElementName$(($name))?)),
                        attributes,
                        children,
                    }
                )
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
        $(($vise:vis $ElementName:ident))? $([$visa:vis $AttributeName:ident])? {
            $($attribute:ident $(: $atype:ty)?),* $(,)?
        }
    } => {
        $crate::typed_attributes!(@STRUCT $(($vise $ElementName))? $([$visa $AttributeName])? { $($attribute $(: $atype)?),* });

        impl $crate::typed::TypedAttributes for $crate::typed_attributes!(@NAME $(($ElementName))? $([$AttributeName])?) {
            fn into_attributes(self) -> ::std::vec::Vec<(::std::string::String, ::std::option::Option<::std::string::String>)> {
                [$((::std::stringify!($attribute), self.$attribute.into_option().map(|opt| opt.map(|a| ::std::string::ToString::to_string(&a))))),*]
                    .into_iter()
                    .flat_map(|(key, maybe_value)| {
                        maybe_value.map(|value| (key.strip_prefix("r#").unwrap_or(key).replace('_', "-"), value))
                    })
                    .collect()
            }
        }
    };
    (($_vise:vis $_ElementName:ident) $([$_visa:vis $_AttributeName:ident])?) => {};
    (@NAME ($ElementName:ident)) => {
        $crate::typed::paste!([< $ElementName:camel Attributes >])
    };
    (@NAME $(($ElementName:ident))? [$AttributeName:ident]) => {
        $AttributeName
    };
    {
        @STRUCT ($vis:vis $ElementName:ident) {
            $($attribute:ident $(:$atype:ty)?),* $(,)?
        }
    } => {
        $crate::typed::paste! {
            #[derive(::std::fmt::Debug, ::std::default::Default)]
            #[allow(missing_docs)]
            $vis struct [< $ElementName:camel Attributes >] {
                $($vis $attribute: $crate::typed::Attribute<$crate::typed_attributes!(@ATTR_TYPE $($atype)?)>),*
            }
        }
    };
    {
        @STRUCT $(($_vis:vis $ElementName:ident))? [$vis:vis $AttributeName:ident] {
            $($attribute:ident $(: $atype:ty)?),* $(,)?
        }
    } => {
        #[derive(::std::fmt::Debug, ::std::default::Default)]
        #[allow(missing_docs)]
        $vis struct $AttributeName {
            $($vis $attribute: $crate::typed::Attribute<$crate::typed_attributes!(@ATTR_TYPE $($atype)?)>),*
        }
    };
    (@ATTR_TYPE $atype:ty) => {$atype};
    (@ATTR_TYPE) => {::std::string::String};
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! typed_component_attributes {
    {
        $(($vise:vis $ElementName:ident))? $([$visa:vis $AttributeName:ident])? {
            $($attribute:ident: $atype:ty),* $(,)?
        }
    } => {
        $crate::typed_component_attributes!(@STRUCT $(($vise $ElementName))? $([$visa $AttributeName])? { $($attribute: $atype),* });
    };
    (($_vise:vis $_ElementName:ident) $([$_visa:vis $_AttributeName:ident])?) => {};
    {
        @STRUCT ($vis:vis $ElementName:ident) {
            $($attribute:ident: $atype:ty),* $(,)?
        }
    } => {
        $crate::typed::paste! {
            #[derive(::std::fmt::Debug)]
            #[allow(missing_docs)]
            $vis struct [< $ElementName:camel Attributes >] {
                $($vis $attribute: $atype),*
            }
        }
    };
    {
        @STRUCT $(($_vis:vis $ElementName:ident))? [$vis:vis $AttributeName:ident] {
            $($attribute:ident: $atype:ty),* $(,)?
        }
    } => {
        #[derive(::std::fmt::Debug)]
        #[allow(missing_docs)]
        $vis struct $AttributeName {
            $($vis $attribute: $atype),*
        }
    };
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! typed_component {
    (
        $vis:vis $ElementName:ident $([$AttributeName:ident])? $({
                $($attribute:ident $(: $atype:ty)?),* $(,)?
        })?;

        |$attributes:pat_param, $extra_attributes:pat_param, $children:pat_param| $body:expr
    ) => {
        $crate::typed_component_attributes!{
            ($vis $ElementName) $([$vis $AttributeName])? $({
                $($attribute $(: $atype)?),*
            })?
        }

        #[derive(::std::fmt::Debug)]
        #[allow(non_camel_case_types)]
        #[allow(missing_docs)]
        $vis struct $ElementName {
            $vis attributes: <Self as $crate::typed::TypedElement>::Attributes,
            $vis extra_attributes: ::std::vec::Vec<(::std::string::String, ::std::option::Option<::std::string::String>)>,
        }

        impl $crate::typed::TypedElement for $ElementName {
            type Attributes = $crate::typed_attributes!(@NAME ($ElementName) $([$AttributeName])?);

            fn from_attributes(
                attributes: Self::Attributes,
                extra_attributes: ::std::vec::Vec<(::std::string::String, ::std::option::Option<::std::string::String>)>,
            ) -> Self {
                Self { attributes, extra_attributes }
            }

            fn into_node(self, $children: ::std::option::Option<::std::vec::Vec<$crate::Node>>) -> $crate::Node {
                let $attributes = self.attributes;
                let $extra_attributes = self.extra_attributes;

                {
                    $body
                }
            }
        }
    };
}
