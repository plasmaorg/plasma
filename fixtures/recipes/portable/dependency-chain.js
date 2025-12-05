// Recipe demonstrating dependency chain concept
// Note: Dependency resolution is not yet implemented in the runtime
// PLASMA output "final/"

console.log("[plasma] Running dependency chain recipe...");

// Parent directories are created automatically by Plasma.writeFile
const content =
  "Dependency chain recipe completed.\n" +
  "This demonstrates the concept of recipes depending on other recipes.\n" +
  "\n" +
  "Timestamp: " + new Date().toISOString();

await Plasma.writeFile("final/chain-result.txt", content);

console.log("[plasma] Dependency chain complete -> final/");
