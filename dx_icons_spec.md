# dx_icons — Dioxus Icon Components

## Overview

`dx_icons` is a collection of crates that provide popular open-source icon sets as native Dioxus 0.7 components. Each icon set is its own crate, with a meta-crate that re-exports them behind feature flags. Publish to crates.io under the `dx_icons` name.

**Repository:** new standalone repo (not part of clawless)
**License:** MIT (all icon sets used are also MIT-compatible)
**Rust edition:** 2024
**Dioxus version:** 0.7.x

---

## Crate Structure

```
dx_icons/
  Cargo.toml              # workspace root
  LICENSE
  README.md
  crates/
    dx_icons/             # meta-crate that re-exports icon set crates
      Cargo.toml
      src/lib.rs
    dx_icons_feather/     # Feather Icons (287 icons)
      Cargo.toml
      build.rs
      icons.json
      src/lib.rs
    dx_icons_lucide/      # Lucide Icons (~1500 icons, Feather successor)
      Cargo.toml
      build.rs
      icons.json
      src/lib.rs
    dx_icons_heroicons/   # Heroicons (300+ icons, outline + solid)
      Cargo.toml
      build.rs
      icons.json
      src/lib.rs
    dx_icons_simple/      # Simple Icons (brand logos, 3000+)
      Cargo.toml
      build.rs
      icons.json
      src/lib.rs
    dx_icons_tabler/      # Tabler Icons (4000+)
      Cargo.toml
      build.rs
      icons.json
      src/lib.rs
```

---

## Meta-Crate (`dx_icons`)

Re-exports all icon set crates behind feature flags. Users pick what they need.

```toml
# crates/dx_icons/Cargo.toml
[package]
name = "dx_icons"
version = "0.1.0"
edition = "2024"
description = "Icon components for Dioxus — Feather, Lucide, Heroicons, and more"
license = "MIT"
repository = "https://github.com/nick-friedrich/dx_icons"
keywords = ["dioxus", "icons", "svg", "ui", "components"]
categories = ["gui", "web-programming"]

[features]
default = ["feather"]
feather = ["dep:dx_icons_feather"]
lucide = ["dep:dx_icons_lucide"]
heroicons = ["dep:dx_icons_heroicons"]
simple = ["dep:dx_icons_simple"]
tabler = ["dep:dx_icons_tabler"]

[dependencies]
dx_icons_feather = { version = "0.1", path = "../dx_icons_feather", optional = true }
dx_icons_lucide = { version = "0.1", path = "../dx_icons_lucide", optional = true }
dx_icons_heroicons = { version = "0.1", path = "../dx_icons_heroicons", optional = true }
dx_icons_simple = { version = "0.1", path = "../dx_icons_simple", optional = true }
dx_icons_tabler = { version = "0.1", path = "../dx_icons_tabler", optional = true }
```

```rust
// crates/dx_icons/src/lib.rs
#[cfg(feature = "feather")]
pub use dx_icons_feather as feather;

#[cfg(feature = "lucide")]
pub use dx_icons_lucide as lucide;

#[cfg(feature = "heroicons")]
pub use dx_icons_heroicons as heroicons;

#[cfg(feature = "simple")]
pub use dx_icons_simple as simple;

#[cfg(feature = "tabler")]
pub use dx_icons_tabler as tabler;
```

---

## Per-Icon-Set Crate Pattern

Each icon set crate follows the same pattern. Here is the existing working implementation from the Feather crate as the reference for all others.

### Build Script (`build.rs`)

Reads an `icons.json` file (format: `{ "icon-name": "<svg inner content>" }`) and generates:

1. **An enum** (e.g. `FeatherIcon`) with one variant per icon in PascalCase
2. **`svg_content(self) -> &'static str`** — returns the inner SVG markup
3. **`name(self) -> &'static str`** — returns the kebab-case name
4. **Individual shorthand components** (e.g. `IconHome`, `IconSearch`) that delegate to the shared `Icon` component

Reference implementation (proven working):

