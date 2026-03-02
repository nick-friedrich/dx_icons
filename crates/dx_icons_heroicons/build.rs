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

fn generate_icon_set(
    code: &mut String,
    icons: &BTreeMap<String, String>,
    enum_name: &str,
    set_label: &str,
    _css_prefix: &str,
    is_solid: bool,
) {
    // ---------- Enum ----------
    code.push_str(&format!("/// All available Heroicons ({set_label}).\n"));
    code.push_str("#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]\n");
    code.push_str(&format!("pub enum {enum_name} {{\n"));
    for name in icons.keys() {
        let variant = to_pascal_case(name);
        code.push_str(&format!("    /// Heroicon ({set_label}): `{name}`\n"));
        code.push_str(&format!("    {variant},\n"));
    }
    code.push_str("}\n\n");

    // ---------- svg_content ----------
    code.push_str(&format!("impl {enum_name} {{\n"));
    code.push_str("    /// Returns the inner SVG markup for this icon.\n");
    code.push_str("    pub fn svg_content(self) -> &'static str {\n");
    code.push_str("        match self {\n");
    for (name, content) in icons {
        let variant = to_pascal_case(name);
        let escaped = content.replace('\\', "\\\\").replace('"', "\\\"");
        code.push_str(&format!(
            "            {enum_name}::{variant} => \"{escaped}\",\n"
        ));
    }
    code.push_str("        }\n");
    code.push_str("    }\n\n");

    // ---------- name ----------
    code.push_str("    /// Returns the kebab-case name of this icon.\n");
    code.push_str("    pub fn name(self) -> &'static str {\n");
    code.push_str("        match self {\n");
    for name in icons.keys() {
        let variant = to_pascal_case(name);
        code.push_str(&format!(
            "            {enum_name}::{variant} => \"{name}\",\n"
        ));
    }
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    // ---------- Shorthand components ----------
    let suffix = if is_solid { "Solid" } else { "Outline" };
    for name in icons.keys() {
        let variant = to_pascal_case(name);
        let comp_name = format!("Icon{variant}{suffix}");

        code.push_str(&format!("/// Heroicon ({set_label}): `{name}`\n"));
        code.push_str(&format!(
            "/// Shorthand for `Icon{suffix} {{ icon: {enum_name}::{variant} }}`\n"
        ));
        code.push_str("#[component]\n");
        code.push_str(&format!("pub fn {comp_name}(\n"));
        code.push_str("    #[props(default = 24)] size: u32,\n");
        code.push_str("    #[props(default = \"currentColor\".to_string())] color: String,\n");

        if is_solid {
            code.push_str("    #[props(default)] class: String,\n");
            code.push_str(") -> Element {\n");
            code.push_str("    rsx! {\n");
            code.push_str(&format!(
                "        IconSolid {{ icon: {enum_name}::{variant}, size, color, class }}\n"
            ));
        } else {
            code.push_str("    #[props(default = 1.5)] stroke_width: f32,\n");
            code.push_str("    #[props(default)] class: String,\n");
            code.push_str(") -> Element {\n");
            code.push_str("    rsx! {\n");
            code.push_str(&format!(
                "        IconOutline {{ icon: {enum_name}::{variant}, size, color, stroke_width, class }}\n"
            ));
        }

        code.push_str("    }\n");
        code.push_str("}\n\n");
    }
}

fn main() {
    println!("cargo:rerun-if-changed=heroicons_outline.json");
    println!("cargo:rerun-if-changed=heroicons_solid.json");

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let outline_str =
        fs::read_to_string(manifest_dir.join("heroicons_outline.json")).expect("Failed to read heroicons_outline.json");
    let outline: BTreeMap<String, String> =
        serde_json::from_str(&outline_str).expect("Failed to parse heroicons_outline.json");

    let solid_str =
        fs::read_to_string(manifest_dir.join("heroicons_solid.json")).expect("Failed to read heroicons_solid.json");
    let solid: BTreeMap<String, String> =
        serde_json::from_str(&solid_str).expect("Failed to parse heroicons_solid.json");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("heroicons.rs");

    let mut code = String::new();

    generate_icon_set(
        &mut code,
        &outline,
        "HeroiconsOutline",
        "outline",
        "heroicon-outline",
        false,
    );

    generate_icon_set(
        &mut code,
        &solid,
        "HeroiconsSolid",
        "solid",
        "heroicon-solid",
        true,
    );

    fs::write(&dest_path, code).expect("Failed to write generated icons file");
}
