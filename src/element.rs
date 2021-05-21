use crate::{
    flag::{Field, Flag},
    model::{
        div, element, extract_spacing_and_padding, html,
        padding_class_name_float, render_root, root_style, spacing_class_name,
        virtual_dom as vdom, virtual_dom::Node, Attribute, Children, Color,
        Description, Element, FocusStyle, HoverSetting, LayoutContext, Length,
        NodeName, Opt, RenderMode, Style,
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

/// This is a special attribute that counts as both a Attribute and a Decoration
type Attr = Attribute;

/// Only decorations
type Decoration = Attribute;

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

pub fn width(w: Length) -> Attribute {
    Attribute::Width(w)
}

pub fn height(w: Length) -> Attribute {
    Attribute::Height(w)
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
///
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

pub fn spacing(x: u8) -> Attribute {
    Attribute::Style(
        Flag::spacing(),
        Style::Spacing(spacing_class_name(x, x), x, x),
    )
}
