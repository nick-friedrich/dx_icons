use dioxus::prelude::*;

/// Renders a Lucide icon by enum variant.
///
/// # Example
/// ```rust,no_run
/// use dx_icons_lucide::*;
///
/// rsx! {
///     Icon { icon: LucideIcon::Home }
///     Icon { icon: LucideIcon::Search, size: 32, color: "red" }
/// };
/// ```
#[component]
pub fn Icon(
    /// Which icon to render.
    icon: LucideIcon,
    /// Width and height in pixels. Defaults to 24.
    #[props(default = 24)]
    size: u32,
    /// Stroke color. Defaults to `"currentColor"`.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Stroke width. Defaults to 2.
    #[props(default = 2.0)]
    stroke_width: f32,
    /// Optional extra CSS class.
    #[props(default)]
    class: String,
    /// Whether to apply `fill` instead of `stroke`. Defaults to `false`.
    #[props(default = false)]
    fill: bool,
) -> Element {
    let fill_val = if fill { &color } else { "none" };
    let stroke_val = if fill { "none" } else { &color };
    let icon_name = icon.name();

    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "{fill_val}",
            stroke: "{stroke_val}",
            stroke_width: "{stroke_width}",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: "lucide lucide-{icon_name} {class}",
            dangerous_inner_html: icon.svg_content(),
        }
    }
}

// Include the generated icon enum and shorthand components.
include!(concat!(env!("OUT_DIR"), "/lucide_icons.rs"));
