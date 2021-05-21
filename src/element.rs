use std::cmp;
use std::ops::Neg;

use crate::{
    flag::{Field, Flag},
    model::{
        div, element, extract_spacing_and_padding, html, padding_class_name,
        padding_class_name_float, render_root, root_style, spacing_class_name,
        unwrap_decorations, virtual_dom as vdom, virtual_dom::Node, Attribute,
        Children, Color, Coordinate, Description, Element, FloatClass,
        FocusStyle, HAlign, HoverSetting, LayoutContext, Length, Location,
        NodeName, Opt, PseudoClass, RenderMode, Style, TransformComponent,
        VAlign,
    },
    style::Classes,
};

///
///
/// # Basic Elements
///
/// @docs Element, none, text, el
///
///
/// # Rows and Columns
///
/// When we want more than one child on an element, we want to be _specific_ about /// how they will be laid out.
///
/// So, the common ways to do that would be `row` and `column`.
///
/// @docs row, wrappedRow, column
///
///
/// # Text Layout
///
/// Text layout needs some specific considerations.
///
/// @docs paragraph, textColumn
///
///
/// # Data Table
///
/// @docs Column, table, IndexedColumn, indexedTable
///
///
/// # Size
///
/// @docs Attribute, width, height, Length, px, shrink, fill, fillPortion, maximum, minimum
///
///
/// # Debugging
///
/// @docs explain
///
///
/// # Padding and Spacing
///
/// There's no concept of margin in `elm-ui`, instead we have padding and spacing.
///
/// Padding is the distance between the outer edge and the content, and spacing is /// the space between children.
///
/// So, if we have the following row, with some padding and spacing.
///
///     Element.row [ padding 10, spacing 7 ]
///         [ Element.el [] nonewidth
///         , Element.el [] none
///         , Element.el [] none
///         ]
///
/// Here's what we can expect:
///
/// ![Three boxes spaced 7 pixels apart. There's a 10 pixel distance from the edge /// of the parent to the boxes.](https://mdgriffith.gitbooks.io/style-elements//// content/assets/spacing-400.png)
///
/// **Note** `spacing` set on a `paragraph`, will set the pixel spacing between /// lines.
///
/// @docs padding, paddingXY, paddingEach
///
/// @docs spacing, spacingXY, spaceEvenly
///
///
/// # Alignment
///
/// Alignment can be used to align an `Element` within another `Element`.
///
///     Element.el [ centerX, alignTop ] (text "I'm centered and aligned top!")
///
/// If alignment is set on elements in a layout such as `row`, then the element /// will push the other elements in that direction. Here's an example.
///
///     Element.row []
///         [ Element.el [] Element.none
///         , Element.el [ alignLeft ] Element.none
///         , Element.el [ centerX ] Element.none
///         , Element.el [ alignRight ] Element.none
///         ]
///
/// will result in a layout like
///
///     |-|-|    |-|    |-|
///
/// Where there are two elements on the left, one on the right, and one in the /// center of the space between the elements on the left and right.
///
/// **Note** For text alignment, check out `Element.Font`!
///
/// @docs centerX, centerY, alignLeft, alignRight, alignTop, alignBottom
///
///
/// # Transparency
///
/// @docs transparent, alpha, pointer
///
///
/// # Adjustment
///
/// @docs moveUp, moveDown, moveRight, moveLeft, rotate, scale
///
///
/// # Clipping and Scrollbars
///
/// Clip the content if it overflows.
///
/// @docs clip, clipX, clipY
///
/// Add a scrollbar if the content is larger than the element.
///
/// @docs scrollbars, scrollbarX, scrollbarY
///
///
/// # Rendering
///
/// @docs layout, layoutWith, Option, noStaticStyleSheet, forceHover, noHover, /// focusStyle, FocusStyle
///
///
/// # Links
///
/// @docs link, newTabLink, download, downloadAs
///
///
/// # Images
///
/// @docs image
///
///
/// # Color
///
/// In order to use attributes like `Font.color` and `Background.color`, you'll /// need to make some colors!
///
/// @docs Color, rgba, rgb, rgb255, rgba255, fromRgb, fromRgb255, toRgb
///
///
/// # Nearby Elements
///
/// Let's say we want a dropdown menu. Essentially we want to say: _put this /// element below this other element, but don't affect the layout when you do_.
///
///     Element.row []
///         [ Element.el
///             [ Element.below (Element.text "I'm below!")
///             ]
///             (Element.text "I'm normal!")
///         ]
///
/// This will result in
///
///     |- I'm normal! -|
///        I'm below
///
/// Where `"I'm Below"` doesn't change the size of `Element.row`.
///
/// This is very useful for things like dropdown menus or tooltips.
///
/// @docs above, below, onRight, onLeft, inFront, behindContent
///
///
/// # Temporary Styling
///
/// @docs Attr, Decoration, mouseOver, mouseDown, focused
///
///
/// # Responsiveness
///
/// The main technique for responsiveness is to store window size information in /// your model.
///
/// Install the `Browser` package, and set up a subscription for [`Browser.Events./// onResize`](https://package.elm-lang.org/packages/elm/browser/latest//// Browser-Events#onResize).
///
/// You'll also need to retrieve the initial window size. You can either use /// [`Browser.Dom.getViewport`](https://package.elm-lang.org/packages/elm/browser//// latest/Browser-Dom#getViewport) or pass in `window.innerWidth` and `window./// innerHeight` as flags to your program, which is the preferred way. This /// requires minor setup on the JS side, but allows you to avoid the state where /// you don't have window info.
///
/// @docs Device, DeviceClass, Orientation, classifyDevice
///
///
/// # Scaling
///
/// @docs modular
///
///
/// ## Mapping
///
/// @docs map, mapAttribute
///
///
/// ## Compatibility
///
/// @docs html, htmlAttribute

