// Recipe that uses Plasma.exec to execute commands
// PLASMA cache ttl="1h"

console.log("[plasma] Testing Plasma.exec functionality...");

// Execute a simple echo command using Plasma.exec
console.log("[plasma] Running: echo 'Hello from Plasma.exec'");
const exitCode = await Plasma.exec("echo", ["Hello from Plasma.exec"]);

if (exitCode !== 0) {
  throw new Error(`Command failed with exit code ${exitCode}`);
}

console.log("[plasma] Command executed successfully with exit code:", exitCode);
console.log("[plasma] Exec test complete!");
