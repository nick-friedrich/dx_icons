#!/usr/bin/env node

// Converts Simple Icons SVG files to a flat JSON map.
//
// Usage:
//   node convert_simple.js /path/to/simple-icons/icons/
//
// Each SVG file is read, the inner content (between <svg ...> and </svg>) is
// extracted, and the <title>...</title> element is stripped. The filename
// (without .svg) becomes the kebab-case key.
//
// Output: {"icon-name": "<path d=\"...\" />", ...}

const fs = require("fs");
const path = require("path");

function extractSvgInner(svgContent) {
  // Remove the outer <svg ...> and </svg> tags
  const openTagEnd = svgContent.indexOf(">") + 1;
  const closeTagStart = svgContent.lastIndexOf("</svg>");
  if (openTagEnd === 0 || closeTagStart === -1) {
    return null;
  }
  let inner = svgContent.slice(openTagEnd, closeTagStart).trim();

  // Strip the <title>...</title> element
  inner = inner.replace(/<title>[^<]*<\/title>/, "").trim();

  return inner;
}

function main() {
  const iconsDir = process.argv[2];
  if (!iconsDir) {
    console.error("Usage: node convert_simple.js <icons-directory>");
    process.exit(1);
  }

  if (!fs.existsSync(iconsDir)) {
    console.error(`Directory not found: ${iconsDir}`);
    process.exit(1);
  }

  const files = fs.readdirSync(iconsDir).filter((f) => f.endsWith(".svg")).sort();
  console.error(`Found ${files.length} SVG files in ${iconsDir}`);

  const result = {};
  let skipped = 0;

  for (const file of files) {
    const filePath = path.join(iconsDir, file);
    const content = fs.readFileSync(filePath, "utf-8").trim();
    const inner = extractSvgInner(content);

    if (!inner) {
      console.error(`  Skipping ${file}: could not extract SVG inner content`);
      skipped++;
      continue;
    }

    // Key is the filename without .svg, already lowercase/kebab from simple-icons
    const key = file.replace(/\.svg$/, "");
    result[key] = inner;
  }

  const count = Object.keys(result).length;
  console.error(`Converted ${count} icons (${skipped} skipped)`);
  process.stdout.write(JSON.stringify(result, null, 2) + "\n");
}

main();
