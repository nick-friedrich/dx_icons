use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;

fn to_pascal_case(s: &str) -> String {
    let result: String = s
        .split(|c: char| c == '-' || c == '_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().to_string() + &chars.as_str().to_lowercase(),
            }
        })
        .collect();
    // Rust identifiers cannot start with a digit — prefix with 'N' if needed.
    if result.starts_with(|c: char| c.is_ascii_digit()) {
        format!("N{result}")
    } else {
        result
    }
}

fn main() {
    println!("cargo:rerun-if-changed=icons.json");

    let json_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("icons.json");
    let json_str = fs::read_to_string(&json_path).expect("Failed to read icons.json");
    let icons: BTreeMap<String, String> =
        serde_json::from_str(&json_str).expect("Failed to parse icons.json");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("simple_icons.rs");

    let mut code = String::new();

    // ---------- SimpleIcon enum ----------
    code.push_str("/// All available Simple Icons (brand logos).\n");
    code.push_str("#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]\n");
    code.push_str("pub enum SimpleIcon {\n");
    for name in icons.keys() {
        let variant = to_pascal_case(name);
        code.push_str(&format!("    /// Simple icon: `{name}`\n"));
        code.push_str(&format!("    {variant},\n"));
    }
    code.push_str("}\n\n");

    // ---------- SimpleIcon::svg_content ----------
    code.push_str("impl SimpleIcon {\n");
    code.push_str("    /// Returns the inner SVG markup for this icon.\n");
    code.push_str("    pub fn svg_content(self) -> &'static str {\n");
    code.push_str("        match self {\n");
    for (name, content) in &icons {
        let variant = to_pascal_case(name);
        let escaped = content.replace('\\', "\\\\").replace('"', "\\\"");
        code.push_str(&format!(
            "            SimpleIcon::{variant} => \"{escaped}\",\n"
        ));
    }
    code.push_str("        }\n");
    code.push_str("    }\n\n");

    // ---------- SimpleIcon::name ----------
    code.push_str("    /// Returns the kebab-case name of this icon.\n");
    code.push_str("    pub fn name(self) -> &'static str {\n");
    code.push_str("        match self {\n");
    for name in icons.keys() {
        let variant = to_pascal_case(name);
        code.push_str(&format!(
            "            SimpleIcon::{variant} => \"{name}\",\n"
        ));
    }
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    // ---------- Individual icon components ----------
    // Simple Icons are brand logos — filled by default, no stroke.
    for name in icons.keys() {
        let variant = to_pascal_case(name);
        let comp_name = format!("Icon{variant}");

        code.push_str(&format!("/// Simple icon (brand logo): `{name}`\n"));
        code.push_str(&format!(
            "/// Shorthand for `Icon {{ icon: SimpleIcon::{variant} }}`\n"
        ));
        code.push_str("#[component]\n");
        code.push_str(&format!("pub fn {comp_name}(\n"));
        code.push_str("    #[props(default = 24)] size: u32,\n");
        code.push_str("    #[props(default = \"currentColor\".to_string())] color: String,\n");
        code.push_str("    #[props(default)] class: String,\n");
        code.push_str(") -> Element {\n");
        code.push_str("    rsx! {\n");
        code.push_str(&format!(
            "        Icon {{ icon: SimpleIcon::{variant}, size, color, class }}\n"
        ));
        code.push_str("    }\n");
        code.push_str("}\n\n");
    }

    fs::write(&dest_path, code).expect("Failed to write generated icons file");
}
