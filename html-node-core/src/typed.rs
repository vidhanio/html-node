//! HTML types.
#![allow(clippy::module_name_repetitions)]
#![allow(non_snake_case)]

use crate::{Element, Node};

/// A typed HTML element.
pub trait TypedElement: Default {
    /// The name of the element.
    const NAME: &'static str;

    /// The attributes of the element.
    type Attributes;

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

pub mod elements {
    //! Predefined HTML elements.

    /// Helper macro to define typed HTML elements.
    #[macro_export]
    macro_rules! typed_element {
        ($($Name:ident[$name:literal] $Attributes:ident { $($attribute:ident),* $(,)? };)*) => {
            $(
                $crate::typed_element!(@ $Name[$name] $Attributes {
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
                });
            )*
        };
        (@ $Name:ident[$name:literal] $Attributes:ident { $($attribute:ident),* $(,)? }) => {
            #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::default::Default)]
            #[allow(non_camel_case_types)]
            #[allow(missing_docs)]
            pub struct $Name {
                pub attributes: $Attributes,
                pub data_attributes: ::std::vec::Vec<(::std::string::String, ::std::option::Option<::std::string::String>)>,
                pub aria_attributes: ::std::vec::Vec<(::std::string::String, ::std::option::Option<::std::string::String>)>,
            }

            impl $crate::typed::TypedElement for $Name {
                const NAME: &'static str = $name;
                type Attributes = $Attributes;

                fn from_attributes(
                    attributes: Self::Attributes,
                    data_attributes: ::std::vec::Vec<(::std::string::String, ::std::option::Option<::std::string::String>)>,
                    aria_attributes: ::std::vec::Vec<(::std::string::String, ::std::option::Option<::std::string::String>)>,
                ) -> Self {
                    Self { attributes, data_attributes, aria_attributes }
                }

                fn into_element(self, children: ::std::option::Option<::std::vec::Vec<$crate::Node>>) -> $crate::Element {
                    use ::std::iter::Iterator as _;

                    $crate::Element {
                        name: Self::NAME.into(),
                        attributes: [$((stringify!($attribute), self.attributes.$attribute)),*]
                            .into_iter()
                            .flat_map(|(key, maybe_value)| {
                                maybe_value.map(|value| (key.strip_prefix("r#").unwrap_or(key).replace('_', "-"), value))
                            })
                            .chain(self.data_attributes)
                            .chain(self.aria_attributes)
                            .collect(),
                        children,
                    }
                }
            }

            #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::default::Default)]
            #[allow(missing_docs)]
            pub struct $Attributes {
                $(pub $attribute: Option<Option<::std::string::String>>),*
            }
        }
    }
    pub use typed_element;

