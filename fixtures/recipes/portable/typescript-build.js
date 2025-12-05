// Simulated TypeScript build recipe
// PLASMA output "dist/"
// PLASMA cache ttl="1h"

console.log("[plasma] Building simulated TypeScript project...");

// Simulated TypeScript files
const simulatedFiles = ["src/index.ts", "src/utils.ts", "src/types.ts"];
console.log("[plasma] Found", simulatedFiles.length, "TypeScript files");

// Simulate compilation by creating .js files (parent directories created automatically)
for (const tsFile of simulatedFiles) {
  const jsFile = "dist/" + tsFile.replace("src/", "").replace(".ts", ".js");
  const content = "// Compiled from " + tsFile + "\nexport default {};\n";
  await Plasma.writeFile(jsFile, content);
}

// Generate manifest
const manifest = {
  files: simulatedFiles.length,
  environment: "development",
  timestamp: new Date().toISOString()
};
await Plasma.writeFile("dist/manifest.json", JSON.stringify(manifest, null, 2));

console.log("[plasma] Compiled", simulatedFiles.length, "files -> dist/");
