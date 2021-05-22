use crate::{
    element::rgb,
    model::{Attribute, Color, Description, Element},
};

/// Input elements have a lot of constraints!
///
/// We want all of our input elements to:
///
///   - _Always be accessible_
///   - _Behave intuitively_
///   - _Be completely restyleable_
///
/// While these three goals may seem pretty obvious,
/// Html and CSS have made it surprisingly difficult to achieve!
///
/// And incredibly difficult for developers to remember all the tricks
/// necessary to make things work. If you've every tried to make a `<textarea>`
/// be the height of it's content or restyle a radio button while maintaining
/// accessibility, you may be familiar.
///
/// This module is intended to be accessible by default.
/// You shouldn't have to wade through docs, articles,
/// and books to find out [exactly how accessible your html actually is](https://www.powermapper.com/tests/screen-readers/aria/index.html).
///
///
/// # Focus Styling
///
/// All Elements can be styled on focus by using [`Element.focusStyle`](Element#focusStyle)
/// to set a global focus style or [`Element.focused`](Element#focused)
/// to set a focus style individually for an element.
///
/// @docs focusedOnLoad
///
///
/// # Buttons
///
/// @docs button
///
///
/// # Checkboxes
///
/// A checkbox requires you to store a `Bool` in your model.
///
/// This is also the first input element that has a [`required label`](#Label).
///
///     import Element exposing (text)
///     import Element.Input as Input
///
///     type Msg
///         = GuacamoleChecked Bool
///
///     view model =
///         Input.checkbox []
///             { onChange = GuacamoleChecked
///             , icon = Input.defaultCheckbox
///             , checked = model.guacamole
///             , label =
///                 Input.labelRight []
///                     (text "Do you want Guacamole?")
///             }
///
/// @docs checkbox, defaultCheckbox
///
///
/// # Text
///
/// @docs text, multiline
///
/// @docs Placeholder, placeholder
///
///
/// ## Text with autofill
///
/// If we want to play nicely with a browser's ability
/// to autofill a form, we need to be able to give it
/// a hint about what we're expecting.
///
/// The following inputs are very similar to `Input.text`,
/// but they give the browser a hint to allow autofill to
/// work correctly.
///
/// @docs username, newPassword, currentPassword, email, search, spellChecked
///
///
/// # Sliders
///
/// A slider is great for choosing between a range of numerical values.
///
///   - **thumb** - The icon that you click and drag to change the value.
///   - **track** - The line behind the thumb denoting where you can slide to.
///
/// @docs slider, Thumb, thumb, defaultThumb
///
///
/// # Radio Selection
///
/// The fact that we still call this a radio selection is fascinating.
/// I can't remember the last time I actually used an honest-to-goodness
/// button on a radio. Chalk it up along with the floppy disk save icon
/// or the word [Dashboard](https://en.wikipedia.org/wiki/Dashboard).
///
/// Perhaps a better name would be `Input.chooseOne`, because this
/// allows you to select one of a set of options!
///
/// Nevertheless, here we are. Here's how you put one together
///
///     Input.radio
///         [ padding 10
///         , spacing 20
///         ]
///         { onChange = ChooseLunch
///         , selected = Just model.lunch
///         , label = Input.labelAbove [] (text "Lunch")
///         , options =
///             [ Input.option Burrito (text "Burrito")
///             , Input.option Taco (text "Taco!")
///             , Input.option Gyro (text "Gyro")
///             ]
///         }
///
/// **Note** we're using `Input.option`, which will render the
/// default radio icon you're probably used to. If you want
/// compeltely custom styling, use `Input.optionWith`!
///
/// @docs radio, radioRow, Option, option, optionWith, OptionState
///
///
/// # Labels
///
/// Every input has a required `Label`.
///
/// @docs Label, labelAbove, labelBelow, labelLeft, labelRight, labelHidden
///
///
/// # Form Elements
///
/// You might be wondering where something like `<form>` is.
///
/// What I've found is that most people who want `<form>`
/// usually want it for the [implicit submission behavior](https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#implicit-submission)
/// or to be clearer, they want to do something when the `Enter` key is pressed.
///
/// Instead of implicit submission behavior, [try making an `onEnter` event handler like in this Ellie Example](https://ellie-app.com/5X6jBKtxzdpa1).
/// Then everything is explicit!
///
/// And no one has to look up obtuse html documentation to
/// understand the behavior of their code :).
///
/// # File Inputs
///
/// Presently, elm-ui does not expose a replacement for
/// `<input type="file">`; in the meantime, an `Input.button`
/// and `elm/file`'s `File.Select` may meet your needs.
///
///
/// # Disabling Inputs
///
/// You also might be wondering how to disable an input.
///
/// Disabled inputs can be a little problematic for user experience,
/// and doubly so for accessibility. This is because it's now your
/// priority to inform the user _why_ some field is disabled.
///
/// If an input is truly disabled, meaning it's not focusable or
/// doesn't send off a `Msg`, you actually lose your ability to
/// help the user out! For those wary about accessibility [this is a big problem.](https://ux.stackexchange.com/questions/103239/should-disabled-elements-be-focusable-for-accessibility-purposes)
///
/// Here are some alternatives to think about that don't
/// involve explicitly disabling an input.
///
/// **Disabled Buttons** - Change the `Msg` it fires, the text
/// that is rendered, and optionally set a `Region.description`
/// which will be available to screen readers.
///
///     import Element.Input as Input
///     import Element.Region as Region
///
///     myButton ready =
///         if ready then
///             Input.button
///                 [ Background.color blue
///                 ]
///                 { onPress =
///                     Just SaveButtonPressed
///                 , label =
///                     text "Save blog post"
///                 }
///
///         else
///             Input.button
///                 [ Background.color grey
///                 , Region.description
///                     "A publish date is required before saving a blogpost."
///                 ]
///                 { onPress =
///                     Just DisabledSaveButtonPressed
///                 , label =
///                     text "Save Blog "
///                 }
///
/// Consider showing a hint if `DisabledSaveButtonPressed` is sent.
///
/// For other inputs such as `Input.text`, consider simply rendering
/// it in a normal `paragraph` or `el` if it's not editable.
///
/// Alternatively, see if it's reasonable to _not_ display an input
/// if you'd normally disable it. Is there an option where it's
/// only visible when it's editable?
pub struct Placeholder(Vec<Attribute>, Element);

