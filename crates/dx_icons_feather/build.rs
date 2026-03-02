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
    let dest_path = Path::new(&out_dir).join("feather_icons.rs");

    let mut code = String::new();

    // ---------- FeatherIcon enum ----------
    code.push_str("/// All available Feather icons.\n");
    code.push_str("#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]\n");
    code.push_str("pub enum FeatherIcon {\n");
    for name in icons.keys() {
        let variant = to_pascal_case(name);
        code.push_str(&format!("    /// Feather icon: `{name}`\n"));
        code.push_str(&format!("    {variant},\n"));
    }
    code.push_str("}\n\n");

    // ---------- FeatherIcon::svg_content ----------
    code.push_str("impl FeatherIcon {\n");
    code.push_str("    /// Returns the inner SVG markup for this icon.\n");
    code.push_str("    pub fn svg_content(self) -> &'static str {\n");
    code.push_str("        match self {\n");
    for (name, content) in &icons {
        let variant = to_pascal_case(name);
        let escaped = content.replace('\\', "\\\\").replace('"', "\\\"");
        code.push_str(&format!(
            "            FeatherIcon::{variant} => \"{escaped}\",\n"
        ));
    }
    code.push_str("        }\n");
    code.push_str("    }\n\n");

    // ---------- FeatherIcon::name ----------
    code.push_str("    /// Returns the kebab-case name of this icon.\n");
    code.push_str("    pub fn name(self) -> &'static str {\n");
    code.push_str("        match self {\n");
    for name in icons.keys() {
        let variant = to_pascal_case(name);
        code.push_str(&format!(
            "            FeatherIcon::{variant} => \"{name}\",\n"
        ));
    }
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    // ---------- Individual icon components ----------
    // Each gets its own component name that delegates to Icon.
    for name in icons.keys() {
        let variant = to_pascal_case(name);
        let comp_name = format!("Icon{variant}");

        code.push_str(&format!("/// Feather icon: `{name}`\n"));
        code.push_str(&format!("///\n"));
        code.push_str(&format!("/// Shorthand for `Icon {{ icon: FeatherIcon::{variant} }}`\n"));
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
        code.push_str("    }\n");
        code.push_str("}\n\n");
    }

    fs::write(&dest_path, code).expect("Failed to write generated icons file");
}
