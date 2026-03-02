# dx_icons

[![crates.io](https://img.shields.io/crates/v/dx_icons.svg)](https://crates.io/crates/dx_icons)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Icon components for [Dioxus](https://dioxuslabs.com) 0.7 — popular open-source icon sets as native Dioxus components.

## Icon Sets

| Crate | Icons | crates.io | Description |
|-------|-------|-----------|-------------|
| [`dx_icons_feather`](https://crates.io/crates/dx_icons_feather) | 287 | [![crates.io](https://img.shields.io/crates/v/dx_icons_feather.svg)](https://crates.io/crates/dx_icons_feather) | [Feather Icons](https://feathericons.com) |
| [`dx_icons_lucide`](https://crates.io/crates/dx_icons_lucide) | ~1700 | [![crates.io](https://img.shields.io/crates/v/dx_icons_lucide.svg)](https://crates.io/crates/dx_icons_lucide) | [Lucide Icons](https://lucide.dev) (Feather successor) |
| [`dx_icons_heroicons`](https://crates.io/crates/dx_icons_heroicons) | 324×2 | [![crates.io](https://img.shields.io/crates/v/dx_icons_heroicons.svg)](https://crates.io/crates/dx_icons_heroicons) | [Heroicons](https://heroicons.com) (outline + solid) |
| [`dx_icons_tabler`](https://crates.io/crates/dx_icons_tabler) | ~5000 | [![crates.io](https://img.shields.io/crates/v/dx_icons_tabler.svg)](https://crates.io/crates/dx_icons_tabler) | [Tabler Icons](https://tabler.io/icons) |
| [`dx_icons_simple`](https://crates.io/crates/dx_icons_simple) | ~3400 | [![crates.io](https://img.shields.io/crates/v/dx_icons_simple.svg)](https://crates.io/crates/dx_icons_simple) | [Simple Icons](https://simpleicons.org) (brand logos) |

## Installation

### Option 1: Meta-crate with feature flags

Pick only the icon sets you need:

```toml
[dependencies]
dx_icons = { version = "0.1", features = ["lucide"] }
```

Available features: `feather` (default), `lucide`, `heroicons`, `tabler`, `simple`

### Option 2: Depend on an icon set directly

```toml
[dependencies]
dx_icons_lucide = "0.1"
dx_icons_heroicons = "0.1"
```

## Quick Start

```rust
use dx_icons::lucide::*;

fn App() -> Element {
    rsx! {
        // Enum-based usage
        Icon { icon: LucideIcon::Home }
        Icon { icon: LucideIcon::Search, size: 32, color: "red" }

        // Shorthand components
        IconHome {}
        IconSearch { size: 32 }
    }
}
```

### Heroicons (outline + solid)

```rust
use dx_icons::heroicons::*;

rsx! {
    IconOutline { icon: HeroiconsOutline::Home }
    IconSolid { icon: HeroiconsSolid::Home, color: "blue" }

    // Shorthand components
    IconHomeOutline {}
    IconHomeSolid { size: 32 }
}
```

### Simple Icons (brand logos)

```rust
use dx_icons::simple::*;

rsx! {
    Icon { icon: SimpleIcon::Github }
    Icon { icon: SimpleIcon::Docker, size: 32, color: "#2496ED" }

    // Shorthand
    IconGithub {}
    IconDocker { size: 48 }
}
```

## Props

All icon components accept these props:

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `icon` | Enum | (required) | Which icon to render |
| `size` | `u32` | `24` | Width & height in px |
| `color` | `String` | `"currentColor"` | Stroke or fill color |
| `stroke_width` | `f32` | `2.0` | SVG stroke width |
| `class` | `String` | `""` | Extra CSS classes |
| `fill` | `bool` | `false` | Use fill instead of stroke |

> **Note:** Simple Icons (brand logos) only accept `size`, `color`, and `class` since they are always filled.
> Heroicons has separate `IconOutline` and `IconSolid` components with their own prop sets.

## License

MIT
