//! Static data tables ported from SVGO's plugins/_collections.js
//!
//! Based on https://www.w3.org/TR/SVG11/intro.html#Definitions

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

/// Editor namespace URIs (e.g. sodipodi, inkscape) that should be stripped.
pub static EDITOR_NAMESPACES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "http://creativecommons.org/ns#",
        "http://inkscape.sourceforge.net/DTD/sodipodi-0.dtd",
        "http://krita.org/namespaces/svg/krita",
        "http://ns.adobe.com/AdobeIllustrator/10.0/",
        "http://ns.adobe.com/AdobeSVGViewerExtensions/3.0/",
        "http://ns.adobe.com/Extensibility/1.0/",
        "http://ns.adobe.com/Flows/1.0/",
        "http://ns.adobe.com/GenericCustomNamespace/1.0/",
        "http://ns.adobe.com/Graphs/1.0/",
        "http://ns.adobe.com/ImageReplacement/1.0/",
        "http://ns.adobe.com/SaveForWeb/1.0/",
        "http://ns.adobe.com/Variables/1.0/",
        "http://ns.adobe.com/XPath/1.0/",
        "http://purl.org/dc/elements/1.1/",
        "http://purl.org/dc/terms/",
        "http://schemas.microsoft.com/visio/2003/SVGExtensions/",
        "http://sodipodi.sourceforge.net/DTD/sodipodi-0.0.dtd",
        "http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd",
        "http://taptrix.org/namespaces/svg_extensions",
        "http://www.bohemiancoding.com/sketch/ns",
        "http://www.evolus.vn/Namespace/Pencil",
        "http://www.frees.org/2005/11/interop",
        "http://www.graphviz.org/xdot/",
        "http://www.inkscape.org/namespaces/inkscape",
        "http://www.serif.com/",
        "http://www.vector.evaxdesign.sk",
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
        "http://www.w3.org/2000/svg",
    ])
});

/// Attributes that are inherited by child elements.
pub static INHERITABLE_ATTRS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "clip-rule",
        "color-interpolation-filters",
        "color-interpolation",
        "color-profile",
        "color-rendering",
        "color",
        "cursor",
        "direction",
        "dominant-baseline",
        "fill-opacity",
        "fill-rule",
        "fill",
        "font-family",
        "font-size-adjust",
        "font-size",
        "font-stretch",
        "font-style",
        "font-variant",
        "font-weight",
        "font",
        "glyph-orientation-horizontal",
        "glyph-orientation-vertical",
        "image-rendering",
        "letter-spacing",
        "marker-end",
        "marker-mid",
        "marker-start",
        "marker",
        "opacity",
        "paint-order",
        "pointer-events",
        "shape-rendering",
        "stroke-dasharray",
        "stroke-dashoffset",
        "stroke-linecap",
        "stroke-linejoin",
        "stroke-miterlimit",
        "stroke-opacity",
        "stroke-width",
        "stroke",
        "text-anchor",
        "text-decoration",
        "text-rendering",
        "transform",
        "visibility",
        "word-spacing",
        "writing-mode",
    ])
});

/// Properties whose values can reference other elements (e.g. url(#id), href).
pub static REFERENCES_PROPS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "clip-path",
        "color-profile",
        "fill",
        "filter",
        "marker-end",
        "marker-mid",
        "marker-start",
        "mask",
        "stroke",
        "style",
    ])
});

