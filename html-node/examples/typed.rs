// if you use `typed_html!`, your macro can be checked at compile-time for
// unknown elements and attributes!

// to do this, you must enable the `typed` feature of `html-node` in your
// dependencies.

// if you want to pull all the default html elements into the scope, you can
// `use html_node::typed::elements::*;`.

use html_node::{
    text,
    typed::{elements::*, TypedElement},
    typed_element, typed_html, Element, Node,
};

fn main() {
    // this will compile!
    let html = typed_html! {
        <div id="my-element" data-i-can-do-whatever-here="true" class="test-class">
            <p>{text!("Hello, world!")}</p>
        </div>
    };

    println!("{:#}", html);

    // this will not compile with error:
    // struct `html_node::typed::elements::DivAttributes` has no field named
    // `my_funky_attribute`
    // as you can see, it turns hyphens into underscores when generating the
    // attribute names. these will be converted back to hyphens when rendering
    // the html though, so don't worry!

    // let html = typed_html! {
    //     <div my-funky-attribute="test">
    //         <p>{text!("Hello, world!")}</p>
    //     </div>
    // };

    // you can also use custom elements! (scroll down to see implementation)
    let html = typed_html! {
        <CustomElement test-attribute="test">
            <p>{text!("Hello, world!")}</p>
        </CustomElement>
    };

    println!("{:#}", html);

    // you can also use custom elements with the macro-based implementation!
    // a benefit of this form is that it automatically adds all global attributes
    // (https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes) to
    // your element, so you don't have to add common attributes like `id`, `class`,
    // etc. yourself.
    let html = typed_html! {
        <CustomElement2 my-cool-attr="test">
            <p>{text!("Hello, world!")}</p>
        </CustomElement2>
    };

    println!("{:#}", html);
}

/// MANUAL IMPLEMENTATION EXAMPLE
#[derive(Default)]
struct CustomElement {
    attributes: CustomElementAttributes,
    data_attributes: Vec<(String, Option<String>)>,
    aria_attributes: Vec<(String, Option<String>)>,
}

#[derive(Default)]
struct CustomElementAttributes {
    // Attributes must be `Option<Option<String>>`.
    // The first layer represents whether the attribute is present.
    // The second layer represents whether the attribute has a value.
    test_attribute: Option<Option<String>>,
}

impl TypedElement for CustomElement {
    type Attributes = CustomElementAttributes;

    const NAME: &'static str = "custom-element";

    fn from_attributes(
        attributes: Self::Attributes,
        data_attributes: Vec<(String, Option<String>)>,
        aria_attributes: Vec<(String, Option<String>)>,
    ) -> Self {
        Self {
            attributes,
            data_attributes,
            aria_attributes,
        }
    }

    fn into_element(self, children: Option<Vec<Node>>) -> html_node_core::Element {
        Element {
            name: Self::NAME.to_string(),
            attributes: [("test-attribute".to_string(), self.attributes.test_attribute)]
                .into_iter()
                .filter_map(|(key, value)| value.map(|value| (key, value)))
                .chain(self.data_attributes)
                .chain(self.aria_attributes)
                .collect(),
            children,
        }
    }
}

// MACRO-BASED IMPLEMENTATION EXAMPLE
typed_element! {
    CustomElement2["custom-element-2"] CustomElement2Attributes {
        my_cool_attr,
    };
}
