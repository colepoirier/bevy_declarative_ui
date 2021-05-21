use std::collections::HashSet;

pub mod virtual_dom {
    use crate::model::Property;

    #[derive(Debug, Default, PartialOrd, PartialEq, Clone)]
    pub struct Node {
        pub tag: String,
        pub attrs: Vec<Attribute>,
        pub children: Vec<NodeType>,
    }

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub enum NodeType {
        Node(Node),
        KeyedNode(String, Node),
        Text(String),
    }

    impl Default for NodeType {
        fn default() -> Self {
            NodeType::Node(Node {
                tag: "div".to_string(),
                attrs: vec![],
                children: vec![],
            })
        }
    }

    pub fn text(txt: String) -> NodeType {
        NodeType::Text(txt)
    }

    pub fn node(
        tag: String,
        attrs: Vec<Attribute>,
        children: Vec<NodeType>,
    ) -> Node {
        Node {
            tag,
            attrs,
            children,
        }
    }

    pub fn keyed_node(
        key: String,
        tag: String,
        attrs: Vec<Attribute>,
        children: Vec<NodeType>,
    ) -> NodeType {
        NodeType::KeyedNode(
            key,
            Node {
                tag,
                attrs,
                children,
            },
        )
    }

    pub fn property(property: Property) -> Attribute {
        Attribute(format!("{}={}", property.0, property.1))
    }

    #[derive(Debug, Default, PartialOrd, PartialEq, Clone)]
    pub struct Attribute(pub String);
}

pub mod html {
    use crate::model::virtual_dom as vdom;
    use vdom::{node, Node, NodeType};

    // pub type Node = Node;

    pub fn text(txt: String) -> NodeType {
        vdom::text(txt)
    }

    pub fn div(attrs: Vec<vdom::Attribute>, children: Vec<NodeType>) -> Node {
        Node {
            tag: "div".to_string(),
            attrs,
            children,
        }
    }

    // paragraph html tag
    pub fn p(attrs: Vec<vdom::Attribute>, children: Vec<NodeType>) -> Node {
        Node {
            tag: "p".to_string(),
            attrs,
            children,
        }
    }

    // strikethrough html tag
    pub fn s(attrs: Vec<vdom::Attribute>, children: Vec<NodeType>) -> Node {
        Node {
            tag: "s".to_string(),
            attrs,
            children,
        }
    }

    // underline html tag
    pub fn u(attrs: Vec<vdom::Attribute>, children: Vec<NodeType>) -> Node {
        Node {
            tag: "u".to_string(),
            attrs,
            children,
        }
    }

    pub mod attributes {
        use crate::model::virtual_dom as vdom;

        pub fn class(cls: String) -> vdom::Attribute {
            vdom::Attribute(cls)
        }

        pub fn style(k: String, v: String) -> vdom::Attribute {
            vdom::Attribute(format!("{}={}", k, v))
        }

        pub fn src(s: String) -> vdom::Attribute {
            style("src".to_string(), s)
        }

        pub fn alt(description: String) -> vdom::Attribute {
            style("alt".to_string(), description)
        }

        pub fn href(url: String) -> vdom::Attribute {
            style("href".to_string(), url)
        }

        pub fn rel(r: String) -> vdom::Attribute {
            style("rel".to_string(), r)
        }

        pub fn target(t: String) -> vdom::Attribute {
            style("target".to_string(), t)
        }

        pub fn download(file_name: String) -> vdom::Attribute {
            style("download".to_string(), file_name)
        }
    }
}

use crate::flag::{Field, Flag};
use crate::style;
use crate::style::Classes;
use html::attributes;
use virtual_dom as vdom;
use virtual_dom::{Node, NodeType};

use self::vdom::property;