/// CSS color names to hex values.
pub static COLORS_NAMES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("aliceblue", "#f0f8ff"),
        ("antiquewhite", "#faebd7"),
        ("aqua", "#0ff"),
        ("aquamarine", "#7fffd4"),
        ("azure", "#f0ffff"),
        ("beige", "#f5f5dc"),
        ("bisque", "#ffe4c4"),
        ("black", "#000"),
        ("blanchedalmond", "#ffebcd"),
        ("blue", "#00f"),
        ("blueviolet", "#8a2be2"),
        ("brown", "#a52a2a"),
        ("burlywood", "#deb887"),
        ("cadetblue", "#5f9ea0"),
        ("chartreuse", "#7fff00"),
        ("chocolate", "#d2691e"),
        ("coral", "#ff7f50"),
        ("cornflowerblue", "#6495ed"),
        ("cornsilk", "#fff8dc"),
        ("crimson", "#dc143c"),
        ("cyan", "#0ff"),
        ("darkblue", "#00008b"),
        ("darkcyan", "#008b8b"),
        ("darkgoldenrod", "#b8860b"),
        ("darkgray", "#a9a9a9"),
        ("darkgreen", "#006400"),
        ("darkgrey", "#a9a9a9"),
        ("darkkhaki", "#bdb76b"),
        ("darkmagenta", "#8b008b"),
        ("darkolivegreen", "#556b2f"),
        ("darkorange", "#ff8c00"),
        ("darkorchid", "#9932cc"),
        ("darkred", "#8b0000"),
        ("darksalmon", "#e9967a"),
        ("darkseagreen", "#8fbc8f"),
        ("darkslateblue", "#483d8b"),
        ("darkslategray", "#2f4f4f"),
        ("darkslategrey", "#2f4f4f"),
        ("darkturquoise", "#00ced1"),
        ("darkviolet", "#9400d3"),
        ("deeppink", "#ff1493"),
        ("deepskyblue", "#00bfff"),
        ("dimgray", "#696969"),
        ("dimgrey", "#696969"),
        ("dodgerblue", "#1e90ff"),
        ("firebrick", "#b22222"),
        ("floralwhite", "#fffaf0"),
        ("forestgreen", "#228b22"),
        ("fuchsia", "#f0f"),
        ("gainsboro", "#dcdcdc"),
        ("ghostwhite", "#f8f8ff"),
        ("gold", "#ffd700"),
        ("goldenrod", "#daa520"),
        ("gray", "#808080"),
        ("green", "#008000"),
        ("greenyellow", "#adff2f"),
        ("grey", "#808080"),
        ("honeydew", "#f0fff0"),
        ("hotpink", "#ff69b4"),
        ("indianred", "#cd5c5c"),
        ("indigo", "#4b0082"),
        ("ivory", "#fffff0"),
        ("khaki", "#f0e68c"),
        ("lavender", "#e6e6fa"),
        ("lavenderblush", "#fff0f5"),
        ("lawngreen", "#7cfc00"),
        ("lemonchiffon", "#fffacd"),
        ("lightblue", "#add8e6"),
        ("lightcoral", "#f08080"),
        ("lightcyan", "#e0ffff"),
        ("lightgoldenrodyellow", "#fafad2"),
        ("lightgray", "#d3d3d3"),
        ("lightgreen", "#90ee90"),
        ("lightgrey", "#d3d3d3"),
        ("lightpink", "#ffb6c1"),
        ("lightsalmon", "#ffa07a"),
        ("lightseagreen", "#20b2aa"),
        ("lightskyblue", "#87cefa"),
        ("lightslategray", "#789"),
        ("lightslategrey", "#789"),
        ("lightsteelblue", "#b0c4de"),
        ("lightyellow", "#ffffe0"),
        ("lime", "#0f0"),
        ("limegreen", "#32cd32"),
        ("linen", "#faf0e6"),
        ("magenta", "#f0f"),
        ("maroon", "#800000"),
        ("mediumaquamarine", "#66cdaa"),
        ("mediumblue", "#0000cd"),
        ("mediumorchid", "#ba55d3"),
        ("mediumpurple", "#9370db"),
        ("mediumseagreen", "#3cb371"),
        ("mediumslateblue", "#7b68ee"),
        ("mediumspringgreen", "#00fa9a"),
        ("mediumturquoise", "#48d1cc"),
        ("mediumvioletred", "#c71585"),
        ("midnightblue", "#191970"),
        ("mintcream", "#f5fffa"),
        ("mistyrose", "#ffe4e1"),
        ("moccasin", "#ffe4b5"),
        ("navajowhite", "#ffdead"),
        ("navy", "#000080"),
        ("oldlace", "#fdf5e6"),
        ("olive", "#808000"),
        ("olivedrab", "#6b8e23"),
        ("orange", "#ffa500"),
        ("orangered", "#ff4500"),
        ("orchid", "#da70d6"),
        ("palegoldenrod", "#eee8aa"),
        ("palegreen", "#98fb98"),
        ("paleturquoise", "#afeeee"),
        ("palevioletred", "#db7093"),
        ("papayawhip", "#ffefd5"),
        ("peachpuff", "#ffdab9"),
        ("peru", "#cd853f"),
        ("pink", "#ffc0cb"),
        ("plum", "#dda0dd"),
        ("powderblue", "#b0e0e6"),
        ("purple", "#800080"),
        ("rebeccapurple", "#663399"),
        ("red", "#f00"),
        ("rosybrown", "#bc8f8f"),
        ("royalblue", "#4169e1"),
        ("saddlebrown", "#8b4513"),
        ("salmon", "#fa8072"),
        ("sandybrown", "#f4a460"),
        ("seagreen", "#2e8b57"),
        ("seashell", "#fff5ee"),
        ("sienna", "#a0522d"),
        ("silver", "#c0c0c0"),
        ("skyblue", "#87ceeb"),
        ("slateblue", "#6a5acd"),
        ("slategray", "#708090"),
        ("slategrey", "#708090"),
        ("snow", "#fffafa"),
        ("springgreen", "#00ff7f"),
        ("steelblue", "#4682b4"),
        ("tan", "#d2b48c"),
        ("teal", "#008080"),
        ("thistle", "#d8bfd8"),
        ("tomato", "#ff6347"),
        ("turquoise", "#40e0d0"),
        ("violet", "#ee82ee"),
        ("wheat", "#f5deb3"),
        ("white", "#fff"),
        ("whitesmoke", "#f5f5f5"),
        ("yellow", "#ff0"),
        ("yellowgreen", "#9acd32"),
    ])
});

/// Element group classifications.
pub static ELEMS_GROUPS: LazyLock<HashMap<&'static str, HashSet<&'static str>>> =
    LazyLock::new(|| {
        let mut m = HashMap::new();
        m.insert(
            "animation",
            HashSet::from([
                "animate",
                "animateColor",
                "animateMotion",
                "animateTransform",
                "set",
            ]),
        );
        m.insert("descriptive", HashSet::from(["desc", "metadata", "title"]));
        m.insert(
            "shape",
            HashSet::from([
                "circle", "ellipse", "line", "path", "polygon", "polyline", "rect",
            ]),
        );
        m.insert(
            "structural",
            HashSet::from(["defs", "g", "svg", "symbol", "use"]),
        );
        m.insert(
            "paintServer",
            HashSet::from([
                "hatch",
                "linearGradient",
                "meshGradient",
                "pattern",
                "radialGradient",
                "solidColor",
            ]),
        );
        m.insert(
            "nonRendering",
            HashSet::from([
                "clipPath",
                "filter",
                "linearGradient",
                "marker",
                "mask",
                "pattern",
                "radialGradient",
                "solidColor",
                "symbol",
            ]),
        );
        m.insert(
            "container",
            HashSet::from([
                "a",
                "defs",
                "foreignObject",
                "g",
                "marker",
                "mask",
                "missing-glyph",
                "pattern",
                "svg",
                "switch",
                "symbol",
            ]),
        );
        m.insert(
            "textContent",
            HashSet::from([
                "a",
                "altGlyph",
                "altGlyphDef",
                "altGlyphItem",
                "glyph",
                "glyphRef",
                "text",
                "textPath",
                "tref",
                "tspan",
            ]),
        );
        m.insert(
            "textContentChild",
            HashSet::from(["altGlyph", "textPath", "tref", "tspan"]),
        );
        m.insert(
            "lightSource",
            HashSet::from([
                "feDiffuseLighting",
                "feDistantLight",
                "fePointLight",
                "feSpecularLighting",
                "feSpotLight",
            ]),
        );
        m.insert(
            "filterPrimitive",
            HashSet::from([
                "feBlend",
                "feColorMatrix",
                "feComponentTransfer",
                "feComposite",
                "feConvolveMatrix",
                "feDiffuseLighting",
                "feDisplacementMap",
                "feDropShadow",
                "feFlood",
                "feFuncA",
                "feFuncB",
                "feFuncG",
                "feFuncR",
                "feGaussianBlur",
                "feImage",
                "feMerge",
                "feMergeNode",
                "feMorphology",
                "feOffset",
                "feSpecularLighting",
                "feTile",
                "feTurbulence",
            ]),
        );
        m
    });

