// Demonstration of Plasma APIs available in portable recipes
// PLASMA cache ttl="1h"

console.log("[plasma] Demonstrating Plasma APIs...");

// Example 1: Using Plasma.glob() to find files
console.log("[plasma] Finding JavaScript files with Plasma.glob()...");
const jsFiles = await Plasma.glob("*.js");
console.log("[plasma] Found", jsFiles.length, "JavaScript file(s)");

// Example 2: Using Plasma.exists() to check file existence
console.log("[plasma] Checking file existence with Plasma.exists()...");
const selfExists = await Plasma.exists("cache-api-demo.js");
console.log("[plasma] cache-api-demo.js exists:", selfExists);

// Example 3: Using Plasma.hashFile() for content-addressed caching
if (jsFiles.length > 0) {
  console.log("[plasma] Computing file hash with Plasma.hashFile()...");
  const hash = await Plasma.hashFile(jsFiles[0]);
  console.log("[plasma]", jsFiles[0], "hash:", hash.substring(0, 16) + "...");
}

// Example 4: Using Plasma.readFile()
console.log("[plasma] Reading file with Plasma.readFile()...");
const selfContent = await Plasma.readFile("cache-api-demo.js");
console.log("[plasma] Read", selfContent.length, "bytes from cache-api-demo.js");

// Example 5: Using Plasma.exec() to run commands
console.log("[plasma] Running command with Plasma.exec()...");
const exitCode = await Plasma.exec("echo", ["Hello from Plasma!"]);
console.log("[plasma] Command exit code:", exitCode);

// Example 6: Using Plasma.writeFile() (accepts strings like Node.js)
// Parent directories are created automatically
console.log("[plasma] Writing file with Plasma.writeFile()...");
const manifest = {
  recipe: "cache-api-demo.js",
  timestamp: new Date().toISOString(),
  apis_demonstrated: [
    "Plasma.glob()",
    "Plasma.exists()",
    "Plasma.hashFile()",
    "Plasma.readFile()",
    "Plasma.exec()",
    "Plasma.writeFile()"
  ]
};
await Plasma.writeFile("output/manifest.json", JSON.stringify(manifest, null, 2));
console.log("[plasma] Wrote manifest to output/manifest.json");

console.log("[plasma] API demonstration complete!");