/// Provide the red, green, and blue channels for the color.
///
/// Each channel takes a value between 0 and 1.
pub fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color { r, g, b, a: 1.0 }
}

pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color { r, g, b, a }
}

/// Provide the red, green, and blue channels for the color.
///
/// Each channel takes a value between 0 and 255.
///
pub fn rgb255(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: (r as f32) / 255.0,
        g: (r as f32) / 255.0,
        b: (r as f32) / 255.0,
        a: 1.0,
    }
}

pub fn rgba255(r: u8, g: u8, b: u8, a: f32) -> Color {
    Color {
        r: (r as f32) / 255.0,
        g: (r as f32) / 255.0,
        b: (r as f32) / 255.0,
        a,
    }
}

// /// This is a special attribute that counts as both a Attribute and a Decoration
// type Attr = Attribute;

// /// Only decorations
// type Decoration = Attribute;

pub fn px(px: u64) -> Length {
    Length::Px(px)
}

/// Shrink an element to fit its contents.
pub fn shrink() -> Length {
    Length::Content
}

/// Fill the available space. The available space will be split evenly between elements that have width(fill()).
pub fn fill() -> Length {
    Length::Fill(1)
}

/// Similarly you can set a minimum boundary.
///
/// el(text("I will stop at 300px"))
/// .height(minimum(30, maximum(300, fill()))
///
pub fn min(i: u64, l: Length) -> Length {
    Length::Min(i, Box::new(l))
}

/// Add a maximum to a length.
///
/// el(text("I will stop at 300px"))
/// .height(maximum(300, fill())
///
pub fn max(i: u64, l: Length) -> Length {
    Length::Max(i, Box::new(l))
}

/// Sometimes you may not want to split available space evenly.
/// In this case you can use fill_portion to define which
/// elements should have what portion of the available space.
///
/// So, two elements, one with width(fill_portion(2)) and one
/// with width(fill_portion(3)). The first would get 2 portions
/// of the available space, while the second would get 3.
///
/// **Also:** fill == fill_portion(1)
pub fn fill_portion(i: u64) -> Length {
    Length::Fill(i)
}

/// This is your top level node where you can turn Element into Html.
pub fn layout(attrs: Vec<Attribute>, child: Element) -> Node {
    layout_with(vec![], attrs, child)
}