/// Attribute groups - sets of related attributes by category.
pub static ATTRS_GROUPS: LazyLock<HashMap<&'static str, HashSet<&'static str>>> =
    LazyLock::new(|| {
        let mut m = HashMap::new();
        m.insert(
            "animationAddition",
            HashSet::from(["additive", "accumulate"]),
        );
        m.insert(
            "animationAttributeTarget",
            HashSet::from(["attributeType", "attributeName"]),
        );
        m.insert(
            "animationEvent",
            HashSet::from(["onbegin", "onend", "onrepeat", "onload"]),
        );
        m.insert(
            "animationTiming",
            HashSet::from([
                "begin",
                "dur",
                "end",
                "fill",
                "max",
                "min",
                "repeatCount",
                "repeatDur",
                "restart",
            ]),
        );
        m.insert(
            "animationValue",
            HashSet::from([
                "by",
                "calcMode",
                "from",
                "keySplines",
                "keyTimes",
                "to",
                "values",
            ]),
        );
        m.insert(
            "conditionalProcessing",
            HashSet::from(["requiredExtensions", "requiredFeatures", "systemLanguage"]),
        );
        m.insert(
            "core",
            HashSet::from(["id", "tabindex", "xml:base", "xml:lang", "xml:space"]),
        );
        m.insert(
            "graphicalEvent",
            HashSet::from([
                "onactivate",
                "onclick",
                "onfocusin",
                "onfocusout",
                "onload",
                "onmousedown",
                "onmousemove",
                "onmouseout",
                "onmouseover",
                "onmouseup",
            ]),
        );
        m.insert(
            "presentation",
            HashSet::from([
                "alignment-baseline",
                "baseline-shift",
                "clip-path",
                "clip-rule",
                "clip",
                "color-interpolation-filters",
                "color-interpolation",
                "color-profile",
                "color-rendering",
                "color",
                "cursor",
                "direction",
                "display",
                "dominant-baseline",
                "enable-background",
                "fill-opacity",
                "fill-rule",
                "fill",
                "filter",
                "flood-color",
                "flood-opacity",
                "font-family",
                "font-size-adjust",
                "font-size",
                "font-stretch",
                "font-style",
                "font-variant",
                "font-weight",
                "glyph-orientation-horizontal",
                "glyph-orientation-vertical",
                "image-rendering",
                "letter-spacing",
                "lighting-color",
                "marker-end",
                "marker-mid",
                "marker-start",
                "mask",
                "opacity",
                "overflow",
                "paint-order",
                "pointer-events",
                "shape-rendering",
                "stop-color",
                "stop-opacity",
                "stroke-dasharray",
                "stroke-dashoffset",
                "stroke-linecap",
                "stroke-linejoin",
                "stroke-miterlimit",
                "stroke-opacity",
                "stroke-width",
                "stroke",
                "text-anchor",
                "text-decoration",
                "text-overflow",
                "text-rendering",
                "transform-origin",
                "transform",
                "unicode-bidi",
                "vector-effect",
                "visibility",
                "word-spacing",
                "writing-mode",
            ]),
        );
        m.insert(
            "xlink",
            HashSet::from([
                "xlink:actuate",
                "xlink:arcrole",
                "xlink:href",
                "xlink:role",
                "xlink:show",
                "xlink:title",
                "xlink:type",
            ]),
        );
        m.insert(
            "documentEvent",
            HashSet::from([
                "onabort", "onerror", "onresize", "onscroll", "onunload", "onzoom",
            ]),
        );
        m.insert(
            "documentElementEvent",
            HashSet::from(["oncopy", "oncut", "onpaste"]),
        );
        m.insert(
            "filterPrimitive",
            HashSet::from(["x", "y", "width", "height", "result"]),
        );
        m.insert(
            "transferFunction",
            HashSet::from([
                "amplitude",
                "exponent",
                "intercept",
                "offset",
                "slope",
                "tableValues",
                "type",
            ]),
        );
        m
    });

/// Default values for presentation attributes.
pub static ATTRS_GROUPS_DEFAULTS: LazyLock<
    HashMap<&'static str, HashMap<&'static str, &'static str>>,