```rust
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;

fn to_pascal_case(s: &str) -> String {
    s.split('-')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().to_string() + &chars.as_str().to_lowercase(),
            }
        })
        .collect()
}

fn main() {
    println!("cargo:rerun-if-changed=icons.json");

    let json_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("icons.json");
    let json_str = fs::read_to_string(&json_path).expect("Failed to read icons.json");
    let icons: BTreeMap<String, String> =
        serde_json::from_str(&json_str).expect("Failed to parse icons.json");

    let out_dir = env::var("OUT_DIR").unwrap();
    // Use the icon set name in the filename, e.g. feather_icons.rs, lucide_icons.rs
    let dest_path = Path::new(&out_dir).join("feather_icons.rs");

    let mut code = String::new();

    // --- Enum ---
    code.push_str("#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]\n");
    code.push_str("pub enum FeatherIcon {\n");
    for name in icons.keys() {
        let variant = to_pascal_case(name);
        code.push_str(&format!("    /// Feather icon: `{name}`\n"));
        code.push_str(&format!("    {variant},\n"));
    }
    code.push_str("}\n\n");

    // --- svg_content ---
    code.push_str("impl FeatherIcon {\n");
    code.push_str("    pub fn svg_content(self) -> &'static str {\n");
    code.push_str("        match self {\n");
    for (name, content) in &icons {
        let variant = to_pascal_case(name);
        let escaped = content.replace('\\', "\\\\").replace('"', "\\\"");
        code.push_str(&format!(
            "            FeatherIcon::{variant} => \"{escaped}\",\n"
        ));
    }
    code.push_str("        }\n    }\n\n");

    // --- name ---
    code.push_str("    pub fn name(self) -> &'static str {\n");
    code.push_str("        match self {\n");
    for name in icons.keys() {
        let variant = to_pascal_case(name);
        code.push_str(&format!(
            "            FeatherIcon::{variant} => \"{name}\",\n"
        ));
    }
    code.push_str("        }\n    }\n}\n\n");

    // --- Shorthand components ---
    for name in icons.keys() {
        let variant = to_pascal_case(name);
        let comp_name = format!("Icon{variant}");
        code.push_str(&format!("/// Feather icon: `{name}`\n"));
        code.push_str("#[component]\n");
        code.push_str(&format!("pub fn {comp_name}(\n"));
        code.push_str("    #[props(default = 24)] size: u32,\n");
        code.push_str("    #[props(default = \"currentColor\".to_string())] color: String,\n");
        code.push_str("    #[props(default = 2.0)] stroke_width: f32,\n");
        code.push_str("    #[props(default)] class: String,\n");
        code.push_str("    #[props(default = false)] fill: bool,\n");
        code.push_str(") -> Element {\n");
        code.push_str("    rsx! {\n");
        code.push_str(&format!(
            "        Icon {{ icon: FeatherIcon::{variant}, size, color, stroke_width, class, fill }}\n"
        ));
        code.push_str("    }\n}\n\n");
    }

    fs::write(&dest_path, code).expect("Failed to write generated icons file");
}
```

### Library (`src/lib.rs`)

Defines the shared `Icon` component and includes the generated code:

```rust
use dioxus::prelude::*;

#[component]
pub fn Icon(
    icon: FeatherIcon,
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor".to_string())] color: String,
    #[props(default = 2.0)] stroke_width: f32,
    #[props(default)] class: String,
    #[props(default = false)] fill: bool,
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
            class: "feather feather-{icon_name} {class}",
            dangerous_inner_html: icon.svg_content(),
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/feather_icons.rs"));
```

### Icon Data (`icons.json`)

Flat JSON map of `"kebab-name" -> "<svg inner content>"`.

Sources:
- **Feather:** `https://unpkg.com/feather-icons/dist/icons.json` (287 icons)
- **Lucide:** `https://unpkg.com/lucide-static/dist/icons.json` or build from their GitHub repo
- **Heroicons:** extract from `https://github.com/tailwindlabs/heroicons` — needs a small script since they ship individual SVGs, not a single JSON. Has `outline` and `solid` variants (handle as separate enums or a `HeroiconsVariant` parameter).
- **Simple Icons:** `https://github.com/simple-icons/simple-icons` — brand logos, filled icons (no stroke), uses `fill` only. The `Icon` component's `fill` prop handles this.
- **Tabler:** `https://github.com/tabler/tabler-icons` — similar structure to Feather

---

## Shared Icon Props (all icon sets)

| Prop           | Type    | Default          | Description                  |
|----------------|---------|------------------|------------------------------|
| `icon`         | Enum    | (required)       | Which icon to render         |
| `size`         | `u32`   | `24`             | Width & height in px         |
| `color`        | `String`| `"currentColor"` | Stroke or fill color         |
| `stroke_width` | `f32`   | `2.0`            | SVG stroke width             |
| `class`        | `String`| `""`             | Extra CSS classes            |
| `fill`         | `bool`  | `false`          | Use fill instead of stroke   |

---

## User-Facing API

```rust
// With meta-crate
use dx_icons::feather::*;

rsx! {
    // Enum-based
    Icon { icon: FeatherIcon::Home }
    Icon { icon: FeatherIcon::Search, size: 32, color: "red" }

    // Shorthand components
    IconHome {}
    IconSearch { size: 32 }
}

// Or depend on a single icon set directly
use dx_icons_feather::*;
```

---

## Adaptation Notes for Non-Feather Sets

- **Heroicons** has `outline` and `solid` variants. Either ship two enums (`HeroiconsOutline`, `HeroiconsSolid`) or use a single enum with a `variant` prop on the `Icon` component. Two enums is simpler.
- **Simple Icons** are brand logos — single-path, filled, no stroke. The default `fill` prop should be `true` for this set, and `stroke_width` is irrelevant.
- **Tabler** has `outline` and `filled` variants, similar approach as Heroicons.
- **view_box** differs per set. Feather/Lucide use `0 0 24 24`. Heroicons use `0 0 24 24`. Simple Icons use `0 0 24 24`. Tabler uses `0 0 24 24`. If a set uses a different viewBox, make it a constant in that crate's `lib.rs`.

---

## Implementation Priority

1. **dx_icons_feather** — already built and working (copy from clawless reference)
2. **dx_icons_lucide** — highest value, superset of Feather with 1500+ icons
3. **dx_icons** meta-crate — wire up re-exports
4. **dx_icons_heroicons** — popular with Tailwind users
5. **dx_icons_tabler** — large set
6. **dx_icons_simple** — brand logos

---

## Future Enhancements

- `from_name(name: &str) -> Option<Enum>` for runtime icon lookup
- `aria_label: String` prop for accessibility
- `title: String` prop that renders `<title>` inside the SVG
- `spin: bool` / `animation: String` prop for loading spinners
- Downloadable icon gallery / demo page built with Dioxus
- CI pipeline: auto-update icon JSON from upstream repos on a schedule