pub fn layout_with(
    opts: Vec<Opt>,
    attrs: Vec<Attribute>,
    child: Element,
) -> Node {
    let mut attr = vec![Attribute::html_class(format!(
        "{} {} {}",
        Classes::Root.to_string(),
        Classes::Any.to_string(),
        Classes::Single.to_string(),
    ))];

    attr.extend(root_style());
    attr.extend(attrs);

    render_root(opts, attr, child)
}

/// Elm UI embeds two StyleSheets, one that is constant,
/// and one that changes dynamically based on styles
/// collected from the elements being rendered.
///
/// This option will stop the static/constant stylesheet
/// from rendering.
///
/// If you're embedding multiple elm-ui `layout` elements,
/// you need to guarantee that only one is rendering the
/// static stylesheet and that it's above all the others
/// in the DOM tree.
///
pub fn no_static_style_sheet() -> Opt {
    Opt::Render(RenderMode::NoStaicStyleSheet)
}

pub fn focus_style(fs: FocusStyle) -> Opt {
    Opt::Focus(fs)
}

/// Disable all mouse_over styles.
pub fn no_hover() -> Opt {
    Opt::Hover(HoverSetting::No)
}

/// Any hover styles, aka attributes with mouse_over in
/// the name, will be always turned on.
///
/// This is useful for when you're targeting a platform
/// that has no mouse, such as mobile.
pub fn force_hover() -> Opt {
    Opt::Hover(HoverSetting::Force)
}

/// When you want to render exactly nothing.
fn none() -> Element {
    Element::Empty
}

/// Create some plain text.
///
/// text "Hello, you stylish developer!"
///
/// **Note** text does not wrap by default.
/// In order to get text to wrap, check out paragraph!
fn text(content: String) -> Element {
    Element::Text(content)
}

/// The basic building block of your layout.
///
/// You can think of an el as a div,
/// but it can only have one child.
///
/// If you want multiple children, you'll need
/// to use something like row or column.
///
///     use element::{Element, rgb, el};
///
///     fn my_element() -> Element {
///         el([text(
///             "You've made a stylish element!"
///         )])
///         .bg_color(rgb(0.0, 0.5, 0.0))
///         .border_color(rgb(0.0, 0.7, 0.0))
///     }
///
pub fn el(attrs: Vec<Attribute>, child: Element) -> Element {
    let mut attr =
        vec![Attribute::Width(shrink()), Attribute::Height(shrink())];

    attr.extend(attrs);
    let attrs = attr;

    element(
        LayoutContext::AsEl,
        NodeName::div(),
        attrs,
        Children::Unkeyed(vec![child]),
    )
}

pub fn row(attrs: Vec<Attribute>, children: Vec<Element>) -> Element {
    let mut attr = vec![
        Attribute::html_class(format!(
            "{} {}",
            Classes::ContentLeft.to_string(),
            Classes::ContentCenterY.to_string()
        )),
        Attribute::Width(shrink()),
        Attribute::Height(shrink()),
    ];

    attr.extend(attrs);
    let attrs = attr;

    element(
        LayoutContext::AsRow,
        NodeName::div(),
        attrs,
        Children::Unkeyed(children),
    )
}

pub fn column(attrs: Vec<Attribute>, children: Vec<Element>) -> Element {
    let mut attr = vec![
        Attribute::html_class(format!(
            "{} {}",
            Classes::ContentTop.to_string(),
            Classes::ContentLeft.to_string()
        )),
        Attribute::Width(shrink()),
        Attribute::Height(shrink()),
    ];

    attr.extend(attrs);
    let attrs = attr;

    element(
        LayoutContext::AsColumn,
        NodeName::div(),
        attrs,
        Children::Unkeyed(children),
    )
}