> = LazyLock::new(|| {
    let mut m = HashMap::new();
    let mut core = HashMap::new();
    core.insert("xml:space", "default");
    m.insert("core", core);
    let mut pres = HashMap::new();
    pres.insert("clip", "auto");
    pres.insert("clip-path", "none");
    pres.insert("clip-rule", "nonzero");
    pres.insert("mask", "none");
    pres.insert("opacity", "1");
    pres.insert("stop-color", "#000");
    pres.insert("stop-opacity", "1");
    pres.insert("fill-opacity", "1");
    pres.insert("fill-rule", "nonzero");
    pres.insert("fill", "#000");
    pres.insert("stroke", "none");
    pres.insert("stroke-width", "1");
    pres.insert("stroke-linecap", "butt");
    pres.insert("stroke-linejoin", "miter");
    pres.insert("stroke-miterlimit", "4");
    pres.insert("stroke-dasharray", "none");
    pres.insert("stroke-dashoffset", "0");
    pres.insert("stroke-opacity", "1");
    pres.insert("paint-order", "normal");
    pres.insert("vector-effect", "none");
    pres.insert("display", "inline");
    pres.insert("visibility", "visible");
    pres.insert("marker-start", "none");
    pres.insert("marker-mid", "none");
    pres.insert("marker-end", "none");
    pres.insert("color-interpolation", "sRGB");
    pres.insert("color-interpolation-filters", "linearRGB");
    pres.insert("color-rendering", "auto");
    pres.insert("shape-rendering", "auto");
    pres.insert("text-rendering", "auto");
    pres.insert("image-rendering", "auto");
    pres.insert("font-style", "normal");
    pres.insert("font-variant", "normal");
    pres.insert("font-weight", "normal");
    pres.insert("font-stretch", "normal");
    pres.insert("font-size", "medium");
    pres.insert("font-size-adjust", "none");
    pres.insert("kerning", "auto");
    pres.insert("letter-spacing", "normal");
    pres.insert("word-spacing", "normal");
    pres.insert("text-decoration", "none");
    pres.insert("text-anchor", "start");
    pres.insert("text-overflow", "clip");
    pres.insert("writing-mode", "lr-tb");
    pres.insert("glyph-orientation-vertical", "auto");
    pres.insert("glyph-orientation-horizontal", "0deg");
    pres.insert("direction", "ltr");
    pres.insert("unicode-bidi", "normal");
    pres.insert("dominant-baseline", "auto");
    pres.insert("alignment-baseline", "baseline");
    pres.insert("baseline-shift", "baseline");
    m.insert("presentation", pres);
    let mut tf = HashMap::new();
    tf.insert("slope", "1");
    tf.insert("intercept", "0");
    tf.insert("amplitude", "1");
    tf.insert("exponent", "1");
    tf.insert("offset", "0");
    m.insert("transferFunction", tf);
    m
});

/// Per-element allowed attributes and defaults.
/// Maps element name -> (allowed_attrs, default_values)
#[allow(clippy::type_complexity)]
pub static ELEMS: LazyLock<
    HashMap<&'static str, (HashSet<&'static str>, HashMap<&'static str, &'static str>)>,
