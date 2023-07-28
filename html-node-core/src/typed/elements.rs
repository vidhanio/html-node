//! Predefined HTML elements.

use crate::typed_elements;

typed_elements! { pub
    // Main root [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#main_root]
    html {
        xmlns,
    };

    // Document metadata [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#document_metadata]
    base {
        href,
        target,
    };
    head {};
    link {
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
    meta {
        charset,
        content,
        http_equiv,
        name,
    };
    style {
        media,
        blocking,
    };
    title {};

    // Sectioning root [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#sectioning_root]
    body {};

    // Content sectioning [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#content_sectioning]
    address {};
    article {};
    aside {};
    footer {};
    header {};
    h1 {};
    h2 {};
    h3 {};
    h4 {};
    h5 {};
    h6 {};
    hgroup {};
    main {};
    nav {};
    section {};
    search {};

    // Text content [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#text_content]
    blockquote {
        cite,
    };
    dd {};
    div {};
    dl {};
    dt {};
    figcaption {};
    figure {};
    hr {};
    li {
        value,
    };
    menu {};
    ol {
        reversed,
        start,
        r#type,
    };
    p {};
    pre {};
    ul {};

    // Inline text semantics [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#inline_text_semantics]
    a {
        download,
        href,
        hreflang,
        ping,
        referrerpolicy,
        rel,
        target,
        r#type,
    };
    abbr {};
    b {};
    bdi {};
    bdo {};
    br {};
    cite {};
    code {};
    data {
        value,
    };
    dfn {};
    em {};
    i {};
    kbd {};
    mark {};
    q {
        cite,
    };
    rp {};
    rt {};
    ruby {};
    s {};
    samp {};
    small {};
    span {};
    strong {};
    sub {};
    sup {};
    time {
        datetime,
    };
    u {};
    var {};
    wbr {};

    // Image and multimedia [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#image_and_multimedia]
    area {
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
    audio {
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
    img {
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
    map {
        name,
    };
    track {
        default,
        kind,
        label,
        src,
        srclang,
    };
    video {
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
    embed {
        height,
        src,
        r#type,
        width,
    };
    iframe {
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
    object {
        data,
        form,
        height,
        name,
        r#type,
        usemap,
        width,
    };
    picture {};
    portal {
        referrerpolicy,
        src,
    };
    source {
        r#type,
        src,
        srcset,
        sizes,
        media,
        height,
        width,
    };

    // SVG and MathML [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#svg_and_mathml]
    svg {
        height,
        preserveaspectratio,
        viewBox,
        width,
        x,
        y,
    };
    math {
        display,
    };

    // Scripting [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#scripting]
    canvas {
        height,
        width,
    };
    noscript {};
    script {
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
    del {
        cite,
        datetime,
    };
    ins {
        cite,
        datetime,
    };

    // Table content [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#table_content]
    caption {};
    col {
        span,
    };
    colgroup {
        span,
    };
    table {};
    tbody {};
    td {
        colspan,
        headers,
        rowspan,
    };
    tfoot {};
    th {
        abbr,
        colspan,
        headers,
        rowspan,
        scope,
    };
    thead {};
    tr {};

    // Forms [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#forms]
    button {
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
    datalist {};
    fieldset {
        disabled,
        form,
        name,
    };
    form {
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
    input {
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
    label {
        r#for,
    };
    legend {};
    meter {
        min,
        max,
        low,
        high,
        optimum,
    };
    optgroup {
        disabled,
        label,
    };
    option {
        disabled,
        label,
        selected,
        value,
    };
    output {
        r#for,
        form,
        name,
    };
    progress {
        max,
        value,
    };
    select {
        autocomplete,
        disabled,
        form,
        multiple,
        name,
        required,
        size,
    };
    textarea {
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
    details {
        open,
    };
    dialog {
        open,
    };

    // Web Components [https://developer.mozilla.org/en-US/docs/Web/HTML/Element#web_components]
    slot {
        name,
    };
    template {
        shadowrootmode,
    };
}
