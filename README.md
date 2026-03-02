# dx_icons

Icon components for [Dioxus](https://dioxuslabs.com) 0.7 — popular open-source icon sets as native Dioxus components.

## Icon Sets

| Crate | Icons | Description |
|-------|-------|-------------|
| `dx_icons_feather` | 287 | [Feather Icons](https://feathericons.com) |
| `dx_icons_lucide` | ~1700 | [Lucide Icons](https://lucide.dev) (Feather successor) |
| `dx_icons_heroicons` | 324×2 | [Heroicons](https://heroicons.com) (outline + solid) |
| `dx_icons_tabler` | ~5000 | [Tabler Icons](https://tabler.io/icons) |
| `dx_icons_simple` | ~3400 | [Simple Icons](https://simpleicons.org) (brand logos) |

## Quick Start

```toml
# Use the meta-crate with feature flags
[dependencies]
dx_icons = { version = "0.1", features = ["lucide"] }

# Or depend on an icon set directly
dx_icons_lucide = "0.1"
```

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
> Heroicons has separate `IconOutline` and `IconSolid` components.

## License

MIT