pub fn white() -> Color {
    rgb(1.0, 1.0, 1.0)
}

pub fn dark_grey() -> Color {
    rgb(186.0 / 255.0, 189.0 / 255.0, 182.00 / 255.0)
}

pub fn charcoal() -> Color {
    rgb((136.0 / 255.0), (138.0 / 255.0), (133.0 / 255.0))
}

pub fn placeholder(attrs: Vec<Attribute>, el: Element) -> Placeholder {
    Placeholder(attrs, el)
}

pub enum LabelLocation {
    OnRight,
    OnLeft,
    Above,
    Below,
}

pub enum Label {
    Label(LabelLocation, Vec<Attribute>, Element),
    HiddenLabel(String),
}

impl Label {
    pub fn is_stacked(label: Label) -> bool {
        match label {
            Label::Label(loc, _, _) => match loc {
                LabelLocation::OnRight => false,
                LabelLocation::OnLeft => false,
                LabelLocation::Above => true,
                LabelLocation::Below => true,
            },
            Label::HiddenLabel(_) => true,
        }
    }
}

pub fn label_right(attrs: Vec<Attribute>, el: Element) -> Label {
    Label::Label(LabelLocation::OnRight, attrs, el)
}

pub fn label_left(attrs: Vec<Attribute>, el: Element) -> Label {
    Label::Label(LabelLocation::OnLeft, attrs, el)
}

pub fn label_above(attrs: Vec<Attribute>, el: Element) -> Label {
    Label::Label(LabelLocation::Above, attrs, el)
}

pub fn label_below(attrs: Vec<Attribute>, el: Element) -> Label {
    Label::Label(LabelLocation::Below, attrs, el)
}

/// Sometimes you may need to have a label which is not visible,
/// but is still accessible to screen readers.
///
/// Seriously consider a visible label before using this.
///
/// The situations where a hidden label makes sense:
///
///  - A searchbar with a `search` button right next to it.
///  - A `table` of inputs where the header gives the label.
///
/// Basically, a hidden label works when there are other
/// contextual clues that sighted people can pick up on.
pub fn label_hidden(label: String) -> Label {
    Label::HiddenLabel(label)
}

pub fn hidden_label_attr(label: Label) -> Attribute {
    match label {
        Label::HiddenLabel(text_label) => {
            Attribute::Describe(Description::Label(text_label))
        }
        Label::Label(_, _, _) => Attribute::None,
    }
}
