#!/usr/bin/env python3
"""
Convert Heroicons SVG files to JSON format for dx_icons.

Reads SVG files from a heroicons npm package extraction and produces
JSON files mapping icon names to their inner SVG content.

Usage:
    python3 scripts/convert_heroicons.py

Expects the heroicons package to be extracted at /tmp/heroicons-pack/package/
"""

import json
import os
import re
import sys

PACKAGE_DIR = "/tmp/heroicons-pack/package"
OUTPUT_DIR = "/home/nick/dev/dx_icons/crates/dx_icons_heroicons"

VARIANTS = {
    "outline": os.path.join(PACKAGE_DIR, "24", "outline"),
    "solid": os.path.join(PACKAGE_DIR, "24", "solid"),
}


def extract_inner_svg(svg_content: str) -> str:
    """Extract the inner content between <svg ...> and </svg> tags."""
    # Match everything between the opening <svg ...> tag and closing </svg>
    match = re.search(r"<svg[^>]*>(.*?)</svg>", svg_content, re.DOTALL)
    if not match:
        return ""
    inner = match.group(1).strip()
    # Normalize whitespace: collapse multiple spaces/newlines into single space
    inner = re.sub(r"\s+", " ", inner)
    return inner


def icon_name_from_filename(filename: str) -> str:
    """Convert a filename like 'academic-cap.svg' to key 'academic-cap'."""
    return filename.removesuffix(".svg")


def process_variant(variant_name: str, svg_dir: str) -> dict:
    """Process all SVG files in a directory and return a dict of icon_name -> inner_svg."""
    icons = {}
    if not os.path.isdir(svg_dir):
        print(f"ERROR: Directory not found: {svg_dir}", file=sys.stderr)
        sys.exit(1)

    svg_files = sorted(f for f in os.listdir(svg_dir) if f.endswith(".svg"))
    for filename in svg_files:
        filepath = os.path.join(svg_dir, filename)
        with open(filepath, "r") as f:
            svg_content = f.read()
        inner = extract_inner_svg(svg_content)
        if not inner:
            print(f"WARNING: No inner content found in {filepath}", file=sys.stderr)
            continue
        name = icon_name_from_filename(filename)
        icons[name] = inner
    return icons


def main():
    os.makedirs(OUTPUT_DIR, exist_ok=True)

    for variant_name, svg_dir in VARIANTS.items():
        print(f"Processing {variant_name} icons from {svg_dir}...")
        icons = process_variant(variant_name, svg_dir)

        output_path = os.path.join(OUTPUT_DIR, f"heroicons_{variant_name}.json")
        with open(output_path, "w") as f:
            json.dump(icons, f, indent=2, sort_keys=True)
            f.write("\n")

        print(f"  -> Wrote {len(icons)} icons to {output_path}")

    print("Done!")


if __name__ == "__main__":
    main()