> = LazyLock::new(|| {
    use std::collections::{HashMap, HashSet};

    let mut m: HashMap<&str, (HashSet<&str>, HashMap<&str, &str>)> = HashMap::new();

    macro_rules! elem {
        ($name:expr, [$($attr:expr),* $(,)?]) => {
            m.insert($name, (HashSet::from([$($attr),*]), HashMap::new()));
        };
    }

    // SVG root element
    elem!(
        "svg",
        [
            "xmlns",
            "xmlns:xlink",
            "xml:space",
            "version",
            "baseProfile",
            "preserveAspectRatio",
            "contentScriptType",
            "contentStyleType",
            "width",
            "height",
            "viewBox",
            "x",
            "y",
            "id",
            "class",
            "style",
            "filter",
            "mask",
            "clip-path",
            "fill",
            "stroke",
            "opacity",
            "fill-opacity",
            "stroke-opacity",
            "fill-rule",
            "clip-rule",
            "display",
            "overflow",
            "visibility",
            "color-interpolation",
            "color-rendering",
            "shape-rendering",
            "text-rendering",
            "image-rendering",
            "font-family",
            "font-size",
            "font-style",
            "font-weight",
            "text-anchor",
            "letter-spacing",
            "word-spacing",
            "enable-background",
            "xml:lang",
            "xml:base",
            "tabindex",
            "transform",
            "onactivate",
            "onclick",
            "onfocusin",
            "onfocusout",
            "onload",
            "onmousedown",
            "onmousemove",
            "onmouseout",
            "onmouseover",
            "onmouseup",
        ]
    );

    elem!(
        "g",
        [
            "id",
            "class",
            "style",
            "transform",
            "fill",
            "stroke",
            "opacity",
            "fill-opacity",
            "stroke-opacity",
            "fill-rule",
            "clip-rule",
            "display",
            "visibility",
            "filter",
            "mask",
            "clip-path",
            "color-interpolation",
            "color-rendering",
            "shape-rendering",
            "text-rendering",
            "image-rendering",
            "font-family",
            "font-size",
            "font-style",
            "font-weight",
            "text-anchor",
            "letter-spacing",
            "word-spacing",
            "cursor",
            "direction",
            "dominant-baseline",
            "alignment-baseline",
            "baseline-shift",
            "paint-order",
            "pointer-events",
            "unicode-bidi",
            "writing-mode",
            "marker-start",
            "marker-mid",
            "marker-end",
            "clip",
            "color-interpolation-filters",
            "color-profile",
            "font",
            "font-size-adjust",
            "font-stretch",
            "font-variant",
            "kerning",
            "text-decoration",
            "text-overflow",
            "overflow",
            "enable-background",
            "onactivate",
            "onclick",
            "onfocusin",
            "onfocusout",
            "onload",
            "onmousedown",
            "onmousemove",
            "onmouseout",
            "onmouseover",
            "onmouseup",
        ]
    );

    elem!("defs", ["id", "class", "style", "transform"]);
    elem!("desc", ["id", "class", "style"]);
    elem!("metadata", ["id"]);
    elem!("title", ["id", "class", "style"]);

    elem!(
        "path",
        [
            "id",
            "class",
            "style",
            "d",
            "pathLength",
            "fill",
            "stroke",
            "transform",
            "fill-opacity",
            "stroke-opacity",
            "fill-rule",
            "stroke-dasharray",
            "stroke-dashoffset",
            "stroke-linecap",
            "stroke-linejoin",
            "stroke-miterlimit",
            "stroke-width",
            "opacity",
            "clip-path",
            "filter",
            "mask",
            "display",
            "visibility",
        ]
    );

    {
        let mut defaults = HashMap::new();
        defaults.insert("x", "0");
        defaults.insert("y", "0");
        m.insert(
            "rect",
            (
                HashSet::from([
                    "id",
                    "class",
                    "style",
                    "x",
                    "y",
                    "width",
                    "height",
                    "rx",
                    "ry",
                    "transform",
                    "fill",
                    "stroke",
                ]),
                defaults,
            ),
        );
    }

    {
        let mut defaults = HashMap::new();
        defaults.insert("cx", "0");
        defaults.insert("cy", "0");
        m.insert(
            "circle",
            (
                HashSet::from([
                    "id",
                    "class",
                    "style",
                    "cx",
                    "cy",
                    "r",
                    "transform",
                    "fill",
                    "stroke",
                ]),
                defaults,
            ),
        );
    }

    {
        let mut defaults = HashMap::new();
        defaults.insert("cx", "0");
        defaults.insert("cy", "0");
        m.insert(
            "ellipse",
            (
                HashSet::from([
                    "id",
                    "class",
                    "style",
                    "cx",
                    "cy",
                    "rx",
                    "ry",
                    "transform",
                    "fill",
                    "stroke",
                ]),
                defaults,
            ),
        );
    }

    {
        let mut defaults = HashMap::new();
        defaults.insert("x1", "0");
        defaults.insert("y1", "0");
        defaults.insert("x2", "0");
        defaults.insert("y2", "0");
        m.insert(
            "line",
            (
                HashSet::from([
                    "id",
                    "class",
                    "style",
                    "x1",
                    "y1",
                    "x2",
                    "y2",
                    "transform",
                    "fill",
                    "stroke",
                ]),
                defaults,
            ),
        );
    }

    elem!(
        "polyline",
        [
            "id",
            "class",
            "style",
            "points",
            "transform",
            "fill",
            "stroke"
        ]
    );
    elem!(
        "polygon",
        [
            "id",
            "class",
            "style",
            "points",
            "transform",
            "fill",
            "stroke"
        ]
    );

    elem!(
        "text",
        [
            "id",
            "class",
            "style",
            "x",
            "y",
            "dx",
            "dy",
            "rotate",
            "textLength",
            "lengthAdjust",
            "transform",
            "fill",
            "stroke",
        ]
    );

    elem!(
        "tspan",
        [
            "id",
            "class",
            "style",
            "x",
            "y",
            "dx",
            "dy",
            "rotate",
            "textLength",
            "lengthAdjust",
        ]
    );

    elem!("tref", ["id", "class", "style", "xlink:href"]);

    elem!(
        "use",
        [
            "id",
            "class",
            "style",
            "x",
            "y",
            "width",
            "height",
            "href",
            "xlink:href",
            "transform",
        ]
    );

    {
        let mut defaults = HashMap::new();
        defaults.insert("x", "0");
        defaults.insert("y", "0");
        defaults.insert("preserveAspectRatio", "xMidYMid meet");
        m.insert(
            "image",
            (
                HashSet::from([
                    "id",
                    "class",
                    "style",
                    "x",
                    "y",
                    "width",
                    "height",
                    "preserveAspectRatio",
                    "href",
                    "xlink:href",
                    "transform",
                ]),
                defaults,
            ),
        );
    }

    {
        let mut defaults = HashMap::new();
        defaults.insert("patternUnits", "objectBoundingBox");
        defaults.insert("x", "0");
        defaults.insert("y", "0");
        defaults.insert("width", "0");
        defaults.insert("height", "0");
        m.insert(
            "pattern",
            (
                HashSet::from([
                    "id",
                    "class",
                    "style",
                    "x",
                    "y",
                    "width",
                    "height",
                    "patternUnits",
                    "patternContentUnits",
                    "patternTransform",
                    "viewBox",
                    "preserveAspectRatio",
                    "href",
                    "xlink:href",
                ]),
                defaults,
            ),
        );
    }

    m.insert(
        "clipPath",
        (
            HashSet::from(["id", "class", "style", "clipPathUnits", "transform"]),
            {
                let mut d = HashMap::new();
                d.insert("clipPathUnits", "userSpaceOnUse");
                d
            },
        ),
    );

    {
        let mut defaults = HashMap::new();
        defaults.insert("maskUnits", "objectBoundingBox");
        defaults.insert("maskContentUnits", "userSpaceOnUse");
        defaults.insert("x", "-10%");
        defaults.insert("y", "-10%");
        defaults.insert("width", "120%");
        defaults.insert("height", "120%");
        m.insert(
            "mask",
            (
                HashSet::from([
                    "id",
                    "class",
                    "style",
                    "x",
                    "y",
                    "width",
                    "height",
                    "maskUnits",
                    "maskContentUnits",
                    "mask-type",
                ]),
                defaults,
            ),
        );
    }

    {
        let mut defaults = HashMap::new();
        defaults.insert("primitiveUnits", "userSpaceOnUse");
        defaults.insert("x", "-10%");
        defaults.insert("y", "-10%");
        defaults.insert("width", "120%");
        defaults.insert("height", "120%");
        m.insert(
            "filter",
            (
                HashSet::from([
                    "id",
                    "class",
                    "style",
                    "x",
                    "y",
                    "width",
                    "height",
                    "filterUnits",
                    "primitiveUnits",
                    "filterRes",
                ]),
                defaults,
            ),
        );
    }

    {
        let mut defaults = HashMap::new();
        defaults.insert("x1", "0");
        defaults.insert("y1", "0");
        defaults.insert("x2", "100%");
        defaults.insert("y2", "0");
        defaults.insert("spreadMethod", "pad");
        m.insert(
            "linearGradient",
            (
                HashSet::from([
                    "id",
                    "class",
                    "style",
                    "x1",
                    "y1",
                    "x2",
                    "y2",
                    "gradientUnits",
                    "gradientTransform",
                    "spreadMethod",
                    "href",
                    "xlink:href",
                ]),
                defaults,
            ),
        );
    }

    {
        let mut defaults = HashMap::new();
        defaults.insert("cx", "50%");
        defaults.insert("cy", "50%");
        defaults.insert("r", "50%");
        defaults.insert("spreadMethod", "pad");
        m.insert(
            "radialGradient",
            (
                HashSet::from([
                    "id",
                    "class",
                    "style",
                    "cx",
                    "cy",
                    "r",
                    "fx",
                    "fy",
                    "gradientUnits",
                    "gradientTransform",
                    "spreadMethod",
                    "href",
                    "xlink:href",
                ]),
                defaults,
            ),
        );
    }

    elem!(
        "stop",
        [
            "id",
            "class",
            "style",
            "offset",
            "stop-color",
            "stop-opacity"
        ]
    );

    elem!(
        "symbol",
        [
            "id",
            "class",
            "style",
            "viewBox",
            "preserveAspectRatio",
            "x",
            "y",
            "width",
            "height",
        ]
    );

    {
        let mut defaults = HashMap::new();
        defaults.insert("markerUnits", "strokeWidth");
        defaults.insert("refX", "0");
        defaults.insert("refY", "0");
        defaults.insert("markerWidth", "3");
        defaults.insert("markerHeight", "3");
        m.insert(
            "marker",
            (
                HashSet::from([
                    "id",
                    "class",
                    "style",
                    "viewBox",
                    "preserveAspectRatio",
                    "refX",
                    "refY",
                    "markerWidth",
                    "markerHeight",
                    "markerUnits",
                    "orient",
                ]),
                defaults,
            ),
        );
    }

    elem!("switch", ["id", "class", "style", "transform"]);

    {
        let mut defaults = HashMap::new();
        defaults.insert("x", "0");
        defaults.insert("y", "0");
        m.insert(
            "foreignObject",
            (
                HashSet::from([
                    "id",
                    "class",
                    "style",
                    "x",
                    "y",
                    "width",
                    "height",
                    "transform",
                ]),
                defaults,
            ),
        );
    }

    elem!(
        "a",
        [
            "id",
            "class",
            "style",
            "target",
            "transform",
            "href",
            "xlink:href",
        ]
    );

    m
});

