#!/usr/bin/env node

// Converts Lucide's icon-nodes.json format to flat SVG string format.
//
// Usage:
//   node convert_lucide.js                    # fetches from unpkg
//   cat icon-nodes.json | node convert_lucide.js  # reads from stdin

const ICON_NODES_URL = "https://unpkg.com/lucide-static/icon-nodes.json";

function nodeToSvgString(node) {
  const [tag, attrs] = node;
  const attrStr = Object.entries(attrs)
    .map(([k, v]) => `${k}="${v}"`)
    .join(" ");
  return `<${tag} ${attrStr} />`;
}

function convertIconNodes(iconNodes) {
  const result = {};
  for (const [name, nodes] of Object.entries(iconNodes)) {
    result[name] = nodes.map(nodeToSvgString).join("");
  }
  return result;
}

async function readStdin() {
  const chunks = [];
  for await (const chunk of process.stdin) {
    chunks.push(chunk);
  }
  return Buffer.concat(chunks).toString("utf-8");
}

async function main() {
  let raw;
  const useStdin = process.argv.includes("--stdin");

  if (useStdin) {
    // Read from stdin
    raw = await readStdin();
    console.error("Read icon-nodes.json from stdin");
  } else {
    // Fetch from URL
    const url = process.argv[2] || ICON_NODES_URL;
    console.error(`Fetching icon-nodes.json from ${url}...`);
    const resp = await fetch(url);
    if (!resp.ok) {
      console.error(`Failed to fetch: ${resp.status} ${resp.statusText}`);
      process.exit(1);
    }
    raw = await resp.text();
    console.error(`Fetched icon-nodes.json (${raw.length} bytes)`);
  }

  const iconNodes = JSON.parse(raw);
  const converted = convertIconNodes(iconNodes);
  const count = Object.keys(converted).length;

  console.error(`Converted ${count} icons`);
  process.stdout.write(JSON.stringify(converted, null, 2) + "\n");
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