    typed_element! {
        // Main root [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#main_root]
        html["html"] HtmlAttributes {
            xmlns,
        };

        // Document metadata [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#document_metadata]
        base["base"] BaseAttributes {
            href,
            target,
        };
        head["head"] HeadAttributes {};
        link["link"] LinkAttributes {
            r#as,
            crossorigin,
            disabled,
            fetchpriority,
            href,
            hreflang,
            imagesizes,
            imagesrcset,
            integrity,
            media,
            prefetch,
            referrerpolicy,
            rel,
            sizes,
            r#type,
            blocking,
        };
        meta["meta"] MetaAttributes {
            charset,
            content,
            http_equiv,
            name,
        };
        style["style"] StyleAttributes {
            media,
            blocking,
        };
        title["title"] TitleAttributes {};

        // Sectioning root [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#sectioning_root]
        body["body"] BodyAttributes {};

        // Content sectioning [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#content_sectioning]
        address["address"] AddressAttributes {};
        article["article"] ArticleAttributes {};
        aside["aside"] AsideAttributes {};
        footer["footer"] FooterAttributes {};
        header["header"] HeaderAttributes {};
        h1["h1"] H1Attributes {};
        h2["h2"] H2Attributes {};
        h3["h3"] H3Attributes {};
        h4["h4"] H4Attributes {};
        h5["h5"] H5Attributes {};
        h6["h6"] H6Attributes {};
        hgroup["hgroup"] HgroupAttributes {};
        main["main"] MainAttributes {};
        nav["nav"] NavAttributes {};
        section["section"] SectionAttributes {};
        search["search"] SearchAttributes {};

        // Text content [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#text_content]
        blockquote["blockquote"] BlockquoteAttributes {
            cite,
        };
        dd["dd"] DdAttributes {};
        div["div"] DivAttributes {};
        dl["dl"] DlAttributes {};
        dt["dt"] DtAttributes {};
        figcaption["figcaption"] FigcaptionAttributes {};
        figure["figure"] FigureAttributes {};
        hr["hr"] HrAttributes {};
        li["li"] LiAttributes {
            value,
        };
        menu["menu"] MenuAttributes {};
        ol["ol"] OlAttributes {
            reversed,
            start,
            r#type,
        };
        p["p"] PAttributes {};
        pre["pre"] PreAttributes {};
        ul["ul"] UlAttributes {};

        // Inline text semantics [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#inline_text_semantics]
        a["a"] AAttributes {
            download,
            href,
            hreflang,
            ping,
            referrerpolicy,
            rel,
            target,
            r#type,
        };
        abbr["abbr"] AbbrAttributes {};
        b["b"] BAttributes {};
        bdi["bdi"] BdiAttributes {};
        bdo["bdo"] BdoAttributes {};
        br["br"] BrAttributes {};
        cite["cite"] CiteAttributes {};
        code["code"] CodeAttributes {};
        data["data"] DataAttributes {
            value,
        };
        dfn["dfn"] DfnAttributes {};
        em["em"] EmAttributes {};
        i["i"] IAttributes {};
        kbd["kbd"] KbdAttributes {};
        mark["mark"] MarkAttributes {};
        q["q"] QAttributes {
            cite,
        };
        rp["rp"] RpAttributes {};
        rt["rt"] RtAttributes {};
        ruby["ruby"] RubyAttributes {};
        s["s"] SAttributes {};
        samp["samp"] SampAttributes {};
        small["small"] SmallAttributes {};
        span["span"] SpanAttributes {};
        strong["strong"] StrongAttributes {};
        sub["sub"] SubAttributes {};
        sup["sup"] SupAttributes {};
        time["time"] TimeAttributes {
            datetime,
        };
        u["u"] UAttributes {};
        var["var"] VarAttributes {};
        wbr["wbr"] WbrAttributes {};

        // Image and multimedia [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#image_and_multimedia]
        area["area"] AreaAttributes {
            alt,
            coords,
            download,
            href,
            ping,
            referrerpolicy,
            rel,
            shape,
            target,
        };
        audio["audio"] AudioAttributes {
            autoplay,
            controls,
            controlslist,
            crossorigin,
            disableremoteplayback,
            r#loop,
            muted,
            preload,
            src,
        };
        img["img"] ImgAttributes {
            alt,
            crossorigin,
            decoding,
            elementtiming,
            fetchpriority,
            height,
            ismap,
            loading,
            referrerpolicy,
            sizes,
            src,
            srcset,
            width,
            usemap,
        };
        map["map"] MapAttributes {
            name,
        };
        track["track"] TrackAttributes {
            default,
            kind,
            label,
            src,
            srclang,
        };
        video["video"] VideoAttributes {
            autoplay,
            controls,
            controlslist,
            crossorigin,
            disablepictureinpicture,
            disableremoteplayback,
            height,
            r#loop,
            muted,
            playsinline,
            poster,
            preload,
            src,
            width,
        };

        // Embedded content [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#embedded_content]
        embed["embed"] EmbedAttributes {
            height,
            src,
            r#type,
            width,
        };
        iframe["iframe"] IframeAttributes {
            allow,
            allowfullscreen,
            allowpaymentrequest,
            credentialless,
            csp,
            height,
            loading,
            name,
            referrerpolicy,
            sandbox,
            src,
            srcdoc,
            width,
        };
        object["object"] ObjectAttributes {
            data,
            form,
            height,
            name,
            r#type,
            usemap,
            width,
        };
        picture["picture"] PictureAttributes {};
        portal["portal"] PortalAttributes {
            referrerpolicy,
            src,
        };
        source["source"] SourceAttributes {
            r#type,
            src,
            srcset,
            sizes,
            media,
            height,
            width,
        };

        // SVG and MathML [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#svg_and_mathml]
        svg["svg"] SvgAttributes {
            height,
            preserveaspectratio,
            viewBox,
            width,
            x,
            y,
        };
        math["math"] MathAttributes {
            display,
        };

        // Scripting [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#scripting]
        canvas["canvas"] CanvasAttributes {
            height,
            width,
        };
        noscript["noscript"] NoscriptAttributes {};
        script["script"] ScriptAttributes {
            r#async,
            crossorigin,
            defer,
            fetchpriority,
            integrity,
            nomodule,
            referrerpolicy,
            src,
            r#type,
            blocking,
        };

        // Demarcating edits [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#demarcating_edits]
        del["del"] DelAttributes {
            cite,
            datetime,
        };
        ins["ins"] InsAttributes {
            cite,
            datetime,
        };

        // Table content [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#table_content]
        caption["caption"] CaptionAttributes {};
        col["col"] ColAttributes {
            span,
        };
        colgroup["colgroup"] ColgroupAttributes {
            span,
        };
        table["table"] TableAttributes {};
        tbody["tbody"] TbodyAttributes {};
        td["td"] TdAttributes {
            colspan,
            headers,
            rowspan,
        };
        tfoot["tfoot"] TfootAttributes {};
        th["th"] ThAttributes {
            abbr,
            colspan,
            headers,
            rowspan,
            scope,
        };
        thead["thead"] TheadAttributes {};
        tr["tr"] TrAttributes {};

        // Forms [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#forms]
        button["button"] ButtonAttributes {
            disabled,
            form,
            formaction,
            formenctype,
            formmethod,
            formnovalidate,
            formtarget,
            name,
            popovertarget,
            popovertargetaction,
            r#type,
            value,
        };
        datalist["datalist"] DatalistAttributes {};
        fieldset["fieldset"] FieldsetAttributes {
            disabled,
            form,
            name,
        };
        form["form"] FormAttributes {
            acceptcharset,
            autocomplete,
            name,
            rel,
            action,
            enctype,
            method,
            novalidate,
            target,
        };
        input["input"] InputAttributes {
            accept,
            alt,
            autocomplete,
            capture,
            checked,
            dirname,
            disabled,
            form,
            formaction,
            formenctype,
            formmethod,
            formnovalidate,
            formtarget,
            height,
            list,
            max,
            maxlength,
            min,
            minlength,
            multiple,
            name,
            pattern,
            placeholder,
            popovertarget,
            popovertargetaction,
            readonly,
            required,
            size,
            src,
            step,
            r#type,
            value,
            width,
            autocorrect,
            incremental,
            mozactionhint,
            orient,
            results,
            webkitdirectory,
        };
        label["label"] LabelAttributes {
            r#for,
        };
        legend["legend"] LegendAttributes {};
        meter["meter"] MeterAttributes {
            min,
            max,
            low,
            high,
            optimum,
        };
        optgroup["optgroup"] OptgroupAttributes {
            disabled,
            label,
        };
        option["option"] OptionAttributes {
            disabled,
            label,
            selected,
            value,
        };
        output["output"] OutputAttributes {
            r#for,
            form,
            name,
        };
        progress["progress"] ProgressAttributes {
            max,
            value,
        };
        select["select"] SelectAttributes {
            autocomplete,
            disabled,
            form,
            multiple,
            name,
            required,
            size,
        };
        textarea["textarea"] TextareaAttributes {
            autocomplete,
            autocorrect,
            cols,
            dirname,
            disabled,
            form,
            maxlength,
            minlength,
            name,
            placeholder,
            readonly,
            required,
            rows,
            wrap,
        };

        // Interactive elements [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#interactive_elements]
        details["details"] DetailsAttributes {
            open,
        };
        dialog["dialog"] DialogAttributes {
            open,
        };

        // Web Components [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#web_components]
        slot["slot"] SlotAttributes {
            name,
        };
        template["template"] TemplateAttributes {
            shadowrootmode,
        };
    }
}