/// Presentation attributes that are NOT inheritable by child elements.
/// Used by removeNonInheritableGroupAttrs.
pub static PRESENTATION_NON_INHERITABLE_GROUP_ATTRS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| {
        // All presentation attrs minus inheritable ones
        let inheritable: HashSet<&&str> = INHERITABLE_ATTRS.iter().collect();
        ATTRS_GROUPS
            .get("presentation")
            .map(|pres| {
                pres.iter()
                    .filter(|a| !inheritable.contains(a))
                    .copied()
                    .collect::<HashSet<&str>>()
            })
            .unwrap_or_default()
    });

/// CSS properties whose values are colors and should be converted.
/// Elements that can have a `d` attribute (path data).
#[allow(non_upper_case_globals)]
pub static pathElems: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(["glyph", "missing-glyph", "path"]));

pub static COLORS_PROPS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "color",
        "fill",
        "flood-color",
        "lighting-color",
        "stop-color",
        "stroke",
    ])
});

/// Short color name lookup: hex -> shortest name.
pub static COLORS_SHORT_NAMES: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        HashMap::from([
            ("#f0ffff", "azure"),
            ("#f5f5dc", "beige"),
            ("#ffe4c4", "bisque"),
            ("#a52a2a", "brown"),
            ("#ff7f50", "coral"),
            ("#ffd700", "gold"),
            ("#808080", "gray"),
            ("#008000", "green"),
            ("#4b0082", "indigo"),
            ("#fffff0", "ivory"),
            ("#f0e68c", "khaki"),
            ("#faf0e6", "linen"),
            ("#800000", "maroon"),
            ("#000080", "navy"),
            ("#808000", "olive"),
            ("#ffa500", "orange"),
            ("#da70d6", "orchid"),
            ("#cd853f", "peru"),
            ("#ffc0cb", "pink"),
            ("#dda0dd", "plum"),
            ("#800080", "purple"),
            ("#f00", "red"),
            ("#ff0000", "red"),
            ("#fa8072", "salmon"),
            ("#a0522d", "sienna"),
            ("#c0c0c0", "silver"),
            ("#fffafa", "snow"),
            ("#d2b48c", "tan"),
            ("#008080", "teal"),
            ("#ff6347", "tomato"),
            ("#ee82ee", "violet"),
            ("#f5deb3", "wheat"),
        ])
    });

