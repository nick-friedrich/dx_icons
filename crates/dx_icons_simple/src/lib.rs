use dioxus::prelude::*;

/// Renders a Simple Icon (brand logo) by enum variant.
///
/// Simple Icons are brand logos — they use fill only, no stroke.
///
/// # Example
/// ```rust,no_run
/// use dx_icons_simple::*;
///
/// rsx! {
///     Icon { icon: SimpleIcon::Github }
///     Icon { icon: SimpleIcon::Docker, size: 32, color: "#2496ED" }
/// };
/// ```
#[component]
pub fn Icon(
    /// Which icon to render.
    icon: SimpleIcon,
    /// Width and height in pixels. Defaults to 24.
    #[props(default = 24)]
    size: u32,
    /// Fill color. Defaults to `"currentColor"`.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Optional extra CSS class.
    #[props(default)]
    class: String,
) -> Element {
    let icon_name = icon.name();

    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "{color}",
            class: "simple-icon simple-icon-{icon_name} {class}",
            role: "img",
            dangerous_inner_html: icon.svg_content(),
        }
    }
}

// Include the generated icon enum and shorthand components.
include!(concat!(env!("OUT_DIR"), "/simple_icons.rs"));