/// Same as row, but will wrap if it takes up
/// too much horizontal space.
pub fn wrapped_row(attrs: Vec<Attribute>, children: Vec<Element>) -> Element {
    let (padded, spaced) = extract_spacing_and_padding(attrs.clone());

    if let Some(Style::Spacing(name, x, y)) = spaced {
        let new_padding = if let Some(Style::Padding(_, t, r, b, l)) = padded {
            if r >= (x / 2) as f32 && b >= (y as f32 / 2.0) {
                let new_top = t - (y as f32 / 2.0);
                let new_right = r - (y as f32 / 2.0);
                let new_bottom = b - (y as f32 / 2.0);
                let new_left = l - (y as f32 / 2.0);

                Some(Attribute::Style(
                    Flag::padding(),
                    Style::Padding(
                        padding_class_name_float(t, r, b, l),
                        new_top,
                        new_right,
                        new_bottom,
                        new_left,
                    ),
                ))
            } else {
                None
            }
        } else {
            None
        };

        if let Some(pad) = new_padding {
            let mut attr = vec![
                Attribute::html_class(format!(
                    "{} {} {}",
                    Classes::ContentLeft.to_string(),
                    Classes::ContentCenterY.to_string(),
                    Classes::Wrapped.to_string(),
                )),
                Attribute::Width(shrink()),
                Attribute::Height(shrink()),
            ];

            attr.extend(attrs);
            attr.push(pad);
            let attrs = attr;

            element(
                LayoutContext::AsRow,
                NodeName::div(),
                attrs,
                Children::Unkeyed(children),
            )
        } else {
            // Not enough space in padding to compensate for spacing
            let half_x = -(x as f32 / 2.0);
            let half_y = -(y as f32 / 2.0);

            element(
                LayoutContext::AsEl,
                div(),
                attrs,
                Children::Unkeyed(vec![element(
                    LayoutContext::AsRow,
                    div(),
                    vec![
                        Attribute::html_class(format!(
                            "{} {} {}",
                            Classes::ContentLeft.to_string(),
                            Classes::ContentCenterY.to_string(),
                            Classes::Wrapped.to_string(),
                        )),
                        Attribute::Attr(html::attributes::style(
                            "margin".to_string(),
                            format!("{}px {}px", half_y, half_x),
                        )),
                        Attribute::Attr(html::attributes::style(
                            "width".to_string(),
                            format!("calc(100% + {}px)", x),
                        )),
                        Attribute::Attr(html::attributes::style(
                            "height".to_string(),
                            format!("calc(100% + {}px)", y),
                        )),
                        Attribute::Style(
                            Flag::spacing(),
                            Style::Spacing(name, x, y),
                        ),
                    ],
                    Children::Unkeyed(children),
                )]),
            )
        }
    } else {
        let mut attr = vec![
            Attribute::html_class(format!(
                "{} {} {}",
                Classes::ContentLeft.to_string(),
                Classes::ContentCenterY.to_string(),
                Classes::Wrapped.to_string(),
            )),
            Attribute::Width(shrink()),
            Attribute::Height(shrink()),
        ];

        attr.extend(attrs);
        let attrs = attr;

        element(
            LayoutContext::AsRow,
            div(),
            attrs,
            Children::Unkeyed(children),
        )
    }
}

pub fn explain() -> Attribute {
    Attribute::html_class("explain".to_string())
}

/// A paragraph will layout all children as wrapped,
/// inline elements.
///
///     import Element exposing (el, paragraph, text)
///     import Element.Font as Font
///
///     view =
///         paragraph []
///             [ text "lots of text ...."
///             , el [ Font.bold ] (text "this is bold")
///             , text "lots of text ...."
///             ]
///
/// This is really useful when you want to markup text
/// by having some parts be bold, or some be links,
/// or whatever you so desire.
///
/// Also, if a child element has `alignLeft` or
/// `alignRight`, then it will be moved to that
/// side and the text will flow around it,
/// (ah yes, `float` behavior).
///
/// This makes it particularly easy to do something
/// like a [dropped capital](https://en.wikipedia.org/wiki/Initial).
///
///     import Element exposing (alignLeft, el, padding, paragraph, text)
///     import Element.Font as Font
///
///     view =
///         paragraph []
///             [ el
///                 [ alignLeft
///                 , padding 5
///                 ]
///                 (text "S")
///             , text "o much text ...."
///             ]
///
/// Which will look something like
///
/// ![A paragraph where the first letter is twice the height of the others](https://mdgriffith.gitbooks.io/style-elements/content/assets/Screen%20Shot%202017-08-25%20at%209.41.52%20PM.png)
///
/// **Note** `spacing` on a paragraph will set
/// the pixel spacing between lines.
pub fn paragraph(attrs: Vec<Attribute>, children: Vec<Element>) -> Element {
    let mut attr = vec![
        Attribute::Describe(Description::Paragraph),
        Attribute::Width(fill()),
        spacing(5),
    ];

    attr.extend(attrs);
    let attrs = attr;

    element(
        LayoutContext::AsParagraph,
        NodeName::div(),
        attrs,
        Children::Unkeyed(children),
    )
}

