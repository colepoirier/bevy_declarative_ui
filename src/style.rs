type Class = (&'static str, Vec<Rule>);

#[derive(Debug, Clone)]
enum Rule {
    Prop(&'static str, &'static str),
    Child(&'static str, Vec<Rule>),
    AllChildren(String, Vec<Rule>),
    Supports(
        (&'static str, &'static str),
        Vec<(&'static str, &'static str)>,
    ),
    Descriptor(&'static str, Vec<Rule>),
    Adjacent(&'static str, Vec<Rule>),
    Batch(Vec<Rule>),
}

enum StyleClasses {
    Root,
    Any,
    Single,
    Row,
    Column,
    Paragraph,
    Page,
    Text,
    Grid,
    Spacer,
}

#[derive(Debug, Copy, Clone)]
enum Alignment {
    Top,
    Bottom,
    Right,
    Left,
    CenterX,
    CenterY,
}

enum Location {
    Above,
    Below,
    OnRight,
    OnLeft,
    Within,
    Behind,
}

#[derive(Debug, Copy, Clone)]
struct SelfDescriptor(Alignment);

#[derive(Debug, Copy, Clone)]
struct ContentDescriptor(Alignment);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Classes {
    Root,
    Any,
    Single,
    Row,
    Column,
    Page,
    Paragraph,
    Text,
    Grid,
    ImageContainer,
    Wrapped,

    // widths/heights
    WidthFill,
    WidthContent,
    WidthExact,
    WidthFillPortion,
    HeightFill,
    HeightContent,
    HeightExact,
    HeightFillPortion,
    SEButton,

    // nearby elements
    Nearby,
    Above,
    Below,
    OnRight,
    OnLeft,
    InFront,
    Behind,
    HasBehind,

    // alignments
    AlignTop,
    AlignBottom,
    AlignRight,
    AlignLeft,
    AlignCenterX,
    AlignCenterY,
    AlignedHorizontally,
    AlignedVertically,

    // space evenly
    SpaceEvenly,
    Container,
    AlignContainerRight,
    AlignContainerBottom,
    AlignContainerCenterX,
    AlignContainerCenterY,

    // content alignments
    ContentTop,
    ContentBottom,
    ContentRight,
    ContentLeft,
    ContentCenterX,
    ContentCenterY,

    // selection
    NoTextSelection,
    CursorPointer,
    CursorText,

    // pointer events
    PassPointerEvents,
    CapturePointerEvents,
    Transparent,
    Opaque,
    OverflowHidden,

    // special state classes
    Hover,
    Focus,
    FocusedWithin,
    Active,

    // scrollbars
    Scrollbars,
    ScrollbarsX,
    ScrollbarsY,
    Clip,
    ClipX,
    ClipY,

    // borders
    BorderNone,
    BorderDashed,
    BorderDotted,
    BorderSolid,

    // text weight
    SizeByCapital,
    FullSize,
    TextThin,
    TextExtraLight,
    TextLight,
    TextNormalWeight,
    TextMedium,
    TextSemiBold,
    Bold,
    TextExtraBold,
    TextHeavy,
    Italic,
    Strike,
    Underline,
    TextUnitalicized,

    // text alignment
    TextJustify,
    TextJustifyAll,
    TextCenter,
    TextRight,
    TextLeft,
    Transition,

    // input text
    InputText,
    InputMultiline,
    InputMultilineParent,
    InputMultilineFiller,
    InputMultilineWrapper,
    InputLabel,

    // link
    Link,
}

impl Classes {
    pub const fn to_string(&self) -> &'static str {
        match self {
            Self::Root => "ui",
            Self::Any => "s",
            Self::Single => "e",
            Self::Row => "r",
            Self::Column => "c",
            Self::Page => "pg",
            Self::Paragraph => "p",
            Self::Text => "t",
            Self::Grid => "g",
            Self::ImageContainer => "ic",
            Self::Wrapped => "wrp",

            // widths/heights
            Self::WidthFill => "wf",
            Self::WidthContent => "wc",
            Self::WidthExact => "we",
            Self::WidthFillPortion => "wfp",
            Self::HeightFill => "hf",
            Self::HeightContent => "hc",
            Self::HeightExact => "he",
            Self::HeightFillPortion => "hfp",
            Self::SEButton => "sbt",

            // nearby elements
            Self::Nearby => "nb",
            Self::Above => "a",
            Self::Below => "b",
            Self::OnRight => "or",
            Self::OnLeft => "ol",
            Self::InFront => "fr",
            Self::Behind => "bh",
            Self::HasBehind => "hbh",

            // alignments
            Self::AlignTop => "at",
            Self::AlignBottom => "ab",
            Self::AlignRight => "ar",
            Self::AlignLeft => "al",
            Self::AlignCenterX => "cx",
            Self::AlignCenterY => "cy",
            Self::AlignedHorizontally => "ah",
            Self::AlignedVertically => "av",

            // space evenly
            Self::SpaceEvenly => "sev",
            Self::Container => "ctr",
            Self::AlignContainerRight => "acr",
            Self::AlignContainerBottom => "acb",
            Self::AlignContainerCenterX => "accx",
            Self::AlignContainerCenterY => "accy",

            // content alignments
            Self::ContentTop => "ct",
            Self::ContentBottom => "cb",
            Self::ContentRight => "cr",
            Self::ContentLeft => "cl",
            Self::ContentCenterX => "ccx",
            Self::ContentCenterY => "ccy",

            // selection
            Self::NoTextSelection => "notxt",
            Self::CursorPointer => "cptr",
            Self::CursorText => "ctxt",

            // pointer events
            Self::PassPointerEvents => "ppe",
            Self::CapturePointerEvents => "cpe",
            Self::Transparent => "clr",
            Self::Opaque => "oq",
            Self::OverflowHidden => "oh",

            // special state classes
            Self::Hover => "hv",
            Self::Focus => "fcs",
            Self::FocusedWithin => "focus-within",
            Self::Active => "atv",

            // scrollbars
            Self::Scrollbars => "sb",
            Self::ScrollbarsX => "sbx",
            Self::ScrollbarsY => "sby",
            Self::Clip => "cp",
            Self::ClipX => "cpx",
            Self::ClipY => "cpy",

            // borders
            Self::BorderNone => "bn",
            Self::BorderDashed => "bd",
            Self::BorderDotted => "bdt",
            Self::BorderSolid => "bs",

            // text weight
            Self::SizeByCapital => "cap",
            Self::FullSize => "fs",
            Self::TextThin => "w1",
            Self::TextExtraLight => "w2",
            Self::TextLight => "w3",
            Self::TextNormalWeight => "w4",
            Self::TextMedium => "w5",
            Self::TextSemiBold => "w6",
            Self::Bold => "w7",
            Self::TextExtraBold => "w8",
            Self::TextHeavy => "w9",
            Self::Italic => "i",
            Self::Strike => "sk",
            Self::Underline => "u",
            Self::TextUnitalicized => "tun",

            // text alignment
            Self::TextJustify => "tj",
            Self::TextJustifyAll => "tja",
            Self::TextCenter => "tc",
            Self::TextRight => "tr",
            Self::TextLeft => "tl",
            Self::Transition => "ts",

            // input text
            Self::InputText => "it",
            Self::InputMultiline => "iml",
            Self::InputMultilineParent => "imlp",
            Self::InputMultilineFiller => "imlf",
            Self::InputMultilineWrapper => "implw",
            Self::InputLabel => "lbl",

            // link
            Self::Link => "lnk",
        }
    }
}

impl ContentDescriptor {
    fn to_string(&self) -> &'static str {
        match self.0 {
            Alignment::Top => Classes::ContentTop.to_string(),
            Alignment::Bottom => Classes::ContentBottom.to_string(),
            Alignment::Right => Classes::ContentRight.to_string(),
            Alignment::Left => Classes::ContentLeft.to_string(),
            Alignment::CenterX => Classes::ContentCenterX.to_string(),
            Alignment::CenterY => Classes::ContentCenterY.to_string(),
        }
    }
}

impl SelfDescriptor {
    fn to_string(&self) -> &'static str {
        match self.0 {
            Alignment::Top => Classes::AlignTop.to_string(),
            Alignment::Bottom => Classes::AlignBottom.to_string(),
            Alignment::Right => Classes::AlignRight.to_string(),
            Alignment::Left => Classes::AlignLeft.to_string(),
            Alignment::CenterX => Classes::AlignCenterX.to_string(),
            Alignment::CenterY => Classes::AlignCenterY.to_string(),
        }
    }
}

fn describe_alignment(
    values: Box<dyn Fn(&Alignment) -> (Vec<Rule>, Vec<Rule>)>,
) -> Rule {
    let create_description = |alignment: &Alignment| {
        let (content, indiv) = values(alignment);
        let rules = vec![
            Rule::Descriptor(
                ContentDescriptor(*alignment).to_string(),
                content,
            ),
            Rule::Child(
                Classes::Any.to_string(),
                vec![Rule::Descriptor(
                    SelfDescriptor(*alignment).to_string(),
                    indiv,
                )],
            ),
        ];
        println!("{:?}", &rules);
        rules
    };
    Rule::Batch(
        vec![
            Alignment::Top,
            Alignment::Bottom,
            Alignment::Right,
            Alignment::Left,
            Alignment::CenterX,
            Alignment::CenterY,
        ]
        .iter()
        .flat_map(create_description)
        .collect(),
    )
}

fn grid_alignments(values: Box<dyn Fn(&Alignment) -> Vec<Rule>>) -> Rule {
    let create_description = |alignment: &Alignment| {
        let rules = vec![Rule::Child(
            Classes::Any.to_string(),
            vec![Rule::Descriptor(
                SelfDescriptor(*alignment).to_string(),
                values(alignment),
            )],
        )];
        println!("{:?}", &rules);
        rules
    };
    Rule::Batch(
        vec![
            Alignment::Top,
            Alignment::Bottom,
            Alignment::Right,
            Alignment::Left,
            Alignment::CenterX,
            Alignment::CenterY,
        ]
        .iter()
        .flat_map(create_description)
        .collect(),
    )
}

#[derive(Debug, Default, Clone)]
struct Intermediate {
    selector: String,
    props: Vec<(&'static str, &'static str)>,
    closing: &'static str,
    others: Vec<Intermediate>,
}

impl Intermediate {
    fn new(selector: String, closing: &'static str) -> Self {
        Self {
            selector: selector,
            props: vec![],
            closing: closing,
            others: vec![],
        }
    }
}

fn render_rules(
    parent: Intermediate,
    rules_to_render: Vec<Rule>,
) -> Intermediate {
    let generate_intermediates = |mut rendered: Intermediate, rule: &Rule| {
        let rule = rule.to_owned();
        match rule {
            Rule::Prop(name, val) => {
                rendered.props.push((name, val));
                rendered
            }
            Rule::Supports((prop, value), props) => {
                rendered.others.push(Intermediate {
                    selector: format!(
                        "@supports ({}:{}) {{{}",
                        prop, value, parent.selector
                    ),
                    props: props,
                    closing: "\n}}",
                    others: vec![],
                });
                rendered
            }
            Rule::Adjacent(selector, rules) => {
                rendered.others.push(render_rules(
                    Intermediate::new(
                        format!("{} + {}", parent.selector, selector),
                        "",
                    ),
                    rules,
                ));
                rendered
            }
            Rule::Child(child, rules) => {
                rendered.others.push(render_rules(
                    Intermediate::new(
                        format!("{} > {}", parent.selector, child),
                        "",
                    ),
                    rules,
                ));
                rendered
            }
            Rule::AllChildren(child, rules) => {
                rendered.others.push(render_rules(
                    Intermediate::new(
                        format!("{} {}", parent.selector, child),
                        "",
                    ),
                    rules,
                ));
                rendered
            }
            Rule::Descriptor(descriptor, rules) => {
                rendered.others.push(render_rules(
                    Intermediate::new(
                        format!("{}{}", parent.selector, descriptor),
                        "",
                    ),
                    rules,
                ));
                rendered
            }
            Rule::Batch(batched) => {
                rendered.others.push(render_rules(
                    Intermediate::new(parent.selector.clone(), ""),
                    batched,
                ));
                rendered
            }
        }
    };
    rules_to_render
        .iter()
        .rev()
        .fold(parent.clone(), generate_intermediates)
}

fn r_values(vals: &Vec<(&'static str, &'static str)>) -> String {
    vals.iter()
        .map(|(x, y)| format!("  {}: {};", x, y))
        .collect::<Vec<String>>()
        .join("\n")
}

fn r_class(rule: &Intermediate) -> String {
    if !rule.props.is_empty() {
        format!(
            "{} {{\n{}{}\n}}",
            rule.selector,
            r_values(&rule.props),
            rule.closing
        )
    } else {
        String::from("")
    }
}

fn r_intermediate(rule: &Intermediate) -> String {
    let mut rendered = r_class(rule);
    &mut rendered.push_str(
        &rule
            .others
            .iter()
            .map(|i| r_intermediate(i))
            .collect::<Vec<String>>()
            .join("\n"),
    );
    rendered
}

fn render(classes: Vec<Class>) -> String {
    classes
        .into_iter()
        .rev()
        .fold(vec![], |mut existing: Vec<Intermediate>, (name, rules)| {
            &mut existing.push(render_rules(
                Intermediate::new(name.to_string(), ""),
                rules,
            ));
            existing
        })
        .iter()
        .map(r_intermediate)
        .collect::<Vec<String>>()
        .join("\n")
}

fn rc_values(vals: &Vec<(&'static str, &'static str)>) -> String {
    vals.iter()
        .map(|(x, y)| format!("{}:{};", x, y))
        .collect::<Vec<String>>()
        .join("")
}

fn rc_class(rule: &Intermediate) -> String {
    if !rule.props.is_empty() {
        format!(
            "{} {{{}{}}}",
            rule.selector,
            rc_values(&rule.props),
            rule.closing
        )
    } else {
        String::from("")
    }
}

fn rc_intermediate(rule: &Intermediate) -> String {
    let mut rendered = rc_class(rule);
    &mut rendered.push_str(
        &rule
            .others
            .iter()
            .map(|i| rc_intermediate(i))
            .collect::<Vec<String>>()
            .join("\n"),
    );
    rendered
}

fn render_compact(classes: Vec<Class>) -> String {
    classes
        .into_iter()
        .rev()
        .fold(vec![], |mut existing: Vec<Intermediate>, (name, rules)| {
            &mut existing.push(render_rules(
                Intermediate::new(name.to_string(), ""),
                rules,
            ));
            existing
        })
        .iter()
        .map(rc_intermediate)
        .collect::<Vec<String>>()
        .join("")
}

fn viewport_rules() -> String {
    format!("
    html, body {{
        height: 100%;
        width: 100%;
    }}
    
    {}
    ", rules())
}

// fn describe_text(class: String, properties: Vec<Rule>) -> Rule {
//     properties.extend(
//         vec![
//             Rule::Child(".text", properties),
//             Rule::Child(".el", properties),
//             Rule::Child(".el > .text", properties),
//         ]
//     );
//     Rule::Descriptor(&class[..],
//         properties.iter().map(make_important)
//     )
// }

// fn make_important(rule: Rule) -> Rule {
//     match rule {
//         Rule::Prop(name, prop) => {
//             &mut prop.push_str(" !important");
//             Rule::Prop(name, prop)
//         },
//         _ => rule,
//     }
// }

const OVERRIDES: &'static str = "
@media screen and (-ms-high-contrast: active), (-ms-high-contrast: none) {
    s.r > .s { flex-basis: auto !important; }
    .s.r > .s.ctr { flex-basis: auto !important; }
}
input[type=\"search\"],
input[type=\"search\"]::-webkit-search-decoration,
input[type=\"search\"]::-webkit-search-cancel-button,
input[type=\"search\"]::-webkit-search-results-button,
input[type=\"search\"]::-webkit-search-results-decoration {
  -webkit-appearance:none;
}
input[type=range] {
  -webkit-appearance: none; 
  background: transparent;
  position:absolute;
  left:0;
  top:0;
  z-index:10;
  width: 100%;
  outline: dashed 1px;
  height: 100%;
  opacity: 0;
}
input[type=range]::-moz-range-track {
    background: transparent;
    cursor: pointer;
}
input[type=range]::-ms-track {
    background: transparent;
    cursor: pointer;
}
input[type=range]::-webkit-slider-runnable-track {
    background: transparent;
    cursor: pointer;
}
input[type=range]::-webkit-slider-thumb {
    -webkit-appearance: none;
    opacity: 0.5;
    width: 80px;
    height: 80px;
    background-color: black;
    border:none;
    border-radius: 5px;
}
input[type=range]::-moz-range-thumb {
    opacity: 0.5;
    width: 80px;
    height: 80px;
    background-color: black;
    border:none;
    border-radius: 5px;
}
input[type=range]::-ms-thumb {
    opacity: 0.5;
    width: 80px;
    height: 80px;
    background-color: black;
    border:none;
    border-radius: 5px;
}
input[type=range][orient=vertical]{
    writing-mode: bt-lr; /* IE */
    -webkit-appearance: slider-vertical;  /* WebKit */
}
.explain {
    border: 6px solid rgb(174, 121, 15) !important;
}
.explain > .s {
    border: 4px dashed rgb(0, 151, 167) !important;
}
.ctr {
    border: none !important;
}
.explain > .ctr > .s {
    border: 4px dashed rgb(0, 151, 167) !important;
}

";

const INPUT_TEXT_RESET: &'static str = "
input[type=\"search\"],
input[type=\"search\"]::-webkit-search-decoration,
input[type=\"search\"]::-webkit-search-cancel-button,
input[type=\"search\"]::-webkit-search-results-button,
input[type=\"search\"]::-webkit-search-results-decoration {
  -webkit-appearance:none;
}
";

const SLIDER_RESET: &'static str = "
input[type=range] {
  -webkit-appearance: none; 
  background: transparent;
  position:absolute;
  left:0;
  top:0;
  z-index:10;
  width: 100%;
  outline: dashed 1px;
  height: 100%;
  opacity: 0;
}
";

const TRACK_RESET: &'static str = "
input[type=range]::-moz-range-track {
    background: transparent;
    cursor: pointer;
}
input[type=range]::-ms-track {
    background: transparent;
    cursor: pointer;
}
input[type=range]::-webkit-slider-runnable-track {
    background: transparent;
    cursor: pointer;
}
";

const THUMB_RESET: &'static str = "
input[type=range]::-webkit-slider-thumb {
    -webkit-appearance: none;
    opacity: 0.5;
    width: 80px;
    height: 80px;
    background-color: black;
    border:none;
    border-radius: 5px;
}
input[type=range]::-moz-range-thumb {
    opacity: 0.5;
    width: 80px;
    height: 80px;
    background-color: black;
    border:none;
    border-radius: 5px;
}
input[type=range]::-ms-thumb {
    opacity: 0.5;
    width: 80px;
    height: 80px;
    background-color: black;
    border:none;
    border-radius: 5px;
}
input[type=range][orient=vertical]{
    writing-mode: bt-lr; /* IE */
    -webkit-appearance: slider-vertical;  /* WebKit */
}
";

const EXPLAINER: &'static str = "
.explain {
    border: 6px solid rgb(174, 121, 15) !important;
}
.explain > .s {
    border: 4px dashed rgb(0, 151, 167) !important;
}

.ctr {
    border: none !important;
}
.explain > .ctr > .s {
    border: 4px dashed rgb(0, 151, 167) !important;
}

";

fn common_values() -> Vec<Class> {
    vec![
        (".border-0", vec![Rule::Prop("border-width", "0px")]),
        (".border-1", vec![Rule::Prop("border-width", "1px")]),
        (".border-2", vec![Rule::Prop("border-width", "2px")]),
        (".border-3", vec![Rule::Prop("border-width", "3px")]),
        (".border-4", vec![Rule::Prop("border-width", "4px")]),
        (".border-5", vec![Rule::Prop("border-width", "5px")]),
        (".border-6", vec![Rule::Prop("border-width", "6px")]),
        (".font-size-8", vec![Rule::Prop("font-size", "8px")]),
        (".font-size-9", vec![Rule::Prop("font-size", "9px")]),
        (".font-size-10", vec![Rule::Prop("font-size", "10px")]),
        (".font-size-11", vec![Rule::Prop("font-size", "11px")]),
        (".font-size-12", vec![Rule::Prop("font-size", "12px")]),
        (".font-size-13", vec![Rule::Prop("font-size", "13px")]),
        (".font-size-14", vec![Rule::Prop("font-size", "14px")]),
        (".font-size-15", vec![Rule::Prop("font-size", "15px")]),
        (".font-size-16", vec![Rule::Prop("font-size", "16px")]),
        (".font-size-17", vec![Rule::Prop("font-size", "17px")]),
        (".font-size-18", vec![Rule::Prop("font-size", "18px")]),
        (".font-size-19", vec![Rule::Prop("font-size", "19px")]),
        (".font-size-20", vec![Rule::Prop("font-size", "20px")]),
        (".font-size-21", vec![Rule::Prop("font-size", "21px")]),
        (".font-size-22", vec![Rule::Prop("font-size", "22px")]),
        (".font-size-23", vec![Rule::Prop("font-size", "23px")]),
        (".font-size-24", vec![Rule::Prop("font-size", "24px")]),
        (".font-size-25", vec![Rule::Prop("font-size", "25px")]),
        (".font-size-26", vec![Rule::Prop("font-size", "26px")]),
        (".font-size-27", vec![Rule::Prop("font-size", "27px")]),
        (".font-size-28", vec![Rule::Prop("font-size", "28px")]),
        (".font-size-29", vec![Rule::Prop("font-size", "29px")]),
        (".font-size-30", vec![Rule::Prop("font-size", "30px")]),
        (".font-size-31", vec![Rule::Prop("font-size", "31px")]),
        (".font-size-32", vec![Rule::Prop("font-size", "32px")]),
        (".p-0", vec![Rule::Prop("padding", "0px")]),
        (".p-1", vec![Rule::Prop("padding", "1px")]),
        (".p-2", vec![Rule::Prop("padding", "2px")]),
        (".p-3", vec![Rule::Prop("padding", "3px")]),
        (".p-4", vec![Rule::Prop("padding", "4px")]),
        (".p-5", vec![Rule::Prop("padding", "5px")]),
        (".p-6", vec![Rule::Prop("padding", "6px")]),
        (".p-7", vec![Rule::Prop("padding", "7px")]),
        (".p-8", vec![Rule::Prop("padding", "8px")]),
        (".p-9", vec![Rule::Prop("padding", "9px")]),
        (".p-10", vec![Rule::Prop("padding", "10px")]),
        (".p-11", vec![Rule::Prop("padding", "11px")]),
        (".p-12", vec![Rule::Prop("padding", "12px")]),
        (".p-13", vec![Rule::Prop("padding", "13px")]),
        (".p-14", vec![Rule::Prop("padding", "14px")]),
        (".p-15", vec![Rule::Prop("padding", "15px")]),
        (".p-16", vec![Rule::Prop("padding", "16px")]),
        (".p-17", vec![Rule::Prop("padding", "17px")]),
        (".p-18", vec![Rule::Prop("padding", "18px")]),
        (".p-19", vec![Rule::Prop("padding", "19px")]),
        (".p-20", vec![Rule::Prop("padding", "20px")]),
        (".p-21", vec![Rule::Prop("padding", "21px")]),
        (".p-22", vec![Rule::Prop("padding", "22px")]),
        (".p-23", vec![Rule::Prop("padding", "23px")]),
        (".p-24", vec![Rule::Prop("padding", "24px")]),
        (".v-smcp", vec![Rule::Prop("font-variant", "small-caps")]),
        (".v-smcp-off", vec![Rule::Prop("font-variant", "normal")]),
        (
            ".v-zero",
            vec![Rule::Prop("font-feature-settings", "\"zero\"")],
        ),
        (
            ".v-zero-off",
            vec![Rule::Prop("font-feature-settings", "\"zero\" 0")],
        ),
        (
            ".v-onum",
            vec![Rule::Prop("font-feature-settings", "\"onum\"")],
        ),
        (
            ".v-onum-off",
            vec![Rule::Prop("font-feature-settings", "\"onum\" 0")],
        ),
        (
            ".v-liga",
            vec![Rule::Prop("font-feature-settings", "\"liga\"")],
        ),
        (
            ".v-liga-off",
            vec![Rule::Prop("font-feature-settings", "\"liga\" 0")],
        ),
        (
            ".v-dlig",
            vec![Rule::Prop("font-feature-settings", "\"dlig\"")],
        ),
        (
            ".v-dlig-off",
            vec![Rule::Prop("font-feature-settings", "\"dlig\" 0")],
        ),
        (
            ".v-ordn",
            vec![Rule::Prop("font-feature-settings", "\"ordn\"")],
        ),
        (
            ".v-ordn-off",
            vec![Rule::Prop("font-feature-settings", "\"ordn\" 0")],
        ),
        (
            ".v-tnum",
            vec![Rule::Prop("font-feature-settings", "\"tnum\"")],
        ),
        (
            ".v-tnum-off",
            vec![Rule::Prop("font-feature-settings", "\"tnum\" 0")],
        ),
        (
            ".v-afrc",
            vec![Rule::Prop("font-feature-settings", "\"afrc\"")],
        ),
        (
            ".v-afrc-off",
            vec![Rule::Prop("font-feature-settings", "\"afrc\" 0")],
        ),
        (
            ".v-frac",
            vec![Rule::Prop("font-feature-settings", "\"frac\"")],
        ),
        (
            ".v-frac-off",
            vec![Rule::Prop("font-feature-settings", "\"frac\" 0")],
        ),
    ]
}

#[test]
fn test() {
    let f = |alignment: &Alignment| match alignment {
        Alignment::Top => (
            vec![Rule::Prop("justify-content", "flex-start")],
            vec![Rule::Prop("margin-bottom", "auto")],
        ),

        Alignment::Bottom => (
            vec![Rule::Prop("justify-content", "flex-end")],
            vec![Rule::Prop("margin-top", "auto")],
        ),

        Alignment::Right => (
            vec![Rule::Prop("align-items", "flex-end")],
            vec![Rule::Prop("align-self", "flex-end")],
        ),

        Alignment::Left => (
            vec![Rule::Prop("align-items", "flex-start")],
            vec![Rule::Prop("align-self", "flex-start")],
        ),

        Alignment::CenterX => (
            vec![Rule::Prop("align-items", "center")],
            vec![Rule::Prop("align-self", "center")],
        ),

        Alignment::CenterY => {
            (vec![Rule::Prop("justify-content", "center")], vec![])
        }
    };
    describe_alignment(Box::new(f));
}

pub fn rules() -> String {
    let mut sheet = basesheet();
    &mut sheet.extend(common_values());
    format!("{}{}",
        OVERRIDES,
        render_compact(sheet),
    )
}

fn basesheet() -> Vec<Class> {
    vec![
        (
            "html,body",
            vec![
                Rule::Prop("height", "100%"),
                Rule::Prop("padding", "0"),
                Rule::Prop("margin", "0"),
            ],
        ),
        (
            ".s.e.ic",
            vec![
                Rule::Prop("display", "block"),
                Rule::Descriptor(
                    ".hf",
                    vec![Rule::Child(
                        "img",
                        vec![
                            Rule::Prop("max-height", "100%"),
                            Rule::Prop("object-fit", "cover"),
                        ],
                    )],
                ),
                Rule::Descriptor(
                    ".wf",
                    vec![Rule::Child(
                        "img",
                        vec![
                            Rule::Prop("max-width", "100%"),
                            Rule::Prop("object-fit", "cover"),
                        ],
                    )],
                ),
            ],
        ),
        (".s:focus", vec![Rule::Prop("outline", "none")]),
        (
            ".ui",
            vec![
                Rule::Prop("width", "100%"),
                Rule::Prop("height", "auto"),
                Rule::Prop("min-height", "100%"),
                Rule::Prop("z-index", "0"),
                Rule::Descriptor(
                    ".s.e.hf",
                    vec![
                        Rule::Prop("height", "100%"),
                        Rule::Child(".hf", vec![Rule::Prop("height", "100%")]),
                    ],
                ),
                Rule::Child(
                    ".fr",
                    vec![Rule::Descriptor(
                        ".nb",
                        vec![
                            Rule::Prop("position", "fixed"),
                            Rule::Prop("z-index", "20"),
                        ],
                    )],
                ),
            ],
        ),
        (
            ".nb",
            vec![
                Rule::Prop("position", "relative"),
                Rule::Prop("border", "none"),
                Rule::Prop("display", "flex"),
                Rule::Prop("flex-direction", "row"),
                Rule::Prop("flex-basis", "auto"),
                // Rule::Descriptor(".e", elDescription),
                Rule::Batch(vec![
                    Rule::Descriptor(
                        ".a",
                        vec![
                            Rule::Prop("position", "absolute"),
                            Rule::Prop("bottom", "100%"),
                            Rule::Prop("left", "0"),
                            Rule::Prop("width", "100%"),
                            Rule::Prop("z-index", "20"),
                            Rule::Prop("margin", "0 !important"),
                            Rule::Child(
                                ".hf",
                                vec![Rule::Prop("height", "auto")],
                            ),
                            Rule::Child(
                                ".wf",
                                vec![Rule::Prop("width", "100%")],
                            ),
                            Rule::Prop("pointer-events", "none"),
                            Rule::Child(
                                "*",
                                vec![Rule::Prop("pointer-events", "auto")],
                            ),
                        ],
                    ),
                    Rule::Descriptor(
                        ".b",
                        vec![
                            Rule::Prop("position", "absolute"),
                            Rule::Prop("bottom", "0"),
                            Rule::Prop("left", "0"),
                            Rule::Prop("height", "0"),
                            Rule::Prop("width", "100%"),
                            Rule::Prop("z-index", "20"),
                            Rule::Prop("margin", "0 !important"),
                            Rule::Prop("pointer-events", "none"),
                            Rule::Child(
                                "*",
                                vec![Rule::Prop("pointer-events", "auto")],
                            ),
                            Rule::Child(
                                ".hf",
                                vec![Rule::Prop("height", "auto")],
                            ),
                        ],
                    ),
                    Rule::Descriptor(
                        ".or",
                        vec![
                            Rule::Prop("position", "absolute"),
                            Rule::Prop("left", "100%"),
                            Rule::Prop("top", "0"),
                            Rule::Prop("height", "100%"),
                            Rule::Prop("margin", "0 !important"),
                            Rule::Prop("z-index", "20"),
                            Rule::Prop("pointer-events", "none"),
                            Rule::Child(
                                "*",
                                vec![Rule::Prop("pointer-events", "auto")],
                            ),
                        ],
                    ),
                    Rule::Descriptor(
                        ".ol",
                        vec![
                            Rule::Prop("position", "absolute"),
                            Rule::Prop("right", "100%"),
                            Rule::Prop("top", "0"),
                            Rule::Prop("height", "100%"),
                            Rule::Prop("margin", "0 !important"),
                            Rule::Prop("z-index", "20"),
                            Rule::Prop("pointer-events", "none"),
                            Rule::Child(
                                "*",
                                vec![Rule::Prop("pointer-events", "auto")],
                            ),
                        ],
                    ),
                    Rule::Descriptor(
                        ".fr",
                        vec![
                            Rule::Prop("position", "absolute"),
                            Rule::Prop("width", "100%"),
                            Rule::Prop("height", "100%"),
                            Rule::Prop("left", "0"),
                            Rule::Prop("top", "0"),
                            Rule::Prop("margin", "0 !important"),
                            Rule::Prop("pointer-events", "none"),
                            Rule::Child(
                                "*",
                                vec![Rule::Prop("pointer-events", "auto")],
                            ),
                        ],
                    ),
                    Rule::Descriptor(
                        ".bh",
                        vec![
                            Rule::Prop("position", "absolute"),
                            Rule::Prop("width", "100%"),
                            Rule::Prop("height", "100%"),
                            Rule::Prop("left", "0"),
                            Rule::Prop("top", "0"),
                            Rule::Prop("margin", "0 !important"),
                            Rule::Prop("z-index", "0"),
                            Rule::Prop("pointer-events", "none"),
                            Rule::Child(
                                "*",
                                vec![Rule::Prop("pointer-events", "auto")],
                            ),
                        ],
                    ),
                ]),
            ],
        ),
        (
            ".s",
            vec![
                Rule::Prop("position", "relative"),
                Rule::Prop("border", "none"),
                Rule::Prop("flex-shrink", "0"),
                Rule::Prop("display", "flex"),
                Rule::Prop("flex-direction", "row"),
                Rule::Prop("flex-basis", "auto"),
                Rule::Prop("resize", "none"),
                Rule::Prop("font-feature-settings", "inherit"),
                // Rule::Prop("flex-basis", "0%"
                Rule::Prop("box-sizing", "border-box"),
                Rule::Prop("margin", "0"),
                Rule::Prop("padding", "0"),
                Rule::Prop("border-width", "0"),
                Rule::Prop("border-style", "solid"),
                // inheritable font properties
                Rule::Prop("font-size", "inherit"),
                Rule::Prop("color", "inherit"),
                Rule::Prop("font-family", "inherit"),
                Rule::Prop("line-height", "1"),
                Rule::Prop("font-weight", "inherit"),
                // Text decoration is *mandatorily inherited* in the css spec.
                // There's no way to change this.  How crazy is that?
                Rule::Prop("text-decoration", "none"),
                Rule::Prop("font-style", "inherit"),
                Rule::Descriptor(".wrp", vec![Rule::Prop("flex-wrap", "wrap")]),
                Rule::Descriptor(
                    ".notxt",
                    vec![
                        Rule::Prop("-moz-user-select", "none"),
                        Rule::Prop("-webkit-user-select", "none"),
                        Rule::Prop("-ms-user-select", "none"),
                        Rule::Prop("user-select", "none"),
                    ],
                ),
                Rule::Descriptor(
                    ".cptr",
                    vec![Rule::Prop("cursor", "pointer")],
                ),
                Rule::Descriptor(".ctxt", vec![Rule::Prop("cursor", "text")]),
                Rule::Descriptor(
                    ".ppe",
                    vec![Rule::Prop("pointer-events", "none !important")],
                ),
                Rule::Descriptor(
                    ".cpe",
                    vec![Rule::Prop("pointer-events", "auto !important")],
                ),
                Rule::Descriptor(".clr", vec![Rule::Prop("opacity", "0")]),
                Rule::Descriptor(".oq", vec![Rule::Prop("opacity", "1")]),
                Rule::Descriptor(
                    ".hv.clr:hover",
                    vec![Rule::Prop("opacity", "0")],
                ),
                Rule::Descriptor(
                    ".hv.oq:hover",
                    vec![Rule::Prop("opacity", "1")],
                ),
                Rule::Descriptor(
                    ".fcs.clr:focus",
                    vec![Rule::Prop("opacity", "0")],
                ),
                Rule::Descriptor(
                    ".fcs.oq:focus",
                    vec![Rule::Prop("opacity", "1")],
                ),
                Rule::Descriptor(
                    ".atv.clr:active",
                    vec![Rule::Prop("opacity", "0")],
                ),
                Rule::Descriptor(
                    ".atv.oq:active",
                    vec![Rule::Prop("opacity", "1")],
                ),
                Rule::Descriptor(".ts",
                    vec![
                        Rule::Prop(
                            "transition",
                            "transform 160ms, opacity 160ms, filter 160ms, background-color 160ms, color 160ms, font-size 160ms"
                        )
                    ]
                ),
                Rule::Descriptor(".sb",
                    vec![
                        Rule::Prop("overflow", "auto"),
                        Rule::Prop("flex-shrink", "1"),
                    ]
                ),
                Rule::Descriptor(".sbx",
                    vec![
                        Rule::Prop("overflow-x", "auto"),
                        Rule::Descriptor(".r",
                            vec![Rule::Prop("flex-shrink", "1")]
                        ),
                    ]
                ),
                Rule::Descriptor(".sby",
                    vec![
                        Rule::Prop("overflow-y", "auto"),
                        Rule::Descriptor(".c",
                            vec![Rule::Prop("flex-shrink", "1")],
                        ),
                        Rule::Descriptor(".e",
                            vec![Rule::Prop("flex-shrink", "1")]
                        ),
                    ]
                ),
                Rule::Descriptor(".cp",
                    vec![Rule::Prop("overflow", "hidden")]
                ),
                Rule::Descriptor(".cpx",
                    vec![Rule::Prop("overflow-x", "hidden")]
                ),
                Rule::Descriptor(".cpy",
                    vec![Rule::Prop("overflow-y", "hidden")]
                ),
                Rule::Descriptor(".wc",
                    vec![Rule::Prop("width", "auto")]
                ),
                Rule::Descriptor(".bn",
                    vec![Rule::Prop("border-width", "0")]
                ),
                Rule::Descriptor(".bd",
                    vec![Rule::Prop("border-style", "dashed")]
                ),
                Rule::Descriptor(".bdt",
                    vec![Rule::Prop("border-style", "dotted")]
                ),
                Rule::Descriptor(".bs",
                    vec![Rule::Prop("border-style", "solid")]
                ),
                Rule::Descriptor(".txt",
                    vec![
                        Rule::Prop("white-space", "pre"),
                        Rule::Prop("display", "inline-block"),
                    ]
                ),
                Rule::Descriptor("it",
                    // chrome and safari have a minimum recognized line height for text input of 1.05
                    // If it's 1, it bumps up to something like 1.2
                    vec![
                        Rule::Prop("line-height", "1.05"),
                        Rule::Prop("background", "transparent"),
                        Rule::Prop("text-align", "inherit"),
                    ]
                ),
                // Rule::Descriptor(".e", elDescription),
                Rule::Descriptor(".r",
                    vec![
                        Rule::Prop("display", "flex"),
                        Rule::Prop("flex-direction", "row"),
                        Rule::Child(".s",
                            vec![
                                Rule::Prop("flex-basis", "0%"),
                                Rule::Descriptor(".we",
                                    vec![ Rule::Prop("flex-basis", "auto"),]
                                ),
                                Rule::Descriptor(".lnk",
                                    vec![ Rule::Prop("flex-basis", "auto"),]
                                )
                            ]
                        ),
                        Rule::Child(".hf",
                            vec![
                                // alignTop, centerY, and alignBottom need to be disabled
                                Rule::Prop("align-self", "stretch !important")
                            ]
                        ),
                        Rule::Child(".hfp",
                            vec![
                                // alignTop, centerY, and alignBottom need to be disabled
                                Rule::Prop("align-self", "stretch !important"),
                            ]
                        ),
                    ],
                ),
                describe_alignment(Box::new(|alignment: &Alignment| match alignment {
                    Alignment::Top => (
                        vec![Rule::Prop("align-items", "flex-start")],
                        vec![Rule::Prop("align-self", "flex-start")],
                    ),

                    Alignment::Bottom => (
                        vec![Rule::Prop("align-items", "flex-end")],
                        vec![Rule::Prop("align-self", "flex-end")],
                    ),

                    Alignment::Right => {
                        (vec![Rule::Prop("justify-content", "flex-end")], vec![])
                    }

                    Alignment::Left => {
                        (vec![Rule::Prop("justify-content", "flex-start")], vec![])
                    }

                    Alignment::CenterX => {
                        (vec![Rule::Prop("justify-content", "center")], vec![])
                    }

                    Alignment::CenterY => (
                        vec![Rule::Prop("align-items", "center")],
                        vec![Rule::Prop("align-self", "center")],
                    ),
                })),
                // Must be below the alignment rules or else it interferes
                Rule::Descriptor("sev",
                    vec![ Rule::Prop("justify-content", "space-between")]
                ),
                Rule::Descriptor("lbl",
                    vec![ Rule::Prop("align-items", "baseline")]
                ),
                Rule::Descriptor(".c",
                    vec![
                        Rule::Prop("display", "flex"),
                        Rule::Prop("flex-direction", "column"),
                        Rule::Child(".s",
                            // *Note* - While rows have flex-basis 0%,
                            // which allows for the children of a row to default to their content size
                            // This apparently is a different story for columns.
                            // Safari has an issue if this is flex-basis: 0%, as it goes entirely to 0,    
                            // instead of the expected content size.
                            // So we add `min-height: min-content`, which isn't supported by IE, but works for all other browsers!
                            // Separately, 0% is different than 0px, but only for columns
                            // In columns, 0% will actually be calculated as `auto` for columns    
                            // So, 0px is the one we want.
                            vec![
                                Rule::Prop("flex-basis", "0px"),
                                Rule::Prop("min-height", "min-content"),
                                Rule::Descriptor("he",
                                    vec![Rule::Prop("flex-basis", "auto")]
                                ),
                            ]
                        ),
                        Rule::Child(".hf",
                            vec![Rule::Prop("flex-grow", "100000")]
                        ),
                        Rule::Child(".wf",
                            vec![ 
                                // alignLeft, alignRight, centerX need to be disabled
                                // Rule::Prop("align-self", "stretch !important"),
                                Rule::Prop("width", "100%"),
                            ]
                        ),
                        Rule::Child(".wfp",
                            vec![
                                // alignLeft, alignRight, centerX need to be disabled
                                // Rule::Prop("align-self", "stretch !important"),
                                Rule::Prop("width", "100%"),
                            ]
                        ),
        
                        // TODO:: This might be necessary, maybe it should move to widthFill?    
                        // Rule::Child(".wf",
                        //     vec![
                        //         Rule::Prop("align-self", "stretch"),
                        //         Rule::Descriptor("ah",
                        //             vec![Rule::Prop("width", "100%")]
                        //         ),
                        //     ]
                        // ),
                        Rule::Child(".wc",
                            vec![Rule::Prop("align-self", "flex-start")]
                        ),

                        // Rule::Child("alignTop:last-of-type.align-container-top",
                        //     vec![Rule::Prop("flex-grow", "1")]
                        // ),
                        Rule::Child("u:first-of-type.acb",
                            vec![Rule::Prop("flex-grow", "1")]
                        ),
        
                        // centerY -> <s>
                        // alignBottom -> <u>
                        // first center y
                        Rule::Child("s:first-of-type.accy",
                            vec![
                                Rule::Prop("flex-grow", "1"),
                                Rule::Child(".cy",
                                    vec![
                                        Rule::Prop("margin-top", "auto !important"),
                                        Rule::Prop("margin-bottom", "0 !important"),
                                    ]
                                ),
                            ]
                        ),
                        Rule::Child("s:last-of-type.accy",
                            vec![
                                Rule::Prop("flex-grow", "1"),
                                Rule::Child(".cy",
                                    vec![
                                        Rule::Prop("margin-bottom", "auto !important"),
                                        Rule::Prop("margin-top", "0 !important"),
                                    ]
                                ),
                            ]
                        ),

                        // lonley centerY
                        Rule::Child("s:only-of-type.accy",
                            vec![
                                Rule::Prop("flex-grow", "1"),
                                Rule::Child("cy",
                                    vec![
                                        Rule::Prop("margin-top", "auto !important"),
                                        Rule::Prop("margin-bottom", "auto !important"),
                                    ]
                                ),
                            ]
                        ),

                        // alignBottom's after a centerY should not grow
                        Rule::Child("s:last-of-type.accy ~ u",
                            vec![Rule::Prop("flex-grow", "0")]
                        ),

                        // centerY's after an alignBottom should be ignored
                        Rule::Child("u:first-of-type.acb ~ s.accy",
                            // Bottom alignment always overrides center alignment
                            vec![ Rule::Prop("flex-grow", "0")]
                        ),
                        describe_alignment(Box::new(|alignment: &Alignment| match alignment {
                            Alignment::Top => (
                                vec![Rule::Prop("justify-content", "flex-start")],
                                vec![Rule::Prop("margin-bottom", "auto")],
                            ),

                            Alignment::Bottom => (
                                vec![Rule::Prop("justify-content", "flex-end")],
                                vec![Rule::Prop("margin-top", "auto")],
                            ),

                            Alignment::Right => (
                                vec![Rule::Prop("align-items", "flex-end")],
                                vec![Rule::Prop("align-self", "flex-end")],
                            ),

                            Alignment::Left => (
                                vec![Rule::Prop("align-items", "flex-start")],
                                vec![Rule::Prop("align-self", "flex-start")],
                            ),

                            Alignment::CenterX => (
                                vec![Rule::Prop("align-items", "center")],
                                vec![Rule::Prop("align-self", "center")],
                            ),

                            Alignment::CenterY => {
                                (vec![Rule::Prop("justify-content", "center")], vec![])
                            }
                        })),
                        Rule::Child(".ctr",
                            vec![
                                Rule::Prop("flex-grow", "0"),
                                Rule::Prop("flex-basis", "auto"),
                                Rule::Prop("width", "100%"),
                                Rule::Prop("align-self", "stretch !important"),
                            ]
                        ),
                        Rule::Descriptor(".se",
                            vec![Rule::Prop("justify-content", "space-between")]
                        ),
                    ]
                ),
                Rule::Descriptor(".g",
                    vec![
                        Rule::Prop("display", "-ms-grid"),
                        Rule::Child(".gp",
                            vec![
                                Rule::Child(".s",
                                    vec![Rule::Prop("width", "100%")]
                                ),
                            ]
                        ),
                        Rule::Supports(("display", "grid"),
                            vec![("display", "grid")]
                        ),
                        grid_alignments(Box::new(|alignment: &Alignment| match alignment {
                            Alignment::Top =>
                                vec![Rule::Prop("justify-content", "flex-start")],
                            Alignment::Bottom =>
                                vec![Rule::Prop("justify-content", "flex-end")],
                            Alignment::Right =>
                                vec![Rule::Prop("align-items", "flex-end")],
                            Alignment::Left =>
                                vec![Rule::Prop("align-items", "flex-start")],
                            Alignment::CenterX =>
                                vec![Rule::Prop("align-items", "center")],
                            Alignment::CenterY =>
                                vec![Rule::Prop("justify-content", "center")],
                        }))
                    ]
                ),
                Rule::Descriptor(".pg",
                    vec![
                        Rule::Prop("display", "block"),
                        Rule::Child(".s:first-child",
                            vec![Rule::Prop("margin", "0 !important")]
                        ),

                        // clear spacing of any subsequent element if an element is float-left    
                        Rule::Child(".s.al:first-child + .s",
                            vec![Rule::Prop("margin", "0 !important")]
                        ),
                        Rule::Child(".s.ar:first-child + .s",
                            vec![Rule::Prop("margin", "0 !important")]
                        ),
                        describe_alignment(Box::new(|alignment: &Alignment| match alignment {    
                            Alignment::Top => (
                                vec![],
                                vec![],
                            ),
                            Alignment::Bottom => (
                                vec![],
                                vec![],
                            ),
                            Alignment::Right => (
                                vec![],
                                vec![
                                    Rule::Prop("float", "right"),
                                    Rule::Descriptor("::after",
                                        vec![
                                            Rule::Prop("content", "\"\""),
                                            Rule::Prop("display", "table"),
                                            Rule::Prop("clear", "both"),
                                        ]
                                    )
                                ],
                            ),
                            Alignment::Left => (
                                vec![],
                                vec![
                                    Rule::Prop("float", "left"),
                                    Rule::Descriptor("::after",
                                        vec![
                                            Rule::Prop("content", "\"\""),
                                            Rule::Prop("display", "table"),
                                            Rule::Prop("clear", "both"),
                                        ]
                                    ),
                                ],
                            ),
                            Alignment::CenterX => (
                                vec![],
                                vec![],
                            ),
                            Alignment::CenterY => (
                                vec![],
                                vec![],
                            ),
                        }))
                    ]
                ),
                Rule::Descriptor(".iml",
                    vec![
                        Rule::Prop("white-space", "pre-wrap !important"),
                        Rule::Prop("height", "100%"),
                        Rule::Prop("width", "100%"),
                        Rule::Prop("background-color", "transparent"),
                    ]
                ),
                Rule::Descriptor(".implw",
                    // Get this.
                    // This allows multiline input to anchor scrolling to the bottom of the node
                    // when in a scrolling viewport, and the user is adding content.
                    // however, it only works in chrome.  In firefox, it prevents scrolling.
                    //
                    // But how crazy is this solution?
                    // vec![
                    //     Rule::Prop("display", "flex"),
                    //     Rule::Prop("flex-direction", "column-reverse"),
                    // ]
                    vec![
                        // to increase specificity to beat another rule
                        Rule::Descriptor(".e",
                            vec![Rule::Prop("flex-basis", "auto")]
                        ),
                    ]
                ),
                Rule::Descriptor("imlp",
                    vec![
                        Rule::Prop("white-space", "pre-wrap !important"),
                        Rule::Prop("cursor", "text"),
                        Rule::Child(".imlf",
                            vec![
                                Rule::Prop("white-space", "pre-wrap !important"),
                                Rule::Prop("color", "transparent"),
                            ]
                        ),
                    ]
                ),
                Rule::Descriptor(".p",
                    vec![
                        Rule::Prop("display", "block"),
                        Rule::Prop("white-space", "normal"),
                        Rule::Prop("overflow-wrap", "break-word"),
                        Rule::Descriptor("hbh",
                            vec![
                                Rule::Prop("z-index", "0"),
                                Rule::Child(".bh",
                                    vec![Rule::Prop("z-index", "-1")]
                                ),
                            ]
                        ),
                        Rule::AllChildren(".txt".to_string(),
                            vec![
                                Rule::Prop("display", "inline"),
                                Rule::Prop("white-space", "normal"),
                            ]
                        ),
                        Rule::AllChildren(".p".to_string(),
                            vec![
                                Rule::Prop("display", "inline"),
                                Rule::Descriptor("::after",
                                    vec![Rule::Prop("content", "none")]
                                ),
                                Rule::Descriptor("::before",
                                    vec![Rule::Prop("content", "none")]
                                ),
                            ]
                        ),
                        Rule::AllChildren(".e".to_string(),
                            vec![
                                Rule::Prop("display", "inline"),
                                Rule::Prop("white-space", "normal"),
                                // Inline block allows the width of the item to be set
                                // but DOES NOT like wrapping text in a standard, normal, sane way.        
                                // We're sorta counting that if an exact width has been set,    
                                // people aren't expecting proper text wrapping for this element        
                                Rule::Descriptor(".we",
                                    vec![Rule::Prop("display", "inline-block")]
                                ),
                                Rule::Descriptor(".fr",
                                    vec![Rule::Prop("display", "flex")],
                                ),
                                Rule::Descriptor(".bh",
                                    vec![Rule::Prop("display", "flex")],
                                ),
                                Rule::Descriptor(".a",
                                    vec![Rule::Prop("display", "flex")],
                                ),
                                Rule::Descriptor(".b",
                                    vec![Rule::Prop("display", "flex")],
                                ),
                                Rule::Descriptor(".or",
                                    vec![Rule::Prop("display", "flex")],
                                ),
                                Rule::Descriptor(".ol",
                                    vec![Rule::Prop("display", "flex")],
                                ),
                                Rule::Child(".txt",
                                    vec![
                                        Rule::Prop("display", "inline"),
                                        Rule::Prop("white-space", "normal"),
                                    ]
                                ),
                            ]
                        ),
                        Rule::Child(".r",
                            vec![Rule::Prop("display", "inline")]
                        ),
                        Rule::Child(".c",
                            vec![Rule::Prop("display", "inline-flex")]
                        ),
                        Rule::Child(".g",
                            vec![Rule::Prop("display", "inline-grid")]
                        ),
                        describe_alignment(Box::new(|alignment: &Alignment| match alignment {    
                            Alignment::Top => (
                                vec![],
                                vec![],
                            ),
                            Alignment::Bottom => (
                                vec![],
                                vec![],
                            ),
                            Alignment::Right => (
                                vec![],
                                vec![
                                    Rule::Prop("float", "right"),
                                ],
                            ),
                            Alignment::Left => (
                                vec![],
                                vec![
                                    Rule::Prop("float", "left"),
                                ],
                            ),
                            Alignment::CenterX => (
                                vec![],
                                vec![],
                            ),
                            Alignment::CenterY => (
                                vec![],
                                vec![],
                            ),
                        })),
                    ]
                ),
                Rule::Descriptor(".hidden",
                    vec![Rule::Prop("display", "none")]
                ),
                Rule::Descriptor(".w1",
                    vec![Rule::Prop("font-weight", "100")]
                ),
                Rule::Descriptor(".w2",
                    vec![Rule::Prop("font-weight", "200")]
                ),
                Rule::Descriptor(".w3",
                    vec![Rule::Prop("font-weight", "300")]
                ),
                Rule::Descriptor(".w4",
                    vec![Rule::Prop("font-weight", "400")]
                ),
                Rule::Descriptor(".w5",
                    vec![Rule::Prop("font-weight", "500")]
                ),
                Rule::Descriptor(".w6",
                    vec![Rule::Prop("font-weight", "600")]
                ),
                Rule::Descriptor(".w7",
                    vec![Rule::Prop("font-weight", "700")]
                ),
                Rule::Descriptor(".w8",
                    vec![Rule::Prop("font-weight", "800")]
                ),
                Rule::Descriptor("w9",
                    vec![Rule::Prop("font-weight", "900")]
                ),
                Rule::Descriptor(".i",
                    vec![Rule::Prop("font-style", "italic")]
                ),
                Rule::Descriptor(".sk",
                    vec![Rule::Prop("text-decoration", "line-through")]
                ),
                Rule::Descriptor(".u",
                    vec![
                        Rule::Prop("text-decoration", "underline"),
                        Rule::Prop("text-decoration-skip-ink", "auto"),
                        Rule::Prop("text-decoration-skip", "ink"),
                    ]
                ),
                Rule::Descriptor(".u.sk",
                    vec![
                        Rule::Prop("text-decoration", "line-througunderline"),
                        Rule::Prop("text-decoration-skip-ink", "auto"),
                        Rule::Prop("text-decoration-skip", "ink"),
                    ]
                ),
                Rule::Descriptor(".tun",
                    vec![Rule::Prop("font-style", "normal")]
                ),
                Rule::Descriptor(".tj",
                    vec![Rule::Prop("text-align", "justify")]
                ),
                Rule::Descriptor(".tja",
                    vec![Rule::Prop("text-align", "justify-all")]
                ),
                Rule::Descriptor(".tc",
                    vec![Rule::Prop("text-align", "center")]
                ),
                Rule::Descriptor(".tr",
                    vec![Rule::Prop("text-align", "right")]
                ),
                Rule::Descriptor(".tl",
                    vec![Rule::Prop("text-align", "left")]
                ),
                Rule::Descriptor(".modal",
                    vec![
                        Rule::Prop("position", "fixed"),
                        Rule::Prop("left", "0"),
                        Rule::Prop("top", "0"),
                        Rule::Prop("width", "100%"),
                        Rule::Prop("height", "100%"),
                        Rule::Prop("pointer-events", "none"),
                    ]
                ),
            ],
        ),
    ]
}
