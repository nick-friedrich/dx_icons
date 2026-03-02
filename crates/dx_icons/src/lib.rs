//! # dx_icons
//!
//! Icon components for [Dioxus](https://dioxuslabs.com) — Feather, Lucide, Heroicons, Tabler,
//! Simple Icons and more.
//!
//! ## Usage
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! dx_icons = { version = "0.1", features = ["lucide"] }
//! ```
//!
//! Then use in your Dioxus app:
//!
//! ```rust,no_run
//! use dx_icons::lucide::*;
//!
//! rsx! {
//!     Icon { icon: LucideIcon::Home }
//!     IconSearch { size: 32 }
//! };
//! ```
//!
//! ## Available Icon Sets
//!
//! Enable via feature flags:
//!
//! | Feature     | Crate                | Icons  | Description                   |
//! |-------------|----------------------|--------|-------------------------------|
//! | `feather`   | `dx_icons_feather`   | 287    | Feather Icons                 |
//! | `lucide`    | `dx_icons_lucide`    | ~1700  | Lucide Icons (Feather fork)   |
//! | `heroicons` | `dx_icons_heroicons` | 324×2  | Heroicons (outline + solid)   |
//! | `tabler`    | `dx_icons_tabler`    | ~5000  | Tabler Icons                  |
//! | `simple`    | `dx_icons_simple`    | ~3400  | Simple Icons (brand logos)    |

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