/// Now that we have a paragraph, we need some
/// way to attach a bunch of paragraph's together.
///
/// To do that we can use a `textColumn`.
///
/// The main difference between a `column` and
/// a `textColumn` is that `textColumn` will flow the
/// text around elements that have `alignRight` or
/// `alignLeft`, just like we just saw with paragraph.
///
/// In the following example, we have a `textColumn`
/// where one child has `alignLeft`.
///
///  Element.textColumn [ spacing 10, padding 10 ]
///      [ paragraph [] [ text "lots of text ...." ]
///      , el [ alignLeft ] none
///      , paragraph [] [ text "lots of text ...." ]
///      ]
///
/// Which will result in something like:
///
/// ![A text layout where an image is on the left.](https://mdgriffith.gitbooks.io/style-elements/content/assets/Screen%20Shot%202017-08-25%20at%208.42.39%20PM.png)
pub fn text_column(
    mut attrs: Vec<Attribute>,
    children: Vec<Element>,
) -> Element {
    attrs.push(width(min(500, max(750, fill()))));

    element(
        LayoutContext::AsTextColumn,
        div(),
        attrs,
        Children::Unkeyed(children),
    )
}

/// Both a source and a description are required for images.
///
/// The description is used for people using screen readers.
///
/// Leaving the description blank will cause the image to be
/// ignored by assistive technology. This can make sense for
/// images that are purely decorative and add no additional
/// information.
///
/// So, take a moment to describe your image as you would to
/// someone who has a harder time seeing.
///
pub fn image(
    attrs: Vec<Attribute>,
    src: String,
    description: String,
) -> Element {
    let img_attrs = attrs
        .iter()
        .filter(|a| match *a {
            Attribute::Width(_) => true,
            Attribute::Height(_) => true,
            _ => false,
        })
        .map(|x| x.clone())
        .collect::<Vec<Attribute>>();

    let mut img_attr = vec![
        Attribute::Attr(html::attributes::src(src)),
        Attribute::Attr(html::attributes::alt(description)),
    ];

    img_attr.extend(img_attrs);

    let img_attrs = img_attr;

    let mut attr = vec![Attribute::html_class(
        Classes::ImageContainer.to_string().to_string(),
    )];
    attr.extend(attrs);
    let attrs = attr;

    element(
        LayoutContext::AsEl,
        div(),
        attrs,
        Children::Unkeyed(vec![element(
            LayoutContext::AsEl,
            NodeName::NodeName("img".to_string()),
            img_attrs,
            Children::Unkeyed(vec![]),
        )]),
    )
}

pub fn link(attrs: Vec<Attribute>, url: String, label: Element) -> Element {
    let mut attr = vec![
        Attribute::Attr(html::attributes::href(url)),
        Attribute::Attr(html::attributes::rel(
            "noopener noreferrer".to_string(),
        )),
        width(shrink()),
        height(shrink()),
        Attribute::html_class(format!(
            "{} {} {}",
            Classes::ContentCenterX.to_string(),
            Classes::ContentCenterY.to_string(),
            Classes::Link.to_string(),
        )),
    ];

    attr.extend(attrs);
    let attrs = attr;

    element(
        LayoutContext::AsEl,
        NodeName::NodeName("a".to_string()),
        attrs,
        Children::Unkeyed(vec![label]),
    )
}