/// Allowed children per element (ported from elems.*.content + contentGroups).
#[allow(clippy::type_complexity)]
pub static ELEMS_CONTENT: LazyLock<HashMap<&'static str, HashSet<&'static str>>> =
    LazyLock::new(|| {
        let mut m: HashMap<&str, HashSet<&str>> = HashMap::new();

        macro_rules! add_content {
            ($name:expr, [$($child:expr),* $(,)?]) => {
                let entry = m.entry($name).or_default();
                $(entry.insert($child);)*
            };
        }

        // From elems.*.content arrays in _collections.js
        add_content!(
            "a",
            [
                "a",
                "altGlyphDef",
                "clipPath",
                "color-profile",
                "cursor",
                "filter",
                "font-face",
                "font",
                "foreignObject",
                "image",
                "marker",
                "mask",
                "pattern",
                "script",
                "style",
                "switch",
                "text",
                "view",
                "tspan"
            ]
        );
        add_content!("altGlyphDef", ["glyphRef"]);
        add_content!("altGlyphItem", ["glyphRef", "altGlyphItem"]);
        add_content!("animate", ["desc", "metadata", "title"]);
        add_content!("animateColor", ["desc", "metadata", "title"]);
        add_content!("animateMotion", ["desc", "metadata", "title", "mpath"]);
        add_content!("animateTransform", ["desc", "metadata", "title"]);
        add_content!(
            "circle",
            [
                "animate",
                "animateColor",
                "animateMotion",
                "animateTransform",
                "set",
                "desc",
                "metadata",
                "title"
            ]
        );
        add_content!(
            "clipPath",
            [
                "animate",
                "animateColor",
                "animateMotion",
                "animateTransform",
                "set",
                "desc",
                "metadata",
                "title",
                "text",
                "use"
            ]
        );
        add_content!("color-profile", ["desc", "metadata", "title"]);
        add_content!("cursor", ["desc", "metadata", "title"]);
        add_content!(
            "defs",
            [
                "a",
                "altGlyphDef",
                "clipPath",
                "color-profile",
                "cursor",
                "filter",
                "font-face",
                "font",
                "foreignObject",
                "image",
                "marker",
                "mask",
                "pattern",
                "script",
                "style",
                "switch",
                "text",
                "view"
            ]
        );
        add_content!("desc", []);
        add_content!(
            "ellipse",
            [
                "animate",
                "animateColor",
                "animateMotion",
                "animateTransform",
                "set",
                "desc",
                "metadata",
                "title"
            ]
        );
        add_content!("feBlend", ["animate", "set"]);
        add_content!("feColorMatrix", ["animate", "set"]);
        add_content!(
            "feComponentTransfer",
            ["feFuncA", "feFuncB", "feFuncG", "feFuncR"]
        );
        add_content!("feComposite", ["animate", "set"]);
        add_content!("feConvolveMatrix", ["animate", "set"]);
        add_content!(
            "feDiffuseLighting",
            [
                "desc",
                "metadata",
                "title",
                "feDistantLight",
                "fePointLight",
                "feSpotLight"
            ]
        );
        add_content!("feDisplacementMap", ["animate", "set"]);
        add_content!("feDistantLight", ["animate", "set"]);
        add_content!("feFlood", ["animate", "animateColor", "set"]);
        add_content!("feFuncA", ["set", "animate"]);
        add_content!("feFuncB", ["set", "animate"]);
        add_content!("feFuncG", ["set", "animate"]);
        add_content!("feFuncR", ["set", "animate"]);
        add_content!("feGaussianBlur", ["set", "animate"]);
        add_content!("feImage", ["animate", "animateTransform", "set"]);
        add_content!("feMerge", ["feMergeNode"]);
        add_content!("feMergeNode", ["animate", "set"]);
        add_content!("feMorphology", ["animate", "set"]);
        add_content!("feOffset", ["animate", "set"]);
        add_content!("fePointLight", ["animate", "set"]);
        add_content!(
            "feSpecularLighting",
            [
                "desc",
                "metadata",
                "title",
                "feDistantLight",
                "fePointLight",
                "feSpotLight"
            ]
        );
        add_content!("feSpotLight", ["animate", "set"]);
        add_content!("feTile", ["animate", "set"]);
        add_content!("feTurbulence", ["animate", "set"]);
        add_content!(
            "filter",
            [
                "animate",
                "animateColor",
                "animateMotion",
                "animateTransform",
                "set",
                "desc",
                "metadata",
                "title",
                "feBlend",
                "feColorMatrix",
                "feComponentTransfer",
                "feComposite",
                "feConvolveMatrix",
                "feDiffuseLighting",
                "feDisplacementMap",
                "feDropShadow",
                "feFlood",
                "feFuncA",
                "feFuncB",
                "feFuncG",
                "feFuncR",
                "feGaussianBlur",
                "feImage",
                "feMerge",
                "feMergeNode",
                "feMorphology",
                "feOffset",
                "feSpecularLighting",
                "feTile",
                "feTurbulence"
            ]
        );
        add_content!(
            "font",
            [
                "desc",
                "metadata",
                "title",
                "font-face",
                "glyph",
                "hkern",
                "missing-glyph",
                "vkern"
            ]
        );
        add_content!("font-face", ["desc", "metadata", "title", "font-face-src"]);
        add_content!("font-face-src", ["font-face-name", "font-face-uri"]);
        add_content!("font-face-uri", ["font-face-format"]);
        add_content!("foreignObject", ["desc", "metadata", "title"]); // accepts any content but we list descriptive
        add_content!(
            "g",
            [
                "a",
                "altGlyphDef",
                "clipPath",
                "color-profile",
                "cursor",
                "filter",
                "font-face",
                "font",
                "foreignObject",
                "image",
                "marker",
                "mask",
                "pattern",
                "script",
                "style",
                "switch",
                "text",
                "view"
            ]
        );
        add_content!(
            "glyph",
            [
                "a",
                "altGlyphDef",
                "clipPath",
                "color-profile",
                "cursor",
                "filter",
                "font-face",
                "font",
                "foreignObject",
                "image",
                "marker",
                "mask",
                "pattern",
                "script",
                "style",
                "switch",
                "text",
                "view"
            ]
        );
        add_content!(
            "glyphRef",
            [
                "a",
                "altGlyphDef",
                "clipPath",
                "color-profile",
                "cursor",
                "filter",
                "font-face",
                "font",
                "foreignObject",
                "image",
                "marker",
                "mask",
                "pattern",
                "script",
                "style",
                "switch",
                "text",
                "view"
            ]
        );
        add_content!(
            "hatch",
            [
                "animate",
                "animateColor",
                "animateMotion",
                "animateTransform",
                "set",
                "desc",
                "metadata",
                "title",
                "hatchPath"
            ]
        );
        add_content!(
            "hatchPath",
            [
                "animate",
                "animateColor",
                "animateMotion",
                "animateTransform",
                "set",
                "desc",
                "metadata",
                "title"
            ]
        );
        add_content!(
            "image",
            [
                "animate",
                "animateColor",
                "animateMotion",
                "animateTransform",
                "set",
                "desc",
                "metadata",
                "title"
            ]
        );
        add_content!(
            "line",
            [
                "animate",
                "animateColor",
                "animateMotion",
                "animateTransform",
                "set",
                "desc",
                "metadata",
                "title"
            ]
        );
        add_content!(
            "linearGradient",
            [
                "animate",
                "animateTransform",
                "set",
                "stop",
                "desc",
                "metadata",
                "title"
            ]
        );
        add_content!(
            "marker",
            [
                "a",
                "altGlyphDef",
                "clipPath",
                "color-profile",
                "cursor",
                "filter",
                "font-face",
                "font",
                "foreignObject",
                "image",
                "marker",
                "mask",
                "pattern",
                "script",
                "style",
                "switch",
                "text",
                "view"
            ]
        );
        add_content!(
            "mask",
            [
                "a",
                "altGlyphDef",
                "clipPath",
                "color-profile",
                "cursor",
                "filter",
                "font-face",
                "font",
                "foreignObject",
                "image",
                "marker",
                "mask",
                "pattern",
                "script",
                "style",
                "switch",
                "text",
                "view"
            ]
        );
        add_content!(
            "meshGradient",
            [
                "animate",
                "animateColor",
                "animateMotion",
                "animateTransform",
                "set",
                "desc",
                "metadata",
                "title",
                "meshRow",
                "linearGradient",
                "radialGradient",
                "pattern",
                "image",
                "svg",
                "text",
                "use",
                "rect",
                "circle",
                "ellipse",
                "line",
                "path",
                "polygon",
                "polyline"
            ]
        );
        add_content!("meshRow", ["desc", "metadata", "title", "meshPatch"]);
        add_content!("meshPatch", ["desc", "metadata", "title", "stop"]);
        add_content!(
            "missing-glyph",
            [
                "a",
                "altGlyphDef",
                "clipPath",
                "color-profile",
                "cursor",
                "filter",
                "font-face",
                "font",
                "foreignObject",
                "image",
                "marker",
                "mask",
                "pattern",
                "script",
                "style",
                "switch",
                "text",
                "view"
            ]
        );
        add_content!("mpath", ["desc", "metadata", "title"]);
        add_content!(
            "path",
            [
                "animate",
                "animateColor",
                "animateMotion",
                "animateTransform",
                "set",
                "desc",
                "metadata",
                "title"
            ]
        );
        add_content!(
            "pattern",
            [
                "a",
                "altGlyphDef",
                "clipPath",
                "color-profile",
                "cursor",
                "filter",
                "font-face",
                "font",
                "foreignObject",
                "image",
                "marker",
                "mask",
                "pattern",
                "script",
                "style",
                "switch",
                "text",
                "view"
            ]
        );
        add_content!(
            "polygon",
            [
                "animate",
                "animateColor",
                "animateMotion",
                "animateTransform",
                "set",
                "desc",
                "metadata",
                "title"
            ]
        );
        add_content!(
            "polyline",
            [
                "animate",
                "animateColor",
                "animateMotion",
                "animateTransform",
                "set",
                "desc",
                "metadata",
                "title"
            ]
        );
        add_content!(
            "radialGradient",
            [
                "animate",
                "animateTransform",
                "set",
                "stop",
                "desc",
                "metadata",
                "title"
            ]
        );
        add_content!(
            "rect",
            [
                "animate",
                "animateColor",
                "animateMotion",
                "animateTransform",
                "set",
                "desc",
                "metadata",
                "title"
            ]
        );
        add_content!("script", []);
        add_content!("set", ["desc", "metadata", "title"]);
        add_content!(
            "solidColor",
            [
                "linearGradient",
                "radialGradient",
                "pattern",
                "hatch",
                "solidColor"
            ]
        );
        add_content!("stop", ["animate", "animateColor", "set"]);
        add_content!(
            "svg",
            [
                "a",
                "altGlyphDef",
                "clipPath",
                "color-profile",
                "cursor",
                "filter",
                "font-face",
                "font",
                "foreignObject",
                "image",
                "marker",
                "mask",
                "pattern",
                "script",
                "style",
                "switch",
                "text",
                "view"
            ]
        );
        add_content!(
            "switch",
            [
                "animate",
                "animateColor",
                "animateMotion",
                "animateTransform",
                "set",
                "desc",
                "metadata",
                "title",
                "a",
                "foreignObject",
                "g",
                "image",
                "svg",
                "switch",
                "text",
                "use",
                "circle",
                "ellipse",
                "line",
                "path",
                "polygon",
                "polyline",
                "rect"
            ]
        );
        add_content!(
            "symbol",
            [
                "a",
                "altGlyphDef",
                "clipPath",
                "color-profile",
                "cursor",
                "filter",
                "font-face",
                "font",
                "foreignObject",
                "image",
                "marker",
                "mask",
                "pattern",
                "script",
                "style",
                "switch",
                "text",
                "view"
            ]
        );
        add_content!(
            "text",
            [
                "animate",
                "animateColor",
                "animateMotion",
                "animateTransform",
                "set",
                "desc",
                "metadata",
                "title",
                "a",
                "altGlyph",
                "textPath",
                "tref",
                "tspan"
            ]
        );
        add_content!(
            "textPath",
            [
                "a",
                "altGlyph",
                "animate",
                "animateColor",
                "set",
                "tref",
                "tspan",
                "desc",
                "metadata",
                "title"
            ]
        );
        add_content!("title", []);
        add_content!(
            "tref",
            [
                "animate",
                "animateColor",
                "set",
                "desc",
                "metadata",
                "title"
            ]
        );
        add_content!(
            "tspan",
            [
                "a",
                "altGlyph",
                "animate",
                "animateColor",
                "set",
                "tref",
                "tspan",
                "desc",
                "metadata",
                "title"
            ]
        );
        add_content!(
            "use",
            [
                "animate",
                "animateColor",
                "animateMotion",
                "animateTransform",
                "set",
                "desc",
                "metadata",
                "title"
            ]
        );
        add_content!("view", ["desc", "metadata", "title"]);

        m
    });
