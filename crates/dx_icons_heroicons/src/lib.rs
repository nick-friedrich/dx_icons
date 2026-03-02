use dioxus::prelude::*;

/// Renders a Heroicons outline icon by enum variant.
///
/// Outline icons use stroke-based rendering (default stroke-width: 1.5).
///
/// # Example
/// ```rust,no_run
/// use dx_icons_heroicons::*;
///
/// rsx! {
///     IconOutline { icon: HeroiconsOutline::Home }
///     IconOutline { icon: HeroiconsOutline::MagnifyingGlass, size: 32, color: "red" }
/// };
/// ```
#[component]
pub fn IconOutline(
    /// Which outline icon to render.
    icon: HeroiconsOutline,
    /// Width and height in pixels. Defaults to 24.
    #[props(default = 24)]
    size: u32,
    /// Stroke color. Defaults to `"currentColor"`.
    #[props(default = "currentColor".to_string())]
    color: String,
    /// Stroke width. Defaults to 1.5.
    #[props(default = 1.5)]
    stroke_width: f32,
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
            fill: "none",
            stroke: "{color}",
            stroke_width: "{stroke_width}",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: "heroicon heroicon-outline heroicon-{icon_name} {class}",
            dangerous_inner_html: icon.svg_content(),
        }
    }
}

/// Renders a Heroicons solid icon by enum variant.
///
/// Solid icons use fill-based rendering.
///
/// # Example
/// ```rust,no_run
/// use dx_icons_heroicons::*;
///
/// rsx! {
///     IconSolid { icon: HeroiconsSolid::Home }
///     IconSolid { icon: HeroiconsSolid::MagnifyingGlass, size: 32, color: "blue" }
/// };
/// ```
#[component]
pub fn IconSolid(
    /// Which solid icon to render.
    icon: HeroiconsSolid,
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
            class: "heroicon heroicon-solid heroicon-{icon_name} {class}",
            dangerous_inner_html: icon.svg_content(),
        }
    }
}

// Include the generated icon enums and shorthand components.
include!(concat!(env!("OUT_DIR"), "/heroicons.rs"));
