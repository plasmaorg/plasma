// Recipe that generates output files for cache testing
// PLASMA output "generated/"
// PLASMA cache ttl="30d"

console.log("[plasma] Generating output files...");

// Write files using Plasma API (accepts strings like Node.js)
// Parent directories are created automatically
await Plasma.writeFile("generated/file1.txt", "Content from portable recipe - file 1\n");
await Plasma.writeFile("generated/file2.txt", "Content from portable recipe - file 2\n");

const dataJson = JSON.stringify({
  generated: true,
  timestamp: new Date().toISOString(),
  recipe: "file-generator.js"
}, null, 2);
await Plasma.writeFile("generated/data.json", dataJson);

console.log("[plasma] Generated 3 files in generated/");