pub fn new_tablink(
    attrs: Vec<Attribute>,
    url: String,
    label: Element,
) -> Element {
    let mut attr = vec![
        Attribute::Attr(html::attributes::href(url)),
        Attribute::Attr(html::attributes::rel(
            "noopener noreferrer".to_string(),
        )),
        Attribute::Attr(html::attributes::target("_blank".to_string())),
        width(shrink()),
        height(shrink()),
        Attribute::html_class(format!(
            "{} {} {}",
            Classes::ContentCenterX.to_string(),
            Classes::ContentCenterY.to_string(),
            Classes::Link.to_string(),
        )),
    ];

    attr.extend(attrs);
    let attrs = attr;

    element(
        LayoutContext::AsEl,
        NodeName::NodeName("a".to_string()),
        attrs,
        Children::Unkeyed(vec![label]),
    )
}

pub fn download(attrs: Vec<Attribute>, url: String, label: Element) -> Element {
    let mut attr = vec![
        Attribute::Attr(html::attributes::href(url)),
        Attribute::Attr(html::attributes::download("".to_string())),
        width(shrink()),
        height(shrink()),
        Attribute::html_class(Classes::ContentCenterX.to_string().to_string()),
        Attribute::html_class(Classes::ContentCenterY.to_string().to_string()),
    ];

    attr.extend(attrs);
    let attrs = attr;

    element(
        LayoutContext::AsEl,
        NodeName::NodeName("a".to_string()),
        attrs,
        Children::Unkeyed(vec![label]),
    )
}

pub fn download_as(
    attrs: Vec<Attribute>,
    url: String,
    file_name: String,
    label: Element,
) -> Element {
    let mut attr = vec![
        Attribute::Attr(html::attributes::href(url)),
        Attribute::Attr(html::attributes::download(file_name)),
        width(shrink()),
        height(shrink()),
        Attribute::html_class(Classes::ContentCenterX.to_string().to_string()),
        Attribute::html_class(Classes::ContentCenterY.to_string().to_string()),
    ];

    attr.extend(attrs);
    let attrs = attr;

    element(
        LayoutContext::AsEl,
        NodeName::NodeName("a".to_string()),
        attrs,
        Children::Unkeyed(vec![label]),
    )
}

pub fn create_nearby(loc: Location, element: Element) -> Attribute {
    match element {
        Element::Empty => Attribute::None,
        _ => Attribute::Nearby(loc, element),
    }
}

pub fn below(element: Element) -> Attribute {
    create_nearby(Location::Below, element)
}

pub fn above(element: Element) -> Attribute {
    create_nearby(Location::Above, element)
}

pub fn on_right(element: Element) -> Attribute {
    create_nearby(Location::OnRight, element)
}

pub fn on_left(element: Element) -> Attribute {
    create_nearby(Location::OnLeft, element)
}

/// This will place an element in front of another.
///
/// **Note:** If you use this on a `layout` element,
/// it will place the element as fixed to the viewport
/// which can be useful for modals and overlays.
pub fn in_front(element: Element) -> Attribute {
    create_nearby(Location::InFront, element)
}

/// This will place an element between the background
/// and the content of an element.
pub fn behind_content(element: Element) -> Attribute {
    create_nearby(Location::Behind, element)
}

pub fn width(w: Length) -> Attribute {
    Attribute::Width(w)
}

pub fn height(w: Length) -> Attribute {
    Attribute::Height(w)
}

pub fn scale(n: f32) -> Attribute {
    Attribute::TransformComponent(
        Flag::scale(),
        TransformComponent::Scale(Coordinate { x: n, y: n, z: 1.0 }),
    )
}

