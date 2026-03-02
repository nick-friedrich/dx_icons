#!/usr/bin/env node

// Converts Tabler Icons outline SVG files to a flat JSON map.
//
// Usage:
//   node convert_tabler.js [svg-directory]
//
// Defaults to reading from /tmp/tabler-icons/package/icons/outline/
// Outputs to crates/dx_icons_tabler/icons.json

const fs = require("fs");
const path = require("path");

const DEFAULT_SVG_DIR = "/tmp/tabler-icons/package/icons/outline";
const OUTPUT_PATH = path.resolve(
  __dirname,
  "../crates/dx_icons_tabler/icons.json"
);

// Attributes set on the outer <svg> that we strip from inner elements
// since they'll be inherited from the parent.
const INHERITED_DEFAULTS = {
  "stroke-width": "2",
  "stroke-linecap": "round",
  "stroke-linejoin": "round",
};

function extractInnerSvg(svgContent) {
  // Match everything between the opening <svg ...> tag and closing </svg>
  const match = svgContent.match(/<svg[^>]*>([\s\S]*?)<\/svg>/);
  if (!match) return "";

  let inner = match[1].trim();

  // Remove the invisible bounding-box rect that Tabler adds to every icon:
  //   <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
  inner = inner.replace(
    /<path\s+stroke="none"\s+d="M0 0h24v24H0z"\s+fill="none"\s*\/>/g,
    ""
  );

  // Strip attributes from inner elements that match the outer SVG defaults
  for (const [attr, defaultVal] of Object.entries(INHERITED_DEFAULTS)) {
    // Match attr="value" allowing flexible whitespace
    const re = new RegExp(`\\s+${attr}="${defaultVal}"`, "g");
    inner = inner.replace(re, "");
  }

  // Normalize whitespace: collapse runs of whitespace into single space,
  // then trim each element boundary
  inner = inner.replace(/\s+/g, " ").trim();

  return inner;
}

function iconNameFromFilename(filename) {
  return filename.replace(/\.svg$/, "");
}

function main() {
  const svgDir = process.argv[2] || DEFAULT_SVG_DIR;

  if (!fs.existsSync(svgDir)) {
    console.error(`ERROR: SVG directory not found: ${svgDir}`);
    process.exit(1);
  }

  const files = fs
    .readdirSync(svgDir)
    .filter((f) => f.endsWith(".svg"))
    .sort();

  console.error(`Found ${files.length} SVG files in ${svgDir}`);

  const icons = {};
  let skipped = 0;

  for (const file of files) {
    const filePath = path.join(svgDir, file);
    const content = fs.readFileSync(filePath, "utf-8");
    const inner = extractInnerSvg(content);

    if (!inner) {
      console.error(`WARNING: No inner content in ${file}`);
      skipped++;
      continue;
    }

    const name = iconNameFromFilename(file);
    icons[name] = inner;
  }

  const count = Object.keys(icons).length;
  console.error(`Converted ${count} icons (${skipped} skipped)`);

  // Ensure output directory exists
  const outputDir = path.dirname(OUTPUT_PATH);
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  fs.writeFileSync(OUTPUT_PATH, JSON.stringify(icons, null, 2) + "\n");
  console.error(`Wrote ${OUTPUT_PATH}`);

  // Print a few samples to stderr for verification
  const sampleKeys = Object.keys(icons).slice(0, 3);
  console.error("\nSample entries:");
  for (const key of sampleKeys) {
    console.error(`  "${key}": "${icons[key].substring(0, 100)}..."`);
  }
}

main();
