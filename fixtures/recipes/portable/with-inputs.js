// Recipe that reads input files and generates output
// PLASMA output "output/"
// PLASMA cache ttl="7d"

console.log("[plasma] Processing input files...");

// Use Plasma.glob to find any txt files (will be empty if none exist)
const inputFiles = await Plasma.glob("*.txt");
console.log("[plasma] Found", inputFiles.length, "input files");

// Process each input file (or simulate if none found)
// Parent directories are created automatically by Plasma.writeFile
if (inputFiles.length === 0) {
  console.log("[plasma] No input files found, generating sample output");
  await Plasma.writeFile("output/sample.txt", "Sample output generated\n");
} else {
  for (const file of inputFiles) {
    const data = await Plasma.readFile(file);
    const lines = data.length; // Count bytes as proxy for content size

    const outputFile = "output/processed-" + file;
    await Plasma.writeFile(outputFile, "Processed: " + file + "\nBytes: " + lines);
  }
}

// Write summary
const summary = {
  filesProcessed: inputFiles.length || 1,
  timestamp: new Date().toISOString()
};
await Plasma.writeFile("output/summary.json", JSON.stringify(summary, null, 2));

console.log("[plasma] Processing complete -> output/");