/// Angle is given in radians. [Here are some conversion functions if you want to use another unit.](https://package.elm-lang.org/packages/elm/core/latest/Basics#degrees)
pub fn rotate(angle: f32) -> Attribute {
    Attribute::TransformComponent(
        Flag::rotate(),
        TransformComponent::Rotate(
            Coordinate {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            angle,
        ),
    )
}

pub fn move_up(y: f32) -> Attribute {
    Attribute::TransformComponent(
        Flag::move_y(),
        TransformComponent::MoveY(y.neg()),
    )
}

pub fn move_down(y: f32) -> Attribute {
    Attribute::TransformComponent(Flag::move_y(), TransformComponent::MoveY(y))
}

pub fn move_left(x: f32) -> Attribute {
    Attribute::TransformComponent(
        Flag::move_x(),
        TransformComponent::MoveX(x.neg()),
    )
}

pub fn move_right(x: f32) -> Attribute {
    Attribute::TransformComponent(Flag::move_x(), TransformComponent::MoveX(x))
}

pub fn padding(x: u32) -> Attribute {
    let f = x as f32;
    Attribute::Style(
        Flag::padding(),
        Style::Padding(format!("p-{}", x), f, f, f, f),
    )
}

pub fn padding_xy(x: u32, y: u32) -> Attribute {
    if x == y {
        let f = x as f32;

        Attribute::Style(
            Flag::padding(),
            Style::Padding(format!("p-{}", x), f, f, f, f),
        )
    } else {
        let s = format!("p-{}-{}", x, y);

        let x = x as f32;
        let y = y as f32;

        Attribute::Style(Flag::padding(), Style::Padding(s, y, x, y, x))
    }
}

pub fn padding_each(top: u32, right: u32, bottom: u32, left: u32) -> Attribute {
    if top == right && top == bottom && top == left {
        let f = top as f32;

        Attribute::Style(
            Flag::padding(),
            Style::Padding(format!("p-{}", top), f, f, f, f),
        )
    } else {
        Attribute::Style(
            Flag::padding(),
            Style::Padding(
                padding_class_name(top, right, bottom, left),
                top as f32,
                right as f32,
                bottom as f32,
                left as f32,
            ),
        )
    }
}

pub fn center_x() -> Attribute {
    Attribute::AlignX(HAlign::CenterX)
}

pub fn center_y() -> Attribute {
    Attribute::AlignY(VAlign::CenterY)
}

pub fn align_top() -> Attribute {
    Attribute::AlignY(VAlign::Top)
}

pub fn align_bottom() -> Attribute {
    Attribute::AlignY(VAlign::Bottom)
}

pub fn align_left() -> Attribute {
    Attribute::AlignX(HAlign::Left)
}

pub fn align_right() -> Attribute {
    Attribute::AlignX(HAlign::Right)
}

pub fn space_evenly() -> Attribute {
    Attribute::Class(
        Flag::spacing(),
        Classes::SpaceEvenly.to_string().to_string(),
    )
}

pub fn spacing(x: u32) -> Attribute {
    Attribute::Style(
        Flag::spacing(),
        Style::Spacing(spacing_class_name(x, x), x, x),
    )
}

/// In the majority of cases you'll just need to use `spacing`,
/// which will work as intended.
///
/// However for some layouts, like `textColumn`, you may want to
/// set a different spacing for the x axis compared to the y axis.
pub fn spacing_xy(x: u32, y: u32) -> Attribute {
    Attribute::Style(
        Flag::spacing(),
        Style::Spacing(spacing_class_name(x, y), x, y),
    )
}

/// Make an element transparent and have it ignore any mouse
/// or touch events, though it will stil take up space.
pub fn transparent(on: bool) -> Attribute {
    if on {
        Attribute::Style(
            Flag::transparency(),
            Style::Transparency("transparent".to_string(), 1.0),
        )
    } else {
        Attribute::Style(
            Flag::transparency(),
            Style::Transparency("visible".to_string(), 0.0),
        )
    }
}

/// A capped value between 0.0 and 1.0, where 0.0
/// is transparent and 1.0 is fully opaque.
///
/// Semantically equivalent to html opacity.
pub fn alpha(o: f32) -> Attribute {
    let t = 1.0 - o.clamp(0.0, 1.0);
    Attribute::Style(
        Flag::transparency(),
        Style::Transparency(format!("transparency-{}", t.float_class()), t),
    )
}

pub fn scrollbars() -> Attribute {
    Attribute::Class(
        Flag::overflow(),
        Classes::Scrollbars.to_string().to_string(),
    )
}

pub fn scrollbar_x() -> Attribute {
    Attribute::Class(
        Flag::overflow(),
        Classes::ScrollbarsX.to_string().to_string(),
    )
}

pub fn scrollbar_y() -> Attribute {
    Attribute::Class(
        Flag::overflow(),
        Classes::ScrollbarsY.to_string().to_string(),
    )
}

pub fn clip() -> Attribute {
    Attribute::Class(Flag::overflow(), Classes::Clip.to_string().to_string())
}

pub fn clip_x() -> Attribute {
    Attribute::Class(Flag::overflow(), Classes::ClipX.to_string().to_string())
}

pub fn clip_y() -> Attribute {
    Attribute::Class(Flag::overflow(), Classes::ClipY.to_string().to_string())
}

/// Set the cursor to be a pointing hand when it's hovering over this element.
pub fn pointer() -> Attribute {
    Attribute::Class(
        Flag::cursor(),
        Classes::CursorPointer.to_string().to_string(),
    )
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum DeviceClass {
    Phone,
    Tablet,
    Desktop,
    BigDesktop,
}

impl Default for DeviceClass {
    fn default() -> Self {
        DeviceClass::Desktop
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum Orientation {
    Portrait,
    Landscape,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Landscape
    }
}

#[derive(Debug, Default, PartialOrd, PartialEq, Clone, Copy)]
pub struct Device {
    class: DeviceClass,
    orientation: Orientation,
}

/// Takes in a Window.Size and returns a device
/// profile which can be used for responsiveness.
///
/// If you have more detailed concerns around
/// responsiveness, it probably makes sense to copy
/// this function into your codebase and modify as
/// needed.
pub fn classify_device(w: u32, h: u32) -> Device {
    let long_side = cmp::max(w, h);
    let short_side = cmp::min(w, h);

    let class = if short_side < 600 {
        DeviceClass::Phone
    } else if long_side <= 1200 {
        DeviceClass::Tablet
    } else if long_side > 1200 && long_side <= 1920 {
        DeviceClass::Desktop
    } else {
        DeviceClass::BigDesktop
    };

    let orientation = if w < h {
        Orientation::Portrait
    } else {
        Orientation::Landscape
    };

    Device { orientation, class }
}

/// When designing it's nice to use a modular scale
/// to set spacial rythms.
///
///    scaled =
///        Element.modular 16 1.25
///
/// A modular scale starts with a number, and multiplies
/// it by a ratio a number of times.
///
/// Then, when setting font sizes you can use:
///
///     Font.size (scaled 1) -- results in 16
///
///     Font.size (scaled 2) -- 16 * 1.25 results in 20
///
///     Font.size (scaled 4) -- 16 * 1.25 ^ (4 - 1) results in 31.25
///
/// We can also provide negative numbers to scale below 16px.
///
///     Font.size (scaled -1) -- 16 * 1.25 ^ (-1) results in 12.8
pub fn modular(normal: f32, ratio: f32, rescale: i32) -> f32 {
    if rescale == 0 {
        normal
    } else if rescale < 0 {
        (normal * ratio).powf(rescale as f32)
    } else {
        (normal * ratio).powf((rescale - 1) as f32)
    }
}

pub fn mouse_over(attrs: Vec<Attribute>) -> Attribute {
    Attribute::Style(
        Flag::hover(),
        Style::PseudoSelector(PseudoClass::Hover, unwrap_decorations(attrs)),
    )
}

pub fn mouse_down(attrs: Vec<Attribute>) -> Attribute {
    Attribute::Style(
        Flag::active(),
        Style::PseudoSelector(PseudoClass::Active, unwrap_decorations(attrs)),
    )
}

pub fn focused(attrs: Vec<Attribute>) -> Attribute {
    Attribute::Style(
        Flag::focus(),
        Style::PseudoSelector(PseudoClass::Focus, unwrap_decorations(attrs)),
    )
}