#[derive(Debug, Clone)]
pub enum Element {
    Unstyled(FinalizeNodeArgs),
    Styled(Styled),
    Text(String),
    Empty,
}
#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum EmbedStyle {
    NoStyleSheet,
    StaticRootAndynamic(OptStruct, Vec<Style>),
    OnlyDynamic(OptStruct, Vec<Style>),
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum LayoutContext {
    AsRow,
    AsColumn,
    AsEl,
    AsGrid,
    AsParagraph,
    AsTextColumn,
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Aligned {
    Unaligned,
    Aligned(Option<HAlign>, Option<VAlign>),
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum HAlign {
    Left,
    CenterX,
    Right,
}

impl HAlign {
    // alignXName
    pub fn name(&self) -> String {
        match self {
            Self::Left => format!(
                "{} {}",
                Classes::AlignedHorizontally.to_string(),
                Classes::AlignLeft.to_string()
            ),
            Self::Right => format!(
                "{} {}",
                Classes::AlignedHorizontally.to_string(),
                Classes::AlignRight.to_string(),
            ),
            Self::CenterX => format!(
                "{} {}",
                Classes::AlignedHorizontally.to_string(),
                Classes::AlignCenterX.to_string(),
            ),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum VAlign {
    Top,
    CenterY,
    Bottom,
}

impl VAlign {
    // alignXName
    pub fn name(&self) -> String {
        match self {
            Self::Top => format!(
                "{} {}",
                Classes::AlignedVertically.to_string(),
                Classes::AlignTop.to_string()
            ),
            Self::Bottom => format!(
                "{} {}",
                Classes::AlignedVertically.to_string(),
                Classes::AlignBottom.to_string(),
            ),
            Self::CenterY => format!(
                "{} {}",
                Classes::AlignedVertically.to_string(),
                Classes::AlignCenterY.to_string(),
            ),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Style {
    Style(String, Vec<Property>),
    FontFamily(String, Vec<Font>),
    FontSize(u8),
    Single(String, String, String),
    Colored(String, String, Color),
    Spacing(String, u8, u8),
    BorderWidth(String, u8, u8, u8, u8),
    Padding(String, f32, f32, f32, f32),
    GridTemplate(GridTemplate),
    GridPosition(GridPosition),
    Transform(Transform),
    PseudoSelector(PseudoClass, Vec<Style>),
    Transparency(String, f32),
    Shadows(String, String),
}

impl Style {
    pub fn name(&self) -> String {
        match self {
            Self::Shadows(name, _) => name.clone(),
            Self::Transparency(name, _) => name.clone(),
            Self::Style(class, _) => class.clone(),
            Self::FontFamily(name, _) => name.clone(),
            Self::FontSize(i) => format!("font-size-{}", i),
            Self::Single(class, _, _) => class.clone(),
            Self::Colored(class, _, _) => class.clone(),
            Self::Spacing(cls, _, _) => cls.clone(),
            Self::Padding(cls, _, _, _, _) => cls.clone(),
            Self::BorderWidth(cls, _, _, _, _) => cls.clone(),
            Self::GridTemplate(template) => {
                let rows = template
                    .rows
                    .iter()
                    .map(|r| r.class_name())
                    .collect::<Vec<String>>()
                    .join("-");
                let cols = template
                    .columns
                    .iter()
                    .map(|c| c.class_name())
                    .collect::<Vec<String>>()
                    .join("-");
                format!(
                    "grid-rows-{}-cols-{}-space-x-{}-space-y-{}",
                    rows,
                    cols,
                    template.spacing.0.class_name(),
                    template.spacing.1.class_name(),
                )
            }
            Self::GridPosition(pos) => format!(
                "gp grid-pos-{}-{}-{}-{}",
                pos.row, pos.col, pos.width, pos.height,
            ),
            Self::PseudoSelector(selector, style) => {
                let s_name = match selector {
                    PseudoClass::Focus => "fs",
                    PseudoClass::Hover => "hv",
                    PseudoClass::Active => "act",
                };

                style
                    .iter()
                    .map(|s| match &s.name()[..] {
                        "" => String::new(),
                        name => format!("{}-{}", s_name, name),
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            }
            Self::Transform(x) => x.class().unwrap_or_default(),
        }
    }
    pub fn toplevel_val(&self) -> Option<(String, Vec<Font>)> {
        if let Style::FontFamily(name, typefaces) = self {
            Some((name.clone(), typefaces.clone()))
        } else {
            None
        }
    }
    pub fn tag(self, label: String) -> Self {
        match self {
            Self::Single(class, prop, val) => {
                Self::Single(format!("{}-{}", label, class), prop, val)
            }
            Self::Colored(class, prop, val) => {
                Self::Colored(format!("{}-{}", label, class), prop, val)
            }
            Self::Style(class, props) => {
                Self::Style(format!("{}-{}", label, class), props)
            }
            Self::Transparency(class, o) => {
                Self::Transparency(format!("{}-{}", label, class), o)
            }
            x => x,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum Transform {
    Untransformed,
    Moved(Coordinate),
    FullTransform(Coordinate, Coordinate, Coordinate, Angle),
}

impl Transform {
    pub fn class(&self) -> Option<String> {
        match self {
            Self::Untransformed => None,
            Self::Moved(Coordinate { x, y, z }) => {
                Some(format!("mv-{}-{}-{}", x, y, z))
            }
            Self::FullTransform(
                Coordinate {
                    x: tx,
                    y: ty,
                    z: tz,
                },
                Coordinate {
                    x: sx,
                    y: sy,
                    z: sz,
                },
                Coordinate {
                    x: ox,
                    y: oy,
                    z: oz,
                },
                angle,
            ) => Some(format!(
                "trfm-{}-{}-{}-{}-{}-{}-{}-{}-{}-{}",
                tx.float_class(),
                ty.float_class(),
                tz.float_class(),
                sx.float_class(),
                sy.float_class(),
                sz.float_class(),
                ox.float_class(),
                oy.float_class(),
                oz.float_class(),
                angle.0.float_class(),
            )),
        }
    }
    pub fn value(&self) -> Option<String> {
        match self {
            Self::Untransformed => None,
            Self::Moved(Coordinate { x, y, z }) => {
                Some(format!("translate3d({}px, {}px, {}px)", x, y, z))
            },
            Self::FullTransform(
                Coordinate {
                    x: tx,
                    y: ty,
                    z: tz,
                },
                Coordinate {
                    x: sx,
                    y: sy,
                    z: sz,
                },
                Coordinate {
                    x: ox,
                    y: oy,
                    z: oz,
                },
                angle,
            ) => Some(format!(
                "translate3d({}px, {}px, {}px) scale3d({}, {}, {}) rotate3d({}, {}, {}, {}rad)", tx, ty, tz, sx, sy, sz, ox ,oy, oz, angle.0
            )),
        }
    }
    pub fn compose(&self, component: &TransformComponent) -> Self {
        match self {
            Self::Untransformed => match component {
                TransformComponent::MoveX(x) => Self::Moved(Coordinate {
                    x: *x,
                    y: 0.0,
                    z: 0.0,
                }),
                TransformComponent::MoveY(y) => Self::Moved(Coordinate {
                    x: 0.0,
                    y: *y,
                    z: 0.0,
                }),
                TransformComponent::MoveZ(z) => Self::Moved(Coordinate {
                    x: 0.0,
                    y: 0.0,
                    z: *z,
                }),
                TransformComponent::Move(coord) => Self::Moved(*coord),
                TransformComponent::Rotate(coord, angle) => {
                    Self::FullTransform(
                        Coordinate::default(),
                        Coordinate::one(),
                        *coord,
                        Angle(*angle),
                    )
                }
                TransformComponent::Scale(coord) => Self::FullTransform(
                    Coordinate::default(),
                    *coord,
                    Coordinate {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    Angle(0.0),
                ),
            },
            Self::Moved(moved) => {
                let Coordinate { x, y, z } = moved;
                match component {
                    TransformComponent::MoveX(x) => Self::Moved(Coordinate {
                        x: *x,
                        y: *y,
                        z: *z,
                    }),
                    TransformComponent::MoveY(y) => Self::Moved(Coordinate {
                        x: *x,
                        y: *y,
                        z: *z,
                    }),
                    TransformComponent::MoveZ(z) => Self::Moved(Coordinate {
                        x: *x,
                        y: *y,
                        z: *z,
                    }),
                    TransformComponent::Move(coord) => Self::Moved(*coord),
                    TransformComponent::Rotate(coord, angle) => {
                        Self::FullTransform(
                            *moved,
                            Coordinate::one(),
                            *coord,
                            Angle(*angle),
                        )
                    }
                    TransformComponent::Scale(coord) => Self::FullTransform(
                        *moved,
                        *coord,
                        Coordinate {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        Angle(0.0),
                    ),
                }
            }
            Self::FullTransform(moved, scaled, origin, angle) => {
                let Coordinate { x, y, z } = moved;
                match component {
                    TransformComponent::MoveX(x) => Self::FullTransform(
                        Coordinate {
                            x: *x,
                            y: *y,
                            z: *z,
                        },
                        *scaled,
                        *origin,
                        *angle,
                    ),
                    TransformComponent::MoveY(y) => Self::FullTransform(
                        Coordinate {
                            x: *x,
                            y: *y,
                            z: *z,
                        },
                        *scaled,
                        *origin,
                        *angle,
                    ),
                    TransformComponent::MoveZ(z) => Self::FullTransform(
                        Coordinate {
                            x: *x,
                            y: *y,
                            z: *z,
                        },
                        *scaled,
                        *origin,
                        *angle,
                    ),
                    TransformComponent::Move(coord) => {
                        Self::FullTransform(*coord, *scaled, *origin, *angle)
                    }
                    TransformComponent::Rotate(coord, angle) => {
                        Self::FullTransform(
                            *moved,
                            *scaled,
                            *coord,
                            Angle(*angle),
                        )
                    }
                    TransformComponent::Scale(coord) => {
                        Self::FullTransform(*moved, *coord, *origin, *angle)
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum PseudoClass {
    Focus,
    Hover,
    Active,
}
#[derive(Debug, Clone)]
pub struct FinalizeNodeArgs {
    has: Field,
    node: NodeName,
    attributes: Vec<vdom::Attribute>,
    children: Children<Node>,
    embed_mode: Option<EmbedStyle>,
}

#[derive(Debug, Clone)]
pub struct Styled {
    styles: Vec<Style>,
    html: FinalizeNodeArgs,
}

pub struct AdjustmentRules {
    full: AdjustmentRule,
    capital: AdjustmentRule,
}

pub type AdjustmentRule = (Vec<(String, String)>, Vec<(String, String)>);

#[derive(Debug, Default, PartialOrd, PartialEq, Clone, Copy)]
pub struct AdjustmentSizes {
    vertical: f32,
    height: f32,
    size: f32,
}

impl AdjustmentSizes {
    pub fn new(size: f32, height: f32, vertical: f32) -> Self {
        Self {
            vertical,
            height: height / size,
            size,
        }
    }
    pub fn font_adjustment_rules(&self) -> AdjustmentRule {
        (
            vec![("display".to_string(), "block".to_string())],
            vec![
                ("display".to_string(), "inline-block".to_string()),
                ("line-height".to_string(), self.height.to_string()),
                ("vertical-align".to_string(), format!("{}em", self.vertical)),
                ("font-size".to_string(), format!("{}em", self.size)),
            ],
        )
    }
}

pub struct AdjustmentSizeRules {
    full: AdjustmentSizes,
    capital: AdjustmentSizes,
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct Adjustment {
    capital: f32,
    lowercase: f32,
    baseline: f32,
    descender: f32,
}

impl Adjustment {
    pub fn convert_to_size_rules(&self) -> AdjustmentSizeRules {
        let lines =
            vec![self.capital, self.baseline, self.descender, self.lowercase];

        let asc = lines
            .iter()
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap_or(&self.capital);

        let dsc = lines
            .iter()
            .min_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap_or(&self.descender);

        let baseline = lines
            .iter()
            .filter(|x| *x != dsc)
            .min_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap_or(&self.baseline);

        let c_vertical = 1.0 - asc;
        let f_vertical = c_vertical;
        let c_diff = (asc - baseline);
        let c_size = 1.0 / c_diff;
        let f_diff = (asc - dsc);
        let f_size = 1.0 / f_diff;

        AdjustmentSizeRules {
            full: AdjustmentSizes::new(f_size, f_diff, f_vertical),
            capital: AdjustmentSizes::new(c_size, c_diff, c_vertical),
        }
    }
}

pub fn typeface_adjustment(typefaces: &Vec<Font>) -> Option<AdjustmentRules> {
    typefaces.iter().fold(None, |found, face| {
        if found.is_none() {
            if let Font::FontWith(with) = face {
                if let Some(adj) = with.adjustment {
                    let ar = adj.convert_to_size_rules();
                    Some(AdjustmentRules {
                        full: ar.full.font_adjustment_rules(),
                        capital: ar.capital.font_adjustment_rules(),
                    })
                } else {
                    found
                }
            } else {
                found
            }
        } else {
            found
        }
    });
    None
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Font {
    Serif,
    SansSerif,
    Monospace,
    Typeface(String),
    ImportFont(String, String),
    FontWith(FontWith),
}

impl Font {
    pub fn render_variants(&self) -> Option<String> {
        match self {
            Font::FontWith(font) => Some(
                font.variants
                    .iter()
                    .map(|variant| variant.render())
                    .collect::<Vec<String>>()
                    .join(", "),
            ),
            _ => None,
        }
    }
    pub fn render_class_name(&self, mut current: String) -> String {
        let name = match self {
            Self::Serif => "serif".to_string(),

            Self::SansSerif => "sans-serif".to_string(),

            Self::Monospace => "monospace".to_string(),

            Self::Typeface(name) => {
                name.clone().to_lowercase().replace(" ", "-")
            }

            Self::ImportFont(name, _) => name.to_lowercase().replace(" ", "-"),

            Self::FontWith(FontWith {
                name,
                adjustment,
                variants,
            }) => name.to_lowercase().replace(" ", "-"),
        };
        current.push_str(&name);
        current
    }
    pub fn name(&self) -> String {
        match self {
            Self::Serif => "serif".to_string(),
            Self::SansSerif => "sans-serif".to_string(),
            Self::Monospace => "monospace".to_string(),
            Self::Typeface(name) => format!("\"{}\"", name),
            Self::ImportFont(name, _) => format!("\"{}\"", name),
            Self::FontWith(FontWith {
                name,
                adjustment,
                variants,
            }) => format!("\"{}\"", name),
        }
    }
    pub fn has_small_caps(&self) -> bool {
        match self {
            Font::FontWith(font) => {
                font.variants.iter().any(|v| v.is_small_caps())
            }
            _ => false,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct FontWith {
    name: String,
    adjustment: Option<Adjustment>,
    variants: Vec<Variant>,
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Variant {
    Active(String),
    Off(String),
    Indexed(String, u64),
}

impl Variant {
    fn render(&self) -> String {
        match self {
            Variant::Active(name) => format!("\"{}\"", name),
            Variant::Off(name) => format!("\"{}\" 0", name),
            Variant::Indexed(name, index) => format!("\"{}\" {}", name, index),
        }
    }
    fn name(&self) -> String {
        match self {
            Variant::Active(name) => String::from(name),
            Variant::Off(name) => format!("{}-0", name),
            Variant::Indexed(name, index) => format!("{}-{}", name, index),
        }
    }
    fn is_small_caps(&self) -> bool {
        match self {
            Variant::Active(name) => name == "smcp",
            Variant::Off(_) => false,
            Variant::Indexed(name, index) => name == "smcp" && *index == 1,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Property(String, String);

#[derive(Debug, Default, PartialOrd, PartialEq, Clone, Copy)]
pub struct Coordinate {
    x: f32,
    y: f32,
    z: f32,
}

impl Coordinate {
    pub fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct Angle(f32);

#[derive(Clone)]
pub enum Attribute {
    None, // NoAttribute
    Attr(vdom::Attribute),
    Describe(Description),
    Class(Flag, String), // invalidation key and literal class
    Style(Flag, Style), // invalidation key "border-color" as opposed to "border-color-10-10-10" that will be the key for the class
    AlignY(VAlign),
    AlignX(HAlign),
    Width(Length),
    Height(Length),
    Nearby(Location, Element),
    TransformComponent(Flag, TransformComponent),
}

impl Attribute {
    pub fn html_class(cls: String) -> Self {
        Self::Attr(attributes::class(cls))
    }
}

impl Attribute {
    pub fn only_styles(&self) -> Option<Style> {
        match self {
            Self::Style(_, style) => Some(style.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum TransformComponent {
    MoveX(f32),
    MoveY(f32),
    MoveZ(f32),
    Move(Coordinate),
    Rotate(Coordinate, f32),
    Scale(Coordinate),
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Description {
    Main,
    Navigation,
    // Search,
    ContentInfo,
    Complementary,
    Heading(u64),
    Label(String),
    LivePolite,
    LiveAssertive,
    Button,
    Paragraph,
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Length {
    Px(u64),
    Content,
    Fill(u64),
    Min(u64, Box<Length>),
    Max(u64, Box<Length>),
}

impl std::fmt::Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Length::Px(px) => write!(f, "{}px", px),
            Length::Content => write!(f, "auto"),
            Length::Fill(i) => write!(f, "{}fr", i),
            Length::Min(min, len) => {
                write!(f, "min{}{}", min, len)
            }
            Length::Max(max, len) => {
                write!(f, "max{}{}", max, len)
            }
        }
    }
}

impl Length {
    pub fn class_name(&self) -> String {
        match self {
            Length::Px(px) => format!("{}px", px),
            Length::Content => String::from("auto"),
            Length::Fill(i) => format!("{}fr", i),
            Length::Min(min, len) => {
                format!("min{}{}", min, len)
            }
            Length::Max(max, len) => {
                format!("max{}{}", max, len)
            }
        }
    }
    pub fn render_width(&self) -> (Field, String, Vec<Style>) {
        match self {
            Length::Px(px) => (
                Field::none(),
                format!("{} width-px-{}", Classes::WidthExact.to_string(), px),
                vec![Style::Single(
                    format!("width-px-{}", px),
                    "width".to_string(),
                    format!("{}px", px),
                )],
            ),
            Length::Content => {
                let mut flag = Field::none();
                flag.add(&Flag::width_content());
                (flag, Classes::WidthContent.to_string().to_string(), vec![])
            }
            Length::Fill(portion) => {
                if *portion == 1 {
                    let mut flag = Field::none();
                    flag.add(&Flag::width_fill());
                    (flag, Classes::WidthFill.to_string().into(), vec![])
                } else {
                    let mut flag = Field::none();
                    flag.add(&Flag::width_fill());
                    (
                        flag,
                        format!(
                            "{} width-fill-{}",
                            Classes::WidthFillPortion.to_string(),
                            portion
                        ),
                        vec![Style::Single(
                            format!(
                                "{}.{} > .width-fill-{}",
                                Classes::Any.to_string(),
                                Classes::Row.to_string(),
                                portion
                            ),
                            "flex-grow".into(),
                            (portion * 100000).to_string(),
                        )],
                    )
                }
            }
            Length::Min(size, len) => {
                let cls = format!("min-width-{}", size);
                let mut style = vec![Style::Single(
                    cls.clone(),
                    "min-width".into(),
                    format!("{}px", size),
                )];

                let (mut flag, attrs, new_style) = len.render_width();
                style.extend(new_style);
                flag.add(&Flag::width_between());
                (flag, format!("{} {}", cls, attrs), style)
            }
            Length::Max(size, len) => {
                let cls = format!("max-width-{}", size);
                let mut style = vec![Style::Single(
                    cls.clone(),
                    "max-width".into(),
                    format!("{}px", size),
                )];

                let (mut flag, attrs, new_style) = len.render_width();
                style.extend(new_style);
                flag.add(&Flag::width_between());
                (flag, format!("{} {}", cls, attrs), style)
            }
        }
    }
    pub fn render_height(&self) -> (Field, String, Vec<Style>) {
        match self {
            Length::Px(px) => (
                Field::none(),
                format!(
                    "{} height-px-{}",
                    Classes::HeightExact.to_string(),
                    px
                ),
                vec![Style::Single(
                    format!("height-px-{}", px),
                    "height".to_string(),
                    format!("{}px", px),
                )],
            ),
            Length::Content => {
                let mut flag = Field::none();
                flag.add(&Flag::height_content());
                (flag, Classes::HeightContent.to_string().to_string(), vec![])
            }
            Length::Fill(portion) => {
                if *portion == 1 {
                    let mut flag = Field::none();
                    flag.add(&Flag::height_fill());
                    (flag, Classes::HeightFill.to_string().into(), vec![])
                } else {
                    let mut flag = Field::none();
                    flag.add(&Flag::height_fill());
                    (
                        flag,
                        format!(
                            "{} height-fill-{}",
                            Classes::HeightFillPortion.to_string(),
                            portion
                        ),
                        vec![Style::Single(
                            format!(
                                "{}.{} > .height-fill-{}",
                                Classes::Any.to_string(),
                                Classes::Column.to_string(),
                                portion
                            ),
                            "flex-grow".into(),
                            (portion * 100000).to_string(),
                        )],
                    )
                }
            }
            Length::Min(size, len) => {
                let cls = format!("min-height-{}", size);
                let mut style = vec![Style::Single(
                    cls.clone(),
                    "min-height".into(),
                    // This needs to be !important because we're using
                    // `min-height: min-content` to correct for
                    // safari's incorrect implementation of flexbox.
                    format!("{}px !important", size),
                )];

                let (mut flag, attrs, new_style) = len.render_height();
                style.extend(new_style);
                flag.add(&Flag::height_between());
                (flag, format!("{} {}", cls, attrs), style)
            }
            Length::Max(size, len) => {
                let cls = format!("max-height-{}", size);
                let mut style = vec![Style::Single(
                    cls.clone(),
                    "max-height".into(),
                    format!("{}px", size),
                )];

                let (mut flag, attrs, new_style) = len.render_height();
                style.extend(new_style);
                flag.add(&Flag::height_between());
                (flag, format!("{} {}", cls, attrs), style)
            }
        }
    }
    pub fn is_content(&self) -> bool {
        match self {
            Self::Content => true,
            Self::Max(_, l) => l.is_content(),
            Self::Min(_, l) => l.is_content(),
            Self::Fill(_) => false,
            Self::Px(_) => false,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    All,
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum Location {
    Above,
    Below,
    OnRight,
    OnLeft,
    InFront,
    Behind,
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn format_color(&self) -> String {
        let Self { r, g, b, a } = self;
        format!(
            "rgba({},{},{},{})",
            r.float_class(),
            g.float_class(),
            b.float_class(),
            (*a as u32).to_string()
        )
    }
    pub fn format_color_class(&self) -> String {
        let Self { r, g, b, a } = self;
        format!(
            "rgba({},{},{},{})",
            r.float_class(),
            g.float_class(),
            b.float_class(),
            a.float_class()
        )
    }
}

trait FloatClass {
    fn float_class(&self) -> String;
}

impl FloatClass for f32 {
    fn float_class(&self) -> String {
        ((self * 255.0).round() as u32).to_string()
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum NodeName {
    Generic,
    NodeName(String),
    Embedded(String, String),
}

pub fn div() -> NodeName {
    NodeName::div()
}

impl NodeName {
    pub fn div() -> NodeName {
        NodeName::Generic
    }
    pub fn add(self, new_node: String) -> NodeName {
        match self {
            NodeName::Generic => NodeName::NodeName(new_node),
            NodeName::NodeName(name) => NodeName::Embedded(name, new_node),
            NodeName::Embedded(_, _) => self,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum NearbyChildren {
    None,
    Behind(Vec<Node>),
    InFront(Vec<Node>),
    Both(NearbyChildrenBoth),
}

impl NearbyChildren {
    pub fn add_nearby_el(self, loc: &Location, el: &Element) -> Self {
        let nearby = nearby_el(loc, el);
        match self {
            NearbyChildren::None => match loc {
                Location::Behind => NearbyChildren::Behind(vec![nearby]),
                _ => NearbyChildren::InFront(vec![nearby]),
            },
            NearbyChildren::Behind(b) => match loc {
                Location::Behind => {
                    let mut nearby = vec![nearby];
                    nearby.extend(b);
                    NearbyChildren::Behind(nearby)
                }
                _ => NearbyChildren::Both(NearbyChildrenBoth {
                    behind: b,
                    in_front: vec![nearby],
                }),
            },
            NearbyChildren::InFront(f) => match loc {
                Location::Behind => NearbyChildren::Both(NearbyChildrenBoth {
                    behind: vec![nearby],
                    in_front: f,
                }),
                _ => {
                    let mut nearby = vec![nearby];
                    nearby.extend(f);
                    NearbyChildren::InFront(nearby)
                }
            },
            NearbyChildren::Both(NearbyChildrenBoth { behind, in_front }) => {
                match loc {
                    Location::Behind => {
                        let mut nearby = vec![nearby];
                        nearby.extend(behind);
                        NearbyChildren::Both(NearbyChildrenBoth {
                            behind: nearby,
                            in_front,
                        })
                    }
                    _ => {
                        let mut nearby = vec![nearby];
                        nearby.extend(in_front);
                        NearbyChildren::Both(NearbyChildrenBoth {
                            behind,
                            in_front: nearby,
                        })
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct NearbyChildrenBoth {
    behind: Vec<Node>,
    in_front: Vec<Node>,
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Gathered {
    node: NodeName,
    attrs: Vec<vdom::Attribute>,
    styles: Vec<Style>,
    children: NearbyChildren,
    has: Field,
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct GridTemplate {
    spacing: (Length, Length),
    columns: Vec<Length>,
    rows: Vec<Length>,
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct GridPosition {
    row: u64,
    col: u64,
    width: u64,
    height: u64,
}

#[derive(Debug, Clone)]
pub enum Children<C> {
    Unkeyed(Vec<C>),
    Keyed(Vec<(String, C)>),
}

// #[derive(Debug, Clone)]
// pub enum Child {
//     Element(Element),
//     Node(Node),
// }

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum HoverSetting {
    No,
    Allow,
    Force,
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct FocusStyle {
    border_color: Option<Color>,
    shadow: Option<Shadow>,
    bg_color: Option<Color>,
}

impl Default for FocusStyle {
    fn default() -> Self {
        Self {
            bg_color: None,
            border_color: None,
            shadow: Some(Shadow {
                color: Color {
                    r: (155.0 / 255.0),
                    g: (203.0 / 255.0),
                    b: 1.0,
                    a: 1.0,
                },
                offset: (0, 0),
                blur: 0,
                size: 3,
            }),
        }
    }
}

impl FocusStyle {
    pub fn render(&self) -> Vec<Style> {
        vec![Style::Style(
        format!(".{}:focus-within", Classes::FocusedWithin.to_string()),
        vec![
            self.border_color.map(|color| {
                Property("border-color".to_string(), color.format_color())
            }),
            self.bg_color.map(|color| {
                Property("background-color".to_string(), color.format_color())
            }),
            self.shadow.map(|shadow| {
                Property(
                    "box-shadow".to_string(),
                    shadow.format_box_shadow(false),
                )
            }),
            Some(Property("outline".to_string(), "none".to_string())),
        ]
        .into_iter()
        .filter_map(|x| x)
        .collect::<Vec<Property>>(),
    ), Style::Style(
        ".s:focus .focusable, .s.focusable:focus, .ui-slide-bar:focus + .s.focusable-thumb".to_string(),
        vec![
            self.border_color.map(|color| {
                Property("border-color".to_string(), color.format_color())
            }),
            self.bg_color.map(|color| {
                Property("background-color".to_string(), color.format_color())
            }),
            self.shadow.map(|shadow| {
                Property(
                    "box-shadow".to_string(),
                    shadow.format_box_shadow(false),
                )
            }),
            Some(Property("outline".to_string(), "none".to_string())),
        ]
        .into_iter()
        .filter_map(|x| x)
        .collect::<Vec<Property>>(),
    )]
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum RenderMode {
    Layout,
    NoStaicStyleSheet,
    WithVirtualCSS,
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum Opt {
    Hover(HoverSetting),
    Focus(FocusStyle),
    Render(RenderMode),
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct OptStruct {
    hover: HoverSetting,
    focus: FocusStyle,
    mode: RenderMode,
}

impl Default for OptStruct {
    fn default() -> Self {
        Self {
            hover: HoverSetting::Allow,
            focus: FocusStyle::default(),
            mode: RenderMode::Layout,
        }
    }
}

impl OptStruct {
    pub fn from_opts(opts: Vec<Opt>) -> Self {
        let combine = |mut strct: (
            Option<HoverSetting>,
            Option<FocusStyle>,
            Option<RenderMode>,
        ),
                       opt: &Opt| match opt {
            Opt::Hover(_) => {
                if let None = strct.0 {
                    strct.0 = Some(HoverSetting::Allow);
                    strct
                } else {
                    strct
                }
            }
            Opt::Focus(_) => {
                if let None = strct.1 {
                    strct.1 = Some(FocusStyle::default());
                    strct
                } else {
                    strct
                }
            }
            Opt::Render(_) => {
                if let None = strct.2 {
                    strct.2 = Some(RenderMode::Layout);
                    strct
                } else {
                    strct
                }
            }
        };
        let and_finally = |strct: (
            Option<HoverSetting>,
            Option<FocusStyle>,
            Option<RenderMode>,
        )| {
            OptStruct {
                hover: if let Some(h) = strct.0 {
                    h
                } else {
                    HoverSetting::Allow
                },
                focus: if let Some(f) = strct.1 {
                    f
                } else {
                    FocusStyle::default()
                },
                mode: if let Some(m) = strct.2 {
                    m
                } else {
                    RenderMode::Layout
                },
            }
        };
        and_finally(opts.iter().rev().fold((None, None, None), combine))
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct Shadow {
    color: Color,
    offset: (u8, u8),
    blur: u8,
    size: u8,
}

impl Shadow {
    pub fn format_drop_shadow(&self) -> String {
        let offset = self.offset;
        let blur = self.blur;
        format!(
            "{}px {}px {}px {}",
            offset.0.to_string(),
            offset.1.to_string(),
            blur.to_string(),
            self.color.format_color(),
        )
    }
    pub fn format_text_shadow(&self) -> String {
        let offset = self.offset;
        let blur = self.blur;
        format!(
            "{}px {}px {}px {}",
            offset.0.to_string(),
            offset.1.to_string(),
            blur.to_string(),
            self.color.format_color(),
        )
    }
    pub fn text_shadow_class(&self) -> String {
        let offset = self.offset;
        let blur = self.blur;
        format!(
            "txt{}px{}px{}px{}",
            (offset.0 as f32).float_class(),
            (offset.1 as f32).float_class(),
            (blur as f32).float_class(),
            self.color.format_color(),
        )
    }
    pub fn format_box_shadow(&self, inset: bool) -> String {
        let offset = self.offset;
        let blur = self.blur;
        if inset {
            format!(
                "inset {}px {}px {}px {}px {}",
                offset.0.to_string(),
                offset.1.to_string(),
                blur.to_string(),
                self.size.to_string(),
                self.color.format_color(),
            )
        } else {
            format!(
                "{}px {}px {}px {}px {}",
                offset.0.to_string(),
                offset.1.to_string(),
                blur.to_string(),
                self.size.to_string(),
                self.color.format_color(),
            )
        }
    }
    pub fn box_shadow_class(&self, inset: bool) -> String {
        let offset = self.offset;
        let blur = self.blur;
        let size = self.size;
        let inset = if inset { "box-inset" } else { "box-" };
        format!(
            "{}{}px{}px{}px{}px{}",
            inset,
            (offset.0 as f32).float_class(),
            (offset.1 as f32).float_class(),
            (blur as f32).float_class(),
            (size as f32).float_class(),
            self.color.format_color(),
        )
    }
}

// unstyled : VirtualDom.Node msg -> Element msg
// unstyled =
//     Unstyled << always

pub fn finalize_node(
    has: Field,
    node: NodeName,
    attributes: Vec<vdom::Attribute>,
    children: Children<Node>,
    embed_mode: EmbedStyle,
    parent_ctx: LayoutContext,
) -> Node {
    let create_node =
        |node_name: String, attrs: Vec<vdom::Attribute>| match children {
            Children::Keyed(keyed) => {
                let keyed = match embed_mode {
                    EmbedStyle::NoStyleSheet => keyed,
                    EmbedStyle::OnlyDynamic(opts, styles) => {
                        embed_keyed(false, opts, &styles, keyed)
                    }
                    EmbedStyle::StaticRootAndynamic(opts, styles) => {
                        embed_keyed(true, opts, &styles, keyed)
                    }
                };
                vdom::keyed_node(
                    node_name.clone(),
                    node_name.clone(),
                    attrs,
                    keyed
                        .into_iter()
                        .map(|(s, n)| NodeType::KeyedNode(s, n))
                        .collect(),
                )
            }
            Children::Unkeyed(unkeyed) => {
                let unkeyed = unkeyed
                    .into_iter()
                    .map(|n| NodeType::Node(n))
                    .collect::<Vec<NodeType>>();
                let children = match embed_mode {
                    EmbedStyle::NoStyleSheet => unkeyed,
                    EmbedStyle::OnlyDynamic(opts, styles) => {
                        embed_with(false, opts, styles, unkeyed)
                    }
                    EmbedStyle::StaticRootAndynamic(opts, styles) => {
                        embed_with(true, opts, styles, unkeyed)
                    }
                };

                let node = match &node_name[..] {
                    "div" => html::div(attrs, children),
                    "p" => html::p(attrs, children),
                    _ => vdom::node(node_name, attrs, children),
                };
                NodeType::Node(node)
            }
        };

    let html = match node {
        NodeName::Generic => create_node("div".to_string(), attributes),
        NodeName::NodeName(name) => create_node(name, attributes),
        NodeName::Embedded(name, internal) => NodeType::Node(vdom::node(
            name,
            attributes,
            vec![create_node(
                internal,
                vec![attributes::class(format!(
                    "s {}",
                    Classes::Single.to_string()
                ))],
            )],
        )),
    };

    match parent_ctx {
        LayoutContext::AsRow => {
            if has.present(&Flag::width_fill())
                && !has.present(&Flag::width_between())
            {
                vdom::node("div".to_string(), vec![], vec![html])
            } else if has.present(&Flag::align_right()) {
                html::u(
                    vec![attributes::class(format!(
                        "s {} {} {} {}",
                        Classes::Single.to_string(),
                        Classes::Container.to_string(),
                        Classes::ContentCenterY.to_string(),
                        Classes::AlignContainerRight.to_string(),
                    ))],
                    vec![html],
                )
            } else if has.present(&Flag::center_x()) {
                html::s(
                    vec![attributes::class(format!(
                        "s {} {} {} {}",
                        Classes::Single.to_string(),
                        Classes::Container.to_string(),
                        Classes::ContentCenterY.to_string(),
                        Classes::AlignContainerCenterX.to_string(),
                    ))],
                    vec![html],
                )
            } else {
                vdom::node("div".to_string(), vec![], vec![html])
            }
        }
        LayoutContext::AsColumn => {
            if has.present(&Flag::height_fill())
                && !has.present(&Flag::height_between())
            {
                vdom::node("div".to_string(), vec![], vec![html])
            } else if has.present(&Flag::center_y()) {
                html::u(
                    vec![attributes::class(format!(
                        "s {} {} {} {}",
                        Classes::Single.to_string(),
                        Classes::Container.to_string(),
                        Classes::ContentCenterY.to_string(),
                        Classes::AlignContainerCenterY.to_string(),
                    ))],
                    vec![html],
                )
            } else if has.present(&Flag::align_bottom()) {
                html::s(
                    vec![attributes::class(format!(
                        "s {} {} {} {}",
                        Classes::Single.to_string(),
                        Classes::Container.to_string(),
                        Classes::ContentCenterY.to_string(),
                        Classes::AlignContainerBottom.to_string(),
                    ))],
                    vec![html],
                )
            } else {
                vdom::node("div".to_string(), vec![], vec![html])
            }
        }
        _ => vdom::node("div".to_string(), vec![], vec![html]),
    }
}

pub fn embed_with(
    is_static: bool,
    opts: OptStruct,
    styles: Vec<Style>,
    children: Vec<NodeType>,
) -> Vec<NodeType> {
    let style_sheet = styles
        .iter()
        .fold(
            (HashSet::new(), opts.focus.render()),
            |(cache, existing), style| reduce_styles(style, cache, existing),
        )
        .1;

    let dynamic_style_sheet = NodeType::Node(to_stylesheet(opts, style_sheet));

    if is_static {
        let mut res =
            vec![NodeType::Node(static_root(opts)), dynamic_style_sheet];
        res.extend(children);
        res
    } else {
        let mut res = vec![dynamic_style_sheet];
        res.extend(children);
        res
    }
}

pub fn embed_keyed(
    is_static: bool,
    opts: OptStruct,
    styles: &Vec<Style>,
    children: Vec<(String, Node)>,
) -> Vec<(String, Node)> {
    let style_sheet = styles
        .iter()
        .fold(
            (HashSet::new(), opts.focus.render()),
            |(cache, existing), style| reduce_styles(style, cache, existing),
        )
        .1;

    let dynamic_style_sheet = to_stylesheet(opts, style_sheet);

    if is_static {
        let mut res = vec![
            (String::from("static-stylesheet"), static_root(opts)),
            (String::from("dynamic-stylesheet"), dynamic_style_sheet),
        ];
        res.extend(children);
        res
    } else {
        let mut res =
            vec![(String::from("dynamic-stylesheet"), dynamic_style_sheet)];
        res.extend(children);
        res
    }
}

pub fn reduce_styles_recursive(
    mut cache: HashSet<String>,
    mut found: Vec<Style>,
    styles: Vec<Style>,
) -> Vec<Style> {
    match &styles[..] {
        [] => found,
        [l @ .., last] => {
            let style_name = styles[0].name();
            if let Some(_) = cache.get(&style_name) {
                reduce_styles_recursive(cache, found, l.to_vec())
            } else {
                cache.insert(style_name);
                found.push(last.clone());
                reduce_styles_recursive(cache, found, l.to_vec())
            }
        }
    }
}

pub fn reduce_styles(
    style: &Style,
    mut cache: HashSet<String>,
    existing: Vec<Style>,
) -> (HashSet<String>, Vec<Style>) {
    let name = style.name();
    if let Some(_) = cache.get(&name) {
        (cache, existing)
    } else {
        cache.insert(name);
        let mut styles = vec![style.clone()];
        styles.extend(existing);
        (cache, styles)
    }
}

pub fn reduce_recursive(
    found: Vec<Style>,
    styles: Vec<(String, Style)>,
) -> Vec<Style> {
    match &styles[..] {
        [] => found,
        [(_, style)] => {
            let mut ret = vec![style.clone()];
            ret.extend(found);
            ret
        }
        [(name1, style1), next, remaining @ ..] => {
            if name1 != &next.0 {
                let mut l1 = vec![style1.clone()];
                l1.extend(found);
                let mut l2 = vec![next.clone()];
                l2.extend(remaining.to_vec());
                reduce_recursive(l1, l2)
            } else {
                let mut l = vec![next.clone()];
                l.extend(remaining.to_vec());
                reduce_recursive(found, l)
            }
        }
    }
}

pub fn skippable(flag: &Flag, style: &Style) -> bool {
    if flag == &Flag::border_width() {
        match style {
            Style::Single(_, _, val) => match &val[..] {
                "0px" => true,
                "1px" => true,
                "2px" => true,
                "3px" => true,
                "4px" => true,
                "5px" => true,
                "6px" => true,
                _ => false,
            },
            _ => false,
        }
    } else {
        match style {
            Style::FontSize(i) => i >= &8 && i <= &32,
            Style::Padding(_, t, r, b, l) => {
                t == b && t == r && t == l && t >= &0.0 && t <= &24.0
            }
            // Style::Spacing(_ _ _) => true,
            // Style::FontFamily(_ _) => true,
            _ => false,
        }
    }
}

pub fn gather_attr_recursive(
    classes: String,
    node: NodeName,
    mut has: Field,
    transform: Transform,
    mut styles: Vec<Style>,
    attrs: Vec<vdom::Attribute>,
    mut children: NearbyChildren,
    element_attrs: Vec<Attribute>,
) -> Gathered {
    use attributes::class;
    match &element_attrs[..] {
        [] => match transform.class() {
            None => {
                let mut classes = vec![class(classes)];
                classes.extend(attrs);
                Gathered {
                    attrs: classes,
                    styles,
                    node,
                    children,
                    has,
                }
            }
            Some(cls) => {
                let classes = format!("{} {}", classes, cls);
                let mut classes = vec![class(classes)];
                classes.extend(attrs);
                let mut transform = vec![Style::Transform(transform)];
                transform.extend(styles);
                Gathered {
                    attrs: classes,
                    styles: transform,
                    node,
                    children,
                    has,
                }
            }
        },
        [attribute, remaining @ ..] => match attribute {
            Attribute::None => gather_attr_recursive(
                classes,
                node,
                has,
                transform,
                styles,
                attrs,
                children,
                remaining.to_vec(),
            ),
            Attribute::Class(flag, exact_class_name) => {
                if has.present(flag) {
                    gather_attr_recursive(
                        classes,
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                } else {
                    let classes = format!("{} {}", exact_class_name, classes);
                    has.add(flag);
                    gather_attr_recursive(
                        classes,
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                }
            }
            Attribute::Attr(actual_attribute) => {
                let mut att = vec![actual_attribute.clone()];
                att.extend(attrs);
                let attrs = att;
                gather_attr_recursive(
                    classes,
                    node,
                    has,
                    transform,
                    styles,
                    attrs,
                    children,
                    remaining.to_vec(),
                )
            }
            Attribute::Style(flag, style) => {
                if has.present(flag) {
                    gather_attr_recursive(
                        classes,
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                } else if skippable(flag, style) {
                    has.add(flag);

                    gather_attr_recursive(
                        format!("{} {}", style.name(), classes),
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                } else {
                    let style_name = style.name();
                    let mut style = vec![style.clone()];
                    style.extend(styles);
                    let styles = style;
                    has.add(flag);
                    gather_attr_recursive(
                        format!("{} {}", style_name, classes),
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                }
            }
            Attribute::TransformComponent(flag, component) => {
                let transform = transform.compose(component);
                has.add(flag);
                gather_attr_recursive(
                    classes,
                    node,
                    has,
                    transform,
                    styles,
                    attrs,
                    children,
                    remaining.to_vec(),
                )
            }
            Attribute::Width(width) => {
                if has.present(&Flag::width()) {
                    gather_attr_recursive(
                        classes,
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                } else {
                    match width {
                        Length::Px(px) => {
                            let classes = format!(
                                "{} width-px-{} {}",
                                Classes::WidthExact.to_string(),
                                px,
                                classes
                            );
                            has.add(&Flag::width());
                            let mut style = vec![Style::Single(
                                format!("width-px-{}", px),
                                String::from("width"),
                                format!("{}px", px),
                            )];
                            style.extend(styles);
                            let styles = style;
                            gather_attr_recursive(
                                classes,
                                node,
                                has,
                                transform,
                                styles,
                                attrs,
                                children,
                                remaining.to_vec(),
                            )
                        }
                        Length::Content => {
                            has.add(&Flag::width());
                            has.add(&Flag::width_content());

                            gather_attr_recursive(
                                format!(
                                    "{} {}",
                                    classes,
                                    Classes::WidthContent.to_string()
                                ),
                                node,
                                has,
                                transform,
                                styles,
                                attrs,
                                children,
                                remaining.to_vec(),
                            )
                        }
                        Length::Fill(portion) => {
                            if *portion == 1 {
                                has.add(&Flag::width());
                                has.add(&Flag::width_fill());
                                gather_attr_recursive(
                                    format!(
                                        "{} {}",
                                        classes,
                                        Classes::WidthFill.to_string()
                                    ),
                                    node,
                                    has,
                                    transform,
                                    styles,
                                    attrs,
                                    children,
                                    remaining.to_vec(),
                                )
                            } else {
                                has.add(&Flag::width());
                                has.add(&Flag::width_fill());
                                let classes = format!(
                                    "{} {} width-fill-{}",
                                    classes,
                                    Classes::WidthFillPortion.to_string(),
                                    portion
                                );
                                let mut style = vec![Style::Single(
                                    format!(
                                        "{}.{} > .width-fill{}",
                                        Classes::Any.to_string(),
                                        Classes::Row.to_string(),
                                        portion,
                                    ),
                                    String::from("flex-grow"),
                                    (portion * 100000).to_string(),
                                )];
                                style.extend(styles);
                                let styles = style;
                                gather_attr_recursive(
                                    classes,
                                    node,
                                    has,
                                    transform,
                                    styles,
                                    attrs,
                                    children,
                                    remaining.to_vec(),
                                )
                            }
                        }
                        _ => {
                            let (add_to_flags, new_class, mut new_styles) =
                                width.render_width();
                            let classes = format!("{} {}", classes, new_class);
                            new_styles.extend(styles);
                            has.add(&Flag::width());
                            has.merge(add_to_flags);
                            gather_attr_recursive(
                                classes,
                                node,
                                has,
                                transform,
                                new_styles,
                                attrs,
                                children,
                                remaining.to_vec(),
                            )
                        }
                    }
                }
            }
            Attribute::Height(height) => {
                if has.present(&Flag::height()) {
                    gather_attr_recursive(
                        classes,
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                } else {
                    match height {
                        Length::Px(px) => {
                            let classes = format!(
                                "{} height-px-{}px {}",
                                Classes::HeightExact.to_string(),
                                px,
                                classes
                            );
                            has.add(&Flag::height());
                            let mut style = vec![Style::Single(
                                format!("height-px-{}", px),
                                String::from("height"),
                                format!("{}px", px),
                            )];
                            style.extend(styles);
                            let styles = style;
                            gather_attr_recursive(
                                classes,
                                node,
                                has,
                                transform,
                                styles,
                                attrs,
                                children,
                                remaining.to_vec(),
                            )
                        }
                        Length::Content => {
                            has.add(&Flag::height());
                            has.add(&Flag::height_content());

                            gather_attr_recursive(
                                format!(
                                    "{} {}",
                                    Classes::HeightContent.to_string(),
                                    classes
                                ),
                                node,
                                has,
                                transform,
                                styles,
                                attrs,
                                children,
                                remaining.to_vec(),
                            )
                        }
                        Length::Fill(portion) => {
                            if *portion == 1 {
                                has.add(&Flag::height());
                                has.add(&Flag::height_fill());
                                gather_attr_recursive(
                                    format!(
                                        "{} {}",
                                        Classes::HeightFill.to_string(),
                                        classes
                                    ),
                                    node,
                                    has,
                                    transform,
                                    styles,
                                    attrs,
                                    children,
                                    remaining.to_vec(),
                                )
                            } else {
                                has.add(&Flag::height());
                                has.add(&Flag::height_fill());
                                let classes = format!(
                                    "{} {} height-fill-{}",
                                    classes,
                                    Classes::HeightFillPortion.to_string(),
                                    portion
                                );
                                let mut style = vec![Style::Single(
                                    format!(
                                        "{}.{} > .height-fill{}",
                                        Classes::Any.to_string(),
                                        Classes::Column.to_string(),
                                        portion,
                                    ),
                                    String::from("flex-grow"),
                                    (portion * 100000).to_string(),
                                )];
                                style.extend(styles);
                                let styles = style;
                                gather_attr_recursive(
                                    classes,
                                    node,
                                    has,
                                    transform,
                                    styles,
                                    attrs,
                                    children,
                                    remaining.to_vec(),
                                )
                            }
                        }
                        _ => {
                            let (add_to_flags, new_class, mut new_styles) =
                                height.render_height();
                            let classes = format!("{} {}", classes, new_class);
                            new_styles.extend(styles);
                            has.add(&Flag::height());
                            has.merge(add_to_flags);
                            gather_attr_recursive(
                                classes,
                                node,
                                has,
                                transform,
                                new_styles,
                                attrs,
                                children,
                                remaining.to_vec(),
                            )
                        }
                    }
                }
            }
            Attribute::Describe(description) => match description {
                Description::Main => {
                    let node = node.add("main".to_string());
                    gather_attr_recursive(
                        classes,
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                }
                Description::Navigation => {
                    let node = node.add("nav".to_string());
                    gather_attr_recursive(
                        classes,
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                }

                Description::ContentInfo => {
                    let node = node.add("footer".to_string());
                    gather_attr_recursive(
                        classes,
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                }
                Description::Complementary => {
                    let node = node.add("aside".to_string());
                    gather_attr_recursive(
                        classes,
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                }
                Description::Heading(i) => {
                    if *i <= 1 {
                        let node = node.add("h1".to_string());
                        gather_attr_recursive(
                            classes,
                            node,
                            has,
                            transform,
                            styles,
                            attrs,
                            children,
                            remaining.to_vec(),
                        )
                    } else if *i < 7 {
                        let node = node.add(format!("h{}", i));
                        gather_attr_recursive(
                            classes,
                            node,
                            has,
                            transform,
                            styles,
                            attrs,
                            children,
                            remaining.to_vec(),
                        )
                    } else {
                        let node = node.add("h6".to_string());
                        gather_attr_recursive(
                            classes,
                            node,
                            has,
                            transform,
                            styles,
                            attrs,
                            children,
                            remaining.to_vec(),
                        )
                    }
                }
                Description::Paragraph => {
                    // previously we rendered a <p> tag, though apparently
                    // this invalidates the html if it has <div>s inside.
                    // Since we can't guaranteee that there are no divs,
                    // we need another strategy.
                    // While it's not documented in many places,
                    // there apparently is a paragraph aria role
                    // https://github.com/w3c/aria/blob/11f85f41a5b621fdbe85fc9bcdcd270e653a48ba/common/script/roleInfo.js
                    // Though we'll need to wait till it gets released
                    // in an official wai-aria spec to use it.
                    // If it's used at the moment, then Lighthouse
                    // complains (likely rightfully) that role paragraph
                    // is not recognized.
                    gather_attr_recursive(
                        classes,
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                }
                Description::Button => {
                    let mut att =
                        vec![vdom::Attribute("role=button".to_string())];
                    att.extend(attrs);
                    let attrs = att;
                    gather_attr_recursive(
                        classes,
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                }

                Description::Label(label) => {
                    let mut att =
                        vec![vdom::Attribute(format!("aria-label={}", label))];
                    att.extend(attrs);
                    let attrs = att;
                    gather_attr_recursive(
                        classes,
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                }
                Description::LivePolite => {
                    let mut att =
                        vec![vdom::Attribute("aria-live=polite".to_string())];
                    att.extend(attrs);
                    let attrs = att;
                    gather_attr_recursive(
                        classes,
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                }
                Description::LiveAssertive => {
                    let mut att =
                        vec![vdom::Attribute("aria-live=polite".to_string())];
                    att.extend(attrs);
                    let attrs = att;
                    gather_attr_recursive(
                        classes,
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                }
            },
            Attribute::Nearby(loc, el) => {
                let styles = match el {
                    Element::Styled(styled) => {
                        styles.extend(styled.styles.clone());
                        styles
                    }
                    _ => styles,
                };
                let children = children.add_nearby_el(loc, el);
                gather_attr_recursive(
                    classes,
                    node,
                    has,
                    transform,
                    styles,
                    attrs,
                    children,
                    remaining.to_vec(),
                )
            }
            Attribute::AlignX(x) => {
                if has.present(&Flag::align_x()) {
                    gather_attr_recursive(
                        classes,
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                } else {
                    has.add(&Flag::align_x());
                    match x {
                        HAlign::CenterX => has.add(&Flag::center_x()),
                        HAlign::Right => has.add(&Flag::align_right()),
                        HAlign::Left => (),
                    }
                    gather_attr_recursive(
                        format!("{} {}", x.name(), classes),
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                }
            }
            Attribute::AlignY(y) => {
                if has.present(&Flag::align_y()) {
                    gather_attr_recursive(
                        classes,
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                } else {
                    has.add(&Flag::align_y());
                    match y {
                        VAlign::CenterY => has.add(&Flag::center_y()),
                        VAlign::Bottom => has.add(&Flag::align_bottom()),
                        VAlign::Top => (),
                    }
                    gather_attr_recursive(
                        format!("{} {}", y.name(), classes),
                        node,
                        has,
                        transform,
                        styles,
                        attrs,
                        children,
                        remaining.to_vec(),
                    )
                }
            }
        },
    }
}

pub fn nearby_el(loc: &Location, el: &Element) -> Node {
    let attrs = match loc {
        Location::Above => format!(
            "{} {} {}",
            Classes::Nearby.to_string(),
            Classes::Single.to_string(),
            Classes::Above.to_string(),
        ),
        Location::Below => format!(
            "{} {} {}",
            Classes::Nearby.to_string(),
            Classes::Single.to_string(),
            Classes::Below.to_string(),
        ),
        Location::OnRight => format!(
            "{} {} {}",
            Classes::Nearby.to_string(),
            Classes::Single.to_string(),
            Classes::OnRight.to_string(),
        ),
        Location::OnLeft => format!(
            "{} {} {}",
            Classes::Nearby.to_string(),
            Classes::Single.to_string(),
            Classes::OnLeft.to_string(),
        ),
        Location::InFront => format!(
            "{} {} {}",
            Classes::Nearby.to_string(),
            Classes::Single.to_string(),
            Classes::InFront.to_string(),
        ),
        Location::Behind => format!(
            "{} {} {}",
            Classes::Nearby.to_string(),
            Classes::Single.to_string(),
            Classes::Behind.to_string(),
        ),
    };
    let attrs = vec![html::attributes::class(attrs)];
    let items = match el {
        Element::Empty => text_element(&"".to_string()),
        Element::Text(s) => text_element(s),
        Element::Unstyled(FinalizeNodeArgs {
            has,
            node,
            attributes,
            children,
            embed_mode,
        }) => finalize_node(
            has.clone(),
            node.clone(),
            attributes.clone(),
            children.clone(),
            embed_mode.clone().unwrap(),
            LayoutContext::AsEl,
        ),
        Element::Styled(Styled {
            styles,
            html:
                FinalizeNodeArgs {
                    has,
                    node,
                    attributes,
                    children,
                    embed_mode,
                },
        }) => finalize_node(
            has.clone(),
            node.clone(),
            attributes.clone(),
            children.clone(),
            EmbedStyle::NoStyleSheet,
            LayoutContext::AsEl,
        ),
    };
    let items = vec![NodeType::Node(items)];
    html::div(attrs, items)
}

pub fn row_class() -> String {
    format!("{} {}", Classes::Any.to_string(), Classes::Row.to_string())
}

pub fn column_class() -> String {
    format!(
        "{} {}",
        Classes::Any.to_string(),
        Classes::Column.to_string()
    )
}

pub fn single_class() -> String {
    format!(
        "{} {}",
        Classes::Any.to_string(),
        Classes::Single.to_string()
    )
}
pub fn grid_class() -> String {
    format!("{} {}", Classes::Any.to_string(), Classes::Grid.to_string())
}
pub fn paragraph_class() -> String {
    format!(
        "{} {}",
        Classes::Any.to_string(),
        Classes::Paragraph.to_string()
    )
}
pub fn page_class() -> String {
    format!("{} {}", Classes::Any.to_string(), Classes::Page.to_string())
}

pub fn context_classes(context: &LayoutContext) -> String {
    match context {
        LayoutContext::AsRow => row_class(),
        LayoutContext::AsColumn => column_class(),
        LayoutContext::AsEl => single_class(),
        LayoutContext::AsGrid => grid_class(),
        LayoutContext::AsParagraph => paragraph_class(),
        LayoutContext::AsTextColumn => page_class(),
    }
}

pub fn element(
    context: LayoutContext,
    node: NodeName,
    mut attrs: Vec<Attribute>,
    children: Children<Element>,
) -> Element {
    attrs.reverse();
    let rendered = gather_attr_recursive(
        context_classes(&context),
        node,
        Field::none(),
        untransformed(),
        vec![],
        vec![],
        NearbyChildren::None,
        attrs,
    );
    create_element(context, children, rendered)
}

pub fn untransformed() -> Transform {
    Transform::Untransformed
}

pub fn create_element(
    context: LayoutContext,
    children: Children<Element>,
    mut rendered: Gathered,
) -> Element {
    let gather = |content: &mut (Vec<Node>, Vec<Style>),
                  child: &mut Element| {
        let (html, mut existing_styles) = content.to_owned();
        // let html = html
        //     .into_iter()
        //     .map(|n| NodeType::Node(n))
        //     .collect::<Vec<NodeType>>();
        match child.clone() {
            Element::Unstyled(FinalizeNodeArgs {
                has,
                node,
                attributes,
                children,
                embed_mode,
            }) => {
                let mut nodes = vec![finalize_node(
                    has,
                    node,
                    attributes,
                    children,
                    embed_mode.unwrap(),
                    context,
                )];
                nodes.extend(html);
                (nodes, existing_styles)
            }
            Element::Styled(Styled {
                mut styles,
                html:
                    FinalizeNodeArgs {
                        has,
                        node,
                        attributes,
                        children,
                        embed_mode,
                    },
            }) => {
                let mut nodes = vec![finalize_node(
                    has,
                    node,
                    attributes,
                    children,
                    EmbedStyle::NoStyleSheet,
                    context,
                )];
                nodes.extend(html);
                let new_styles = if existing_styles.is_empty() {
                    styles
                } else {
                    styles.extend(existing_styles);
                    styles
                };
                (nodes, new_styles)
            }
            Element::Text(txt) => {
                // TEXT OPTIMIZATION
                // You can have raw text if the element is an el,
                // and has `width-content` and `height-content`
                // Same if it's a column or row with one child and
                // width-content, height-content interferes with css grid
                // Maybe we could unpack text elements in a paragraph as
                // well, however, embedded elements that are larger than
                // the line height will overlap with exisitng text.
                // I don't think that's what we want.
                // if
                //     context
                //         == asEl
                //         || context
                //         == asParagraph
                // then
                //     ( VirtualDom.text
                //         (if context == asParagraph then
                //             str
                //          else
                //             str
                //         )
                //         :: htmls
                //     , existingStyles
                //     )
                // else
                let mut h = if context == LayoutContext::AsEl {
                    vec![text_element_fill(&txt)]
                } else {
                    vec![text_element(&txt)]
                };
                h.extend(html);
                (h, existing_styles)
            }
            Element::Empty => (html, existing_styles),
        }
    };
    let gather_keyed = |content: &mut (Vec<(String, Node)>, Vec<Style>),
                        keyed: &mut (String, Element)| {
        let (html, mut existing_styles) = content.to_owned();
        let (key, child) = keyed.to_owned();
        // let html = html
        //     .into_iter()
        //     .map(|(s, n)| NodeType::KeyedNode(s, n))
        //     .collect::<Vec<NodeType>>();
        match child {
            Element::Unstyled(FinalizeNodeArgs {
                has,
                node,
                attributes,
                children,
                embed_mode,
            }) => {
                let mut nodes = vec![(
                    key,
                    finalize_node(
                        has,
                        node,
                        attributes,
                        children,
                        embed_mode.unwrap(),
                        context,
                    ),
                )];
                nodes.extend(html);
                (nodes, existing_styles)
            }
            Element::Styled(Styled {
                mut styles,
                html:
                    FinalizeNodeArgs {
                        has,
                        node,
                        attributes,
                        children,
                        embed_mode,
                    },
            }) => {
                let mut nodes = vec![(
                    key,
                    finalize_node(
                        has,
                        node,
                        attributes,
                        children,
                        EmbedStyle::NoStyleSheet,
                        context,
                    ),
                )];
                nodes.extend(html);
                let new_styles = if existing_styles.is_empty() {
                    styles
                } else {
                    styles.extend(existing_styles);
                    styles
                };
                (nodes, new_styles)
            }
            Element::Text(txt) => {
                // TEXT OPTIMIZATION
                // You can have raw text if the element is an el,
                // and has `width-content` and `height-content`
                // Same if it's a column or row with one child and
                // width-content, height-content interferes with css grid
                // Maybe we could unpack text elements in a paragraph as
                // well, however, embedded elements that are larger than
                // the line height will overlap with exisitng text.
                // I don't think that's what we want.
                // if
                //     context
                //         == asEl
                //         || context
                //         == asParagraph
                // then
                //     ( ( key
                //       , VirtualDom.text
                //             str
                //       )
                //         :: htmls
                //     , existingStyles
                //     )
                // else
                let mut h = if context == LayoutContext::AsEl {
                    vec![(key, text_element_fill(&txt))]
                } else {
                    vec![(key, text_element(&txt))]
                };
                h.extend(html);
                (h, existing_styles)
            }
            Element::Empty => (html, existing_styles),
        }
    };
    match children {
        Children::Keyed(c) => {
            let (keyed, styles) = c
                .into_iter()
                .rev()
                .fold((vec![], vec![]), |mut content, mut keyed| {
                    gather_keyed(&mut content, &mut keyed)
                });
            let new_styles = if styles.is_empty() {
                rendered.styles
            } else {
                rendered.styles.extend(styles);
                rendered.styles
            };
            if new_styles.is_empty() {
                let ck = Children::Keyed::<Node>(add_keyed_children(
                    String::from("nearby-element-pls"),
                    keyed,
                    rendered.children,
                ));
                Element::Unstyled(FinalizeNodeArgs {
                    has: rendered.has,
                    node: rendered.node,
                    attributes: rendered.attrs,
                    children: ck,
                    embed_mode: Some(EmbedStyle::NoStyleSheet),
                })
            } else {
                let ck = Children::Keyed::<Node>(add_keyed_children(
                    String::from("nearby-element-pls"),
                    keyed,
                    rendered.children,
                ));
                Element::Styled(Styled {
                    styles: new_styles,
                    html: FinalizeNodeArgs {
                        has: rendered.has,
                        node: rendered.node,
                        attributes: rendered.attrs,
                        children: ck,
                        embed_mode: None,
                    },
                })
            }
        }
        Children::Unkeyed(c) => {
            let (unkeyed, styles) = c
                .into_iter()
                .rev()
                .fold((vec![], vec![]), |mut content, mut child| {
                    gather(&mut content, &mut child)
                });
            let new_styles = if styles.is_empty() {
                rendered.styles
            } else {
                rendered.styles.extend(styles);
                rendered.styles
            };
            if new_styles.is_empty() {
                let ck = Children::Unkeyed::<Node>(add_children(
                    unkeyed,
                    rendered.children,
                ));
                Element::Unstyled(FinalizeNodeArgs {
                    has: rendered.has,
                    node: rendered.node,
                    attributes: rendered.attrs,
                    children: ck,
                    embed_mode: Some(EmbedStyle::NoStyleSheet),
                })
            } else {
                let ck = Children::Unkeyed::<Node>(add_children(
                    unkeyed,
                    rendered.children,
                ));

                Element::Styled(Styled {
                    styles: new_styles,
                    html: FinalizeNodeArgs {
                        has: rendered.has,
                        node: rendered.node,
                        attributes: rendered.attrs,
                        children: ck,
                        embed_mode: None,
                    },
                })
            }
        }
    }
}

pub fn add_children(mut existing: Vec<Node>, nc: NearbyChildren) -> Vec<Node> {
    match nc {
        NearbyChildren::None => existing,
        NearbyChildren::Behind(mut b) => {
            b.extend(existing);
            b
        }
        NearbyChildren::InFront(f) => {
            existing.extend(f);
            existing
        }
        NearbyChildren::Both(NearbyChildrenBoth {
            mut behind,
            in_front,
        }) => {
            behind.extend(existing);
            behind.extend(in_front);
            behind
        }
    }
}

pub fn add_keyed_children(
    key: String,
    mut existing: Vec<(String, Node)>,
    nc: NearbyChildren,
) -> Vec<(String, Node)> {
    match nc {
        NearbyChildren::None => existing,
        NearbyChildren::Behind(mut b) => {
            let mut b = b
                .iter()
                .map(|x| (key.clone(), x.clone()))
                .collect::<Vec<(String, Node)>>();
            b.extend(existing);
            b
        }
        NearbyChildren::InFront(f) => {
            let mut f = f
                .iter()
                .map(|x| (key.clone(), x.clone()))
                .collect::<Vec<(String, Node)>>();
            existing.extend(f);
            existing
        }
        NearbyChildren::Both(NearbyChildrenBoth {
            mut behind,
            mut in_front,
        }) => {
            let mut behind = behind
                .iter()
                .map(|x| (key.clone(), x.clone()))
                .collect::<Vec<(String, Node)>>();
            behind.extend(existing);
            let in_front = in_front
                .iter()
                .map(|x| (key.clone(), x.clone()))
                .collect::<Vec<(String, Node)>>();
            behind.extend(in_front);
            behind
        }
    }
}

pub fn static_root(opts: OptStruct) -> Node {
    match opts.mode {
        RenderMode::Layout => {
            // wrap the style node in a div to prevent `Dark Reader` from blowin up the dom.
            Node {
                tag: "div".to_string(),
                attrs: vec![],
                children: vec![NodeType::Node(Node {
                    tag: "style".to_string(),
                    attrs: vec![],
                    children: vec![vdom::text(style::rules())],
                })],
            }
        }
        RenderMode::NoStaicStyleSheet => Node {
            tag: "div".to_string(),
            attrs: vec![],
            children: vec![vdom::text("".to_string())],
        },
        RenderMode::WithVirtualCSS => Node {
            tag: "elm-ui-static-rules".to_string(),
            attrs: vec![vdom::property(Property(
                "rules".to_string(),
                style::rules(),
            ))],
            children: vec![],
        },
    }
}

pub fn add_when<T>(if_this: bool, x: T, to: Vec<T>) -> Vec<T> {
    if if_this {
        let mut x = vec![x];
        x.extend(to);
        x
    } else {
        to
    }
}

/// TODO: This doesn't reduce equivalent attributes completely.
pub fn filter(attrs: Vec<Attribute>) -> Vec<Attribute> {
    let f = |x: Attribute, y: (Vec<Attribute>, HashSet<String>)| {
        let (found, mut has) = y;
        match x {
            Attribute::None => (found, has),
            Attribute::Class(_, _) => {
                let mut x = vec![x];
                x.extend(found);
                (x, has)
            }
            Attribute::Attr(_) => {
                let mut x = vec![x];
                x.extend(found);
                (x, has)
            }
            Attribute::Style(_, _) => {
                let mut x = vec![x];
                x.extend(found);
                (x, has)
            }
            Attribute::Width(_) => {
                if has.contains("width") {
                    (found, has)
                } else {
                    has.insert("width".to_string());
                    let mut x = vec![x];
                    x.extend(found);
                    (x, has)
                }
            }
            Attribute::Height(_) => {
                if has.contains("height") {
                    (found, has)
                } else {
                    has.insert("height".to_string());
                    let mut x = vec![x];
                    x.extend(found);
                    (x, has)
                }
            }
            Attribute::Describe(_) => {
                if has.contains("described") {
                    (found, has)
                } else {
                    has.insert("described".to_string());
                    let mut x = vec![x];
                    x.extend(found);
                    (x, has)
                }
            }
            Attribute::Nearby(_, _) => {
                let mut x = vec![x];
                x.extend(found);
                (x, has)
            }
            Attribute::AlignX(_) => {
                if has.contains("align-x") {
                    (found, has)
                } else {
                    has.insert("align-x".to_string());
                    let mut x = vec![x];
                    x.extend(found);
                    (x, has)
                }
            }
            Attribute::AlignY(_) => {
                if has.contains("align-y") {
                    (found, has)
                } else {
                    has.insert("align-y".to_string());
                    let mut x = vec![x];
                    x.extend(found);
                    (x, has)
                }
            }
            Attribute::TransformComponent(_, _) => {
                if has.contains("transform") {
                    (found, has)
                } else {
                    has.insert("transform".to_string());
                    let mut x = vec![x];
                    x.extend(found);
                    (x, has)
                }
            }
        }
    };

    attrs
        .into_iter()
        .rev()
        .fold((vec![], HashSet::new()), |acc, x| f(x, acc))
        .0
}

pub fn get(
    attrs: Vec<Attribute>,
    mut is_attr: Box<dyn Fn(&Attribute) -> bool>,
) -> Vec<Attribute> {
    let attrs = filter(attrs);
    attrs.into_iter().rev().fold(vec![], |found, x| {
        if is_attr(&x) {
            let mut x = vec![x];
            x.extend(found);
            x
        } else {
            found
        }
    })
}

pub fn extract_spacing_and_padding(
    attrs: Vec<Attribute>,
) -> (Option<Style>, Option<Style>) {
    attrs
        .into_iter()
        .rev()
        .fold((None, None), |(padding, spacing), attr| {
            (
                if padding.is_none() {
                    match &attr {
                        Attribute::Style(
                            _,
                            Style::Padding(name, t, r, b, l),
                        ) => Some(Style::Padding(name.clone(), *t, *r, *b, *l)),
                        _ => None,
                    }
                } else {
                    padding
                },
                if spacing.is_none() {
                    match &attr {
                        Attribute::Style(_, Style::Spacing(name, x, y)) => {
                            Some(Style::Spacing(name.clone(), *x, *y))
                        }
                        _ => None,
                    }
                } else {
                    spacing
                },
            )
        })
}

pub fn get_spacing(attrs: Vec<Attribute>, default: (u8, u8)) -> (u8, u8) {
    let res = attrs.into_iter().rev().fold(None, |acc, attr| {
        if let Some(x) = acc {
            Some(x)
        } else {
            if let Attribute::Style(_, Style::Spacing(_, x, y)) = attr {
                Some((x, y))
            } else {
                None
            }
        }
    });
    res.unwrap_or_else(|| default)
}

pub fn spacing_class_name(x: u8, y: u8) -> String {
    format!("spacing-{}-{}", x, y)
}

pub fn padding_class_name(top: u8, right: u8, bottom: u8, left: u8) -> String {
    format!("pad-{}-{}-{}-{}", top, right, bottom, left,)
}

pub fn padding_class_name_float(
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
) -> String {
    format!(
        "pad-{}-{}-{}-{}",
        top.float_class(),
        right.float_class(),
        bottom.float_class(),
        left.float_class(),
    )
}

pub fn get_width(attrs: Vec<Attribute>) -> Option<Length> {
    attrs.into_iter().rev().fold(None, |acc, attr| {
        if let Some(x) = acc {
            Some(x)
        } else {
            if let Attribute::Width(len) = attr {
                Some(len)
            } else {
                None
            }
        }
    })
}

pub fn get_height(attrs: Vec<Attribute>) -> Option<Length> {
    attrs.into_iter().rev().fold(None, |acc, attr| {
        if let Some(x) = acc {
            Some(x)
        } else {
            if let Attribute::Height(len) = attr {
                Some(len)
            } else {
                None
            }
        }
    })
}

pub fn text_element_classes() -> String {
    format!(
        ".s {} {} {}",
        Classes::Text.to_string(),
        Classes::WidthContent.to_string(),
        Classes::HeightContent.to_string()
    )
}

pub fn text_element(txt: &String) -> Node {
    html::div(
        vec![html::attributes::class(text_element_classes())],
        vec![html::text(txt.clone())],
    )
}

pub fn text_element_fill_classes() -> String {
    format!(
        ".s {} {} {}",
        Classes::Text.to_string(),
        Classes::WidthFill.to_string(),
        Classes::HeightFill.to_string()
    )
}

pub fn text_element_fill(txt: &String) -> Node {
    html::div(
        vec![html::attributes::class(text_element_fill_classes())],
        vec![html::text(txt.clone())],
    )
}

pub fn render_root(
    opts: Vec<Opt>,
    attrs: Vec<Attribute>,
    child: Element,
) -> Node {
    let opts = OptStruct::from_opts(opts);

    let el = element(
        LayoutContext::AsEl,
        NodeName::div(),
        attrs,
        Children::Unkeyed(vec![child]),
    );

    match el {
        Element::Unstyled(FinalizeNodeArgs {
            has,
            node,
            attributes,
            children,
            embed_mode,
        }) => finalize_node(
            has,
            node,
            attributes,
            children,
            embed_mode.unwrap(),
            LayoutContext::AsEl,
        ),
        Element::Styled(Styled {
            styles,
            html:
                FinalizeNodeArgs {
                    has,
                    node,
                    attributes,
                    children,
                    embed_mode,
                },
        }) => finalize_node(
            has,
            node,
            attributes,
            children,
            if let RenderMode::NoStaicStyleSheet = &opts.mode {
                EmbedStyle::OnlyDynamic(opts, styles)
            } else {
                EmbedStyle::StaticRootAndynamic(opts, styles)
            },
            LayoutContext::AsEl,
        ),
        Element::Text(txt) => text_element(&txt),
        Element::Empty => text_element(&"".to_string()),
    }
}

pub fn root_style() -> Vec<Attribute> {
    let families = vec![
        Font::Typeface("Open Sans".to_string()),
        Font::Typeface("Helvetica".to_string()),
        Font::Typeface("Verdana".to_string()),
        Font::SansSerif,
    ];

    vec![
        Attribute::Style(
            Flag::bg_color(),
            Style::Colored(
                format!(
                    "bg-{}",
                    Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 0.0
                    }
                    .format_color_class()
                ),
                "background-color".to_string(),
                Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 0.0,
                },
            ),
        ),
        Attribute::Style(
            Flag::font_color(),
            Style::Colored(
                format!(
                    "fc-{}",
                    Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0
                    }
                    .format_color_class()
                ),
                "color".to_string(),
                Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
            ),
        ),
        Attribute::Style(Flag::font_size(), Style::FontSize(20)),
        Attribute::Style(
            Flag::font_family(),
            Style::FontFamily(
                families
                    .iter()
                    .fold(String::from("font-"), |current, font| {
                        font.render_class_name(current)
                    }),
                families,
            ),
        ),
    ]
}

pub fn to_stylesheet(opts: OptStruct, stylesheet: Vec<Style>) -> Node {
    match opts.mode {
        RenderMode::Layout | RenderMode::NoStaicStyleSheet => {
            // wrap the style node in a div to prevent `Dark Reader` from blowin up the dom.
            vdom::node(
                "div".to_string(),
                vec![],
                vec![NodeType::Node(vdom::node(
                    "style".to_string(),
                    vec![],
                    vec![vdom::text(to_stylesheet_str(opts, stylesheet))],
                ))],
            )
        }
        RenderMode::WithVirtualCSS => {
            // wrap the style node in a div to prevent `Dark Reader` from blowin up the dom.
            vdom::node(
                "elm-ui-rules".to_string(),
                vec![property(Property(
                    "rules".to_string(),
                    encode_styles(opts, stylesheet),
                ))],
                vec![],
            )
        }
    }
}

pub fn render_toplevel_vals(rules: &mut Vec<(String, Vec<Font>)>) -> String {
    let with_import = |font: &Font| match font {
        Font::ImportFont(_, url) => Some(format!("@import url('{}');", url)),
        // Font::FontWith(with) => {
        //     with.url.map(|x| Some(format!("@import url('{}');", x)))
        // }
        _ => None,
    };

    let mut all_names =
        rules.iter().map(|rule| &rule.0).collect::<Vec<&String>>();
    let font_imports = |name: &String, typfaces: &Vec<Font>| {
        typfaces
            .iter()
            .filter_map(|font| with_import(font))
            .collect::<Vec<String>>()
            .join("\n")
    };
    let mut font_adjustments =
        |name: &String, typefaces: &Vec<Font>| -> String {
            match typeface_adjustment(typefaces) {
                None => all_names
                    .iter()
                    .map(|n| render_null_adjustment_rule(name, n))
                    .collect::<Vec<String>>()
                    .join(""),
                Some(mut adj) => all_names
                    .iter()
                    .map(|n| render_font_adjustment_rule(name, &adj, n))
                    .collect::<Vec<String>>()
                    .join(""),
            }
        };

    let mut x = rules
        .iter()
        .map(|(name, typefaces)| font_imports(name, typefaces))
        .collect::<Vec<String>>()
        .join("");

    let y = rules
        .iter()
        .map(|(name, typefaces)| font_adjustments(name, typefaces))
        .collect::<Vec<String>>()
        .join("");
    x.push_str(&y);
    x
}

pub fn render_null_adjustment_rule(target: &String, other: &String) -> String {
    let name = if target == other {
        target.clone()
    } else {
        format!("{} .{}", other, target)
    };

    let x = bracket(
        &format!(
            ".{}.{}, .{} .{}",
            name,
            Classes::SizeByCapital.to_string(),
            name,
            Classes::SizeByCapital.to_string()
        ),
        &vec![("line-height".to_string(), "1".to_string())],
    );

    let y = bracket(
        &format!(
            ".{}.{}> .{}, .{} .{} > .{}",
            name,
            Classes::SizeByCapital.to_string(),
            Classes::Text.to_string(),
            name,
            Classes::SizeByCapital.to_string(),
            Classes::Text.to_string()
        ),
        &vec![
            ("vertical-align".to_string(), "0".to_string()),
            ("line-height".to_string(), "1".to_string()),
        ],
    );

    format!("{} {}", x, y)
}

pub fn font_rule(
    name: &String,
    modifier: &String,
    adjustments: &(Vec<(String, String)>, Vec<(String, String)>),
) -> Vec<String> {
    let (parent_adj, text_adj) = adjustments;
    vec![
        bracket(
            &format!(".{}.{}, .{} .{}", name, modifier, name, modifier),
            parent_adj,
        ),
        bracket(
            &format!(
                ".{}.{}> .{}, .{} .{} > .{}",
                name,
                modifier,
                Classes::Text.to_string(),
                name,
                modifier,
                Classes::Text.to_string()
            ),
            text_adj,
        ),
    ]
}

pub fn render_font_adjustment_rule(
    target: &String,
    adjustment: &AdjustmentRules,
    other: &String,
) -> String {
    let AdjustmentRules { full, capital } = adjustment;
    let name = if target == other {
        target.clone()
    } else {
        format!("{} .{}", other, target)
    };
    let mut x = font_rule(
        &name,
        &Classes::SizeByCapital.to_string().to_string(),
        capital,
    );
    x.extend(font_rule(
        &name,
        &Classes::FullSize.to_string().to_string(),
        full,
    ));
    x.join(" ")
}

pub fn bracket(selector: &String, rules: &Vec<(String, String)>) -> String {
    format!(
        "{} {{{}}}",
        selector,
        rules
            .iter()
            .map(|(name, val)| format!("{}: {};", name, val))
            .collect::<Vec<String>>()
            .join("")
    )
}

pub fn render_props(
    force: bool,
    property: &Property,
    existing: &String,
) -> String {
    let Property(k, v) = property;
    if force {
        format!("{}\n  {}: {} !important;", existing, k, v)
    } else {
        format!("{}\n  {}: {};", existing, k, v)
    }
}

pub fn encode_styles(opts: OptStruct, stylesheet: Vec<Style>) -> String {
    let styles = stylesheet
        .into_iter()
        .map(|style| {
            let styled = todo_render_style_rule(opts, style.clone(), None);
            (
                style.name(),
                styled
                    .iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<String>>()
                    .join(","),
            )
        })
        .map(|(k, v)| format!("\"{}\":\"{}\"", k, v))
        .collect::<Vec<String>>()
        .join(",");
    format!("{{{}}}", styles)
}

pub fn to_stylesheet_str(opts: OptStruct, stylesheet: Vec<Style>) -> String {
    let combine = stylesheet.iter().fold(
        (vec![], vec![]),
        |rendered: (Vec<String>, Vec<(String, Vec<Font>)>), style| {
            let mut rules = rendered.0;
            rules.extend(todo_render_style_rule(opts, style.clone(), None));

            let top = if let Some(t) = style.toplevel_val() {
                let mut t = vec![t];
                t.extend(rendered.1);
                t
            } else {
                rendered.1
            };

            (rules, top)
        },
    );
    let (rules, mut top) = combine;
    let mut vals = render_toplevel_vals(&mut top);
    vals.push_str(&rules.concat());
    vals
}

pub fn render_style(
    opts: OptStruct,
    pseudo: &Option<PseudoClass>,
    selector: String,
    props: Vec<Property>,
) -> Vec<String> {
    if let Some(pseudo) = pseudo {
        match pseudo {
            PseudoClass::Hover => match opts.hover {
                HoverSetting::No => vec![],
                HoverSetting::Force => {
                    let rprops = props.iter().fold(
                        String::new(),
                        |existing, property| {
                            render_props(true, property, &existing)
                        },
                    );
                    vec![format!("{}-hv {{{}\n}}", selector, rprops)]
                }
                HoverSetting::Allow => {
                    let rprops = props.iter().fold(
                        String::new(),
                        |existing, property| {
                            render_props(false, property, &existing)
                        },
                    );
                    vec![format!("{}-hv:hover {{{}\n}}", selector, rprops)]
                }
            },
            PseudoClass::Focus => {
                let rprops =
                    props.iter().fold(String::new(), |existing, property| {
                        render_props(false, property, &existing)
                    });

                vec![
                    format!(
                        "{}-fs:focus {{{}\n}}",
                        selector,
                        rprops,
                    ),
                    format!(
                        ".s:focus {}-fs {{{}\n}}",
                        selector,
                        rprops,
                    ),
                    format!(
                        "{}-fs:focus-within {{{}\n}}",
                        selector,
                        rprops,
                    ),
                    format!(
                        ".ui-slide-bar:focus + .s .focusable-thumb{}-fs {{{}\n}}", selector, rprops
                    )
                ]
            }
            PseudoClass::Active => {
                let rprops =
                    props.iter().fold(String::new(), |existing, property| {
                        render_props(false, property, &existing)
                    });
                vec![format!("{}-act:active {{{}\n}}", selector, rprops)]
            }
        }
    } else {
        let rprops = props.iter().fold(String::new(), |existing, property| {
            render_props(false, property, &existing)
        });
        vec![format!("{}-hv:hover {{{}\n}}", selector, rprops)]
    }
}

pub fn todo_render_style_rule(
    opts: OptStruct,
    rule: Style,
    pseudo: Option<PseudoClass>,
) -> Vec<String> {
    match rule {
        Style::Style(selector, props) => {
            render_style(opts, &pseudo, selector, props)
        }
        Style::Shadows(name, prop) => render_style(
            opts,
            &pseudo,
            format!(".{}", name),
            vec![Property("box-shadow".to_string(), prop)],
        ),
        Style::Transparency(name, transparency) => {
            let opacity = f32::max(0.0, f32::min(1.0, (1.0 - transparency)));
            render_style(
                opts,
                &pseudo,
                format!(".{}", name),
                vec![Property("opacity".to_string(), format!("{}", opacity))],
            )
        }
        Style::FontSize(i) => render_style(
            opts,
            &pseudo,
            format!(".font-size-{}", i),
            vec![Property("font-size".to_string(), format!("{}px", i))],
        ),
        Style::FontFamily(name, typefaces) => {
            let features = typefaces
                .iter()
                .filter_map(|f| f.render_variants())
                .collect::<Vec<String>>()
                .join(", ");

            let families = vec![
                Property(
                    "font-family".to_string(),
                    typefaces
                        .iter()
                        .map(|f| f.name())
                        .collect::<Vec<String>>()
                        .join(" ,"),
                ),
                Property("font-feature-settings".to_string(), features),
                Property(
                    "font-variant".to_string(),
                    if typefaces.iter().any(|f| f.has_small_caps()) {
                        "small-caps".to_string()
                    } else {
                        "normal".to_string()
                    },
                ),
            ];

            render_style(opts, &pseudo, format!(".{}", name), families)
        }
        Style::Single(class, prop, val) => render_style(
            opts,
            &pseudo,
            format!(".{}", class),
            vec![Property(prop, val)],
        ),
        Style::Colored(class, prop, color) => render_style(
            opts,
            &pseudo,
            format!(".{}", class),
            vec![Property(prop, color.format_color())],
        ),
        Style::Spacing(cls, x, y) => {
            let class = format!(".{}", cls);

            let half_x = format!("{}px", (x as f32 / 2.0));

            let half_y = format!("{}px", (x as f32 / 2.0));

            let px_x = format!("{}px", x);

            let px_y = format!("{}px", y);

            let row = format!(".{}", Classes::Row.to_string());

            let wrapped_row =
                format!(".{}{}", Classes::Wrapped.to_string(), row);

            let col = format!(".{}", Classes::Column.to_string());

            let page = format!(".{}", Classes::Page.to_string());

            let paragraph = format!(".{}", Classes::Paragraph.to_string());

            let left = format!(".{}", Classes::AlignLeft.to_string());

            let right = format!(".{}", Classes::AlignRight.to_string());

            let any = format!(".{}", Classes::Any.to_string());

            vec![
                render_style(
                    opts,
                    &pseudo,
                    format!("{}{} > {} + {}", class, row, any, any),
                    vec![Property("margin-left".to_string(), px_x.clone())],
                ),
                // margins don't apply to last element of normal, unwrapped rows
                // render_style(
                //     opts,
                //     pseudo,
                //     format!(
                //         "{}{} > {}:first-child",
                //         class,
                //         row,
                //         any
                //     ),
                //     vec![Property("margin".to_string(), "0".to_string())]
                // )
                // For wrapped rows, margins always apply because
                // we handle "canceling out" the other margins
                // manually in the element.
                render_style(
                    opts,
                    &pseudo,
                    format!("{}{} > {}", class, wrapped_row, any),
                    vec![Property(
                        "margin".to_string(),
                        format!("{} {}", half_y, half_x),
                    )],
                ),
                // render_style(
                //     opts,
                //     pseudo
                //     format!(
                //         "{}{} > {}:last-child",
                //         class
                //         wrapped_row,
                //         any
                //     ),
                //     vec![
                //         Property("margin-right".to_string(), "0".to_string())
                //     ]
                // ),
                // columns
                render_style(
                    opts,
                    &pseudo,
                    format!("{}{} > {} + {}", class, col, any, any),
                    vec![Property("margin-top".to_string(), px_y.clone())],
                ),
                render_style(
                    opts,
                    &pseudo,
                    format!("{}{} > {} + {}", class, page, any, any),
                    vec![Property("margin-top".to_string(), px_y.clone())],
                ),
                render_style(
                    opts,
                    &pseudo,
                    format!("{}{} > {}", class, page, left),
                    vec![Property("margin-right".to_string(), px_x.clone())],
                ),
                render_style(
                    opts,
                    &pseudo,
                    format!("{}{} > {}", class, page, right),
                    vec![Property("margin-left".to_string(), px_x.clone())],
                ),
                render_style(
                    opts,
                    &pseudo,
                    format!("{}{}", class, paragraph),
                    vec![Property(
                        "line-height".to_string(),
                        format!("calc(1em + {}px)", y),
                    )],
                ),
                render_style(
                    opts,
                    &pseudo,
                    format!("textarea{}{}", any, class),
                    vec![
                        Property(
                            "line-height".to_string(),
                            format!("calc(1em + {}px)", y),
                        ),
                        Property(
                            "height".to_string(),
                            format!("calc(100% + {}px)", y),
                        ),
                    ],
                ),
                // render_style(
                //     opts,
                //     &pseudo,
                //     format!("{}{} > {}", class, paragraph, any),
                //     vec![
                //         Property("margin-right".to_string(), px_x),
                //         Property("margin-bottom".to_string(), px_y)
                //     ]
                // )
                render_style(
                    opts,
                    &pseudo,
                    format!("{}{} > {}", class, paragraph, left),
                    vec![Property("margin-right".to_string(), px_x.clone())],
                ),
                render_style(
                    opts,
                    &pseudo,
                    format!("{} {} > {}", class, paragraph, right),
                    vec![Property("margin-left".to_string(), px_x.clone())],
                ),
                render_style(
                    opts,
                    &pseudo,
                    format!("{} {}::after", class, paragraph),
                    vec![
                        Property("content".to_string(), "''".to_string()),
                        Property("display".to_string(), "block".to_string()),
                        Property("height".to_string(), "0".to_string()),
                        Property("width".to_string(), "0".to_string()),
                        Property(
                            "margin-top".to_string(),
                            format!("{}px", (-1 * (y as i8 / 2))),
                        ),
                    ],
                ),
                render_style(
                    opts,
                    &pseudo,
                    format!("{} {}::before", class, paragraph),
                    vec![
                        Property("content".to_string(), "''".to_string()),
                        Property("display".to_string(), "block".to_string()),
                        Property("height".to_string(), "0".to_string()),
                        Property("width".to_string(), "0".to_string()),
                        Property(
                            "margin-bottom".to_string(),
                            format!("{}px", (-1 * (y as i8 / 2))),
                        ),
                    ],
                ),
            ]
            .concat()
        }
        Style::Padding(cls, top, right, bottom, left) => {
            let class = format!(".{}", cls);

            render_style(
                opts,
                &pseudo,
                class,
                vec![Property(
                    "padding".to_string(),
                    format!("{} px {}px {}px {}px", top, right, bottom, left),
                )],
            )
        }
        Style::BorderWidth(cls, top, right, bottom, left) => {
            let class = format!(".{}", cls);

            render_style(
                opts,
                &pseudo,
                class,
                vec![Property(
                    "border-width".to_string(),
                    format!("{} px {}px {}px {}px", top, right, bottom, left),
                )],
            )
        }
        Style::GridTemplate(template) => {
            let grid_rows = template
                .rows
                .iter()
                .map(|n| n.class_name())
                .collect::<Vec<String>>()
                .join("-");

            let grid_cols = template
                .columns
                .iter()
                .map(|n| n.class_name())
                .collect::<Vec<String>>()
                .join("-");

            let class = format!(
                ".grid-rows-{}-cols-{}-space-x-{}-space-y-{}",
                grid_rows,
                grid_cols,
                template.spacing.0.class_name(),
                template.spacing.1.class_name(),
            );

            let to_grid_len = |l: &Length| to_grid_len_helper(&None, &None, l);

            let spacing_y = to_grid_len(&template.spacing.1);

            let ms_cols = format!(
                "-ms-grid-columns: {};",
                template
                    .columns
                    .iter()
                    .map(|l| to_grid_len(l))
                    .collect::<Vec<String>>()
                    .join(&spacing_y)
            );

            let ms_rows = format!(
                "-ms-grid-rows: {};",
                template
                    .rows
                    .iter()
                    .map(|l| to_grid_len(l))
                    .collect::<Vec<String>>()
                    .join(&spacing_y)
            );

            let base =
                format!("{}{{{}}}", class, format!("{}{}", ms_cols, ms_rows));

            let cols = format!(
                "grid-template-columns: {};",
                template
                    .columns
                    .iter()
                    .map(|l| to_grid_len(l))
                    .collect::<Vec<String>>()
                    .join(" ")
            );

            let rows = format!(
                "grid-template-rows: {};",
                template
                    .rows
                    .iter()
                    .map(|l| to_grid_len(l))
                    .collect::<Vec<String>>()
                    .join(" ")
            );

            let gap_x = format!(
                "grid-column-gap:{};",
                to_grid_len(&template.spacing.0)
            );

            let gap_y =
                format!("grid-row-gap:{};", to_grid_len(&template.spacing.1));

            let modern_grid = format!(
                "{}{{{}}}",
                class,
                format!("{}{}{}{}", cols, rows, gap_x, gap_y)
            );

            let supports =
                format!("@supports (display:grid) {{{}}}", modern_grid);

            vec![base, supports]
        }
        Style::GridPosition(pos) => {
            let class = format!(
                ".grid-pos-{}-{}-{}-{}",
                pos.row, pos.col, pos.width, pos.height,
            );

            let ms_pos = format!(
                "-ms-grid-row: {}; -ms-grid-row-span: {}; -ms-grid-column: {}; -ms-grid-column-span: {};",
                pos.row,
                pos.height,
                pos.col,
                pos.width
            );

            let base = format!("{}{{{}}}", class, ms_pos);

            let modern_pos = format!(
                "grid-row: {} / {}; grid-column: {} / {};",
                pos.row,
                (pos.row + pos.height),
                pos.col,
                (pos.col + pos.width)
            );

            let modern_grid = format!("{}{{{}}}", class, modern_pos);

            let supports =
                format!("@supports (display:grid) {{{}}}", modern_grid);

            vec![base, supports]
        }
        Style::PseudoSelector(class, styles) => styles
            .into_iter()
            .flat_map(|s| todo_render_style_rule(opts, s, Some(class.clone())))
            .collect(),
        Style::Transform(transform) => {
            let val = transform.value();
            let class = transform.class();

            if let (Some(class), Some(val)) = (class, val) {
                render_style(
                    opts,
                    &pseudo,
                    format!(".{}", class),
                    vec![Property("transform".to_string(), val)],
                )
            } else {
                vec![]
            }
        }
    }
}

fn to_grid_len_helper(
    min: &Option<u64>,
    max: &Option<u64>,
    l: &Length,
) -> String {
    match l {
        Length::Px(px) => format!("{}px", px),
        Length::Content => match (min, max) {
            (None, None) => "max-content".to_string(),
            (Some(size), None) => {
                format!("minmax({}px, max-content)", size)
            }
            (None, Some(size)) => {
                format!("minmax(max-content, {}px)", size)
            }
            (Some(min), Some(max)) => {
                format!("minmax({}px, {}px", min, max)
            }
        },
        Length::Fill(i) => match (min, max) {
            (None, None) => format!("{}fr", i),
            (Some(size), None) => {
                format!("minmax({}px, {}frfr)", size, i)
            }
            (None, Some(size)) => {
                format!("minmax(max-content, {}px)", size)
            }
            (Some(min), Some(max)) => {
                format!("minmax({}px, {}px", min, max)
            }
        },
        Length::Min(m, len) => to_grid_len_helper(&Some(*m), max, &**len),
        Length::Max(m, len) => to_grid_len_helper(min, &Some(*m), &**len),
    }
}

// map : (msg -> msg1) -> Element msg -> Element msg1
// map fn el =
//     case el of
//         Styled styled ->
//             Styled
//                 { styles = styled.styles
//                 , html = \add context -> VirtualDom.map fn <| styled.html add context
//                 }

//         Unstyled html ->
//             Unstyled (VirtualDom.map fn << html)

//         Text str ->
//             Text str

//         Empty ->
//             Empty

// mapAttr : (msg -> msg1) -> Attribute aligned msg -> Attribute aligned msg1
// mapAttr fn attr =
//     case attr of
//         NoAttribute ->
//             NoAttribute

//         Describe description ->
//             Describe description

//         AlignX x ->
//             AlignX x

//         AlignY y ->
//             AlignY y

//         Width x ->
//             Width x

//         Height x ->
//             Height x

//         Class x y ->
//             Class x y

//         StyleClass flag style ->
//             StyleClass flag style

//         Nearby location elem ->
//             Nearby location (map fn elem)

//         Attr htmlAttr ->
//             Attr (VirtualDom.mapAttribute fn htmlAttr)

//         TransformComponent fl trans ->
//             TransformComponent fl trans

// mapAttrFromStyle : (msg -> msg1) -> Attribute Never msg -> Attribute () msg1
// mapAttrFromStyle fn attr =
//     case attr of
//         NoAttribute ->
//             NoAttribute

//         Describe description ->
//             Describe description

//         AlignX x ->
//             AlignX x

//         AlignY y ->
//             AlignY y

//         Width x ->
//             Width x

//         Height x ->
//             Height x

//         -- invalidation key "border-color" as opposed to "border-color-10-10-10" that will be the key for the class
//         Class x y ->
//             Class x y

//         StyleClass flag style ->
//             StyleClass flag style

//         Nearby location elem ->
//             Nearby location (map fn elem)

//         Attr htmlAttr ->
//             Attr (VirtualDom.mapAttribute fn htmlAttr)

//         TransformComponent fl trans ->
//             TransformComponent fl trans

// unwrapDecorations : List (Attribute Never Never) -> List Style
// unwrapDecorations attrs =
//     case List.foldl unwrapDecsHelper ( [], Untransformed ) attrs of
//         ( styles, transform ) ->
//             Transform transform :: styles

// unwrapDecsHelper : Attribute Never Never -> (List Style, Transformation) -> (List Style, Transformation)
// unwrapDecsHelper attr ( styles, trans ) =
//     case removeNever attr of
//         StyleClass _ style ->
//             ( style :: styles, trans )

//         TransformComponent _ component ->
//             ( styles, composeTransformation trans component )

//         _ ->
//             ( styles, trans )

// removeNever : Attribute Never Never -> Attribute () msg
// removeNever style =
//     mapAttrFromStyle Basics.never style
