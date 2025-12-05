# Annotations Reference

Complete reference for all PLASMA annotations used in standard recipes.

## Overview

Annotations are declared as comments in your script using the `PLASMA` keyword, prefixed with your language's comment symbol. The syntax is based on [KDL (KDL Document Language)](https://kdl.dev/), but you don't need to understand KDL to use them - just follow the examples below.

**Comment syntax varies by language:**

```bash
# Bash, Python, Ruby
#PLASMA input "src/**/*.py"
```

```javascript
// Node.js, JavaScript
//PLASMA input "src/**/*.js"
```

```
; Lisp, Clojure
;PLASMA input "src/**/*.clj"
```

The pattern is always: `<comment-symbol>PLASMA <directive> <arguments>`

## Input Tracking

### `#PLASMA input`

Track input files that affect the cache key. When any tracked file changes, the cache invalidates.

**Basic syntax:**
```bash
#PLASMA input "path/to/file"
```

**With globs:**
```bash
#PLASMA input "src/**/*.ts"
#PLASMA input "*.json"
#PLASMA input "config/*.yml"
```

**With hash method:**
```bash
#PLASMA input "large-binary.dat" hash=size      # Only track file size
#PLASMA input "video.mp4" hash=mtime            # Only track modification time
#PLASMA input "source.ts" hash=content          # Full content hash (default)
```

**Hash methods:**
- `content` - Hash entire file contents (default, most reliable)
- `mtime` - Hash modification time only (faster, less reliable)
- `size` - Hash file size only (fastest, least reliable)

**Examples:**
```bash
# Track TypeScript source files
#PLASMA input "src/**/*.ts"

# Track package manifest
#PLASMA input "package.json"

# Track multiple config files
#PLASMA input "tsconfig.json"
#PLASMA input ".eslintrc.js"

# Track large binary with size-based hashing
#PLASMA input "assets/video.mp4" hash=size
```

## Output Declaration

### `#PLASMA output`

Declare output paths (files or directories) that should be cached and restored.

**Syntax:**
```bash
#PLASMA output "path/to/output"
```

**Examples:**
```bash
# Cache build directory
#PLASMA output "dist/"

# Cache specific file
#PLASMA output "bundle.js"

# Cache multiple outputs
#PLASMA output "dist/"
#PLASMA output "build/"
#PLASMA output "coverage/"
```

**Notes:**
- Outputs are archived and compressed (tar + zstd)
- On cache hit, outputs are extracted before the script "executes"
- Only cached if script exits with code 0 (success)

## Environment Variables

### `#PLASMA env`

Track environment variable values in the cache key. When the variable changes, cache invalidates.

**Syntax:**
```bash
#PLASMA env "VARIABLE_NAME"
```

**Examples:**
```bash
# Track NODE_ENV
#PLASMA env "NODE_ENV"

# Track multiple variables
#PLASMA env "NODE_ENV"
#PLASMA env "API_KEY"
#PLASMA env "BUILD_TARGET"
```

**Use cases:**
- Configuration that affects output (`NODE_ENV`, `BUILD_MODE`)
- API keys or tokens that affect behavior
- Target platforms or architectures

**Notes:**
- Only the variable **value** is tracked, not its presence/absence
- If variable is unset, it's treated as empty string

## Script Dependencies

### `#PLASMA depends`

Declare dependencies on other scripts. Plasma will execute dependencies before the current script.

**Basic syntax:**
```bash
#PLASMA depends "path/to/script.sh"
```

**With output reuse:**
```bash
#PLASMA depends "build-deps.sh" use-outputs=#true
```

**Examples:**
```bash
# Simple dependency
#PLASMA depends "./prepare.sh"

# Dependency with output reuse (adds dependency outputs as inputs)
#PLASMA depends "./build-libs.sh" use-outputs=#true

# Multiple dependencies
#PLASMA depends "./step1.sh"
#PLASMA depends "./step2.sh"
```

**With `use-outputs=#true`:**
- Dependency's outputs are automatically added as inputs to current script
- Ensures cache invalidation when dependency outputs change
- Useful for build pipelines (build → test → deploy)

**Notes:**
- Dependencies are resolved recursively
- Cyclic dependencies are detected and rejected
- Dependencies execute in order

## Cache Control

### `#PLASMA cache disable`

Disable caching for this script.

**Syntax:**
```bash
#PLASMA cache disable
```

**Example:**
```bash
#!/usr/bin/env bash
#PLASMA cache disable

# This script will always execute, never cache
echo "Current time: $(date)"
```

**Use cases:**
- Scripts with side effects (API calls, database updates)
- Scripts that depend on current time
- Scripts with non-deterministic outputs

### `#PLASMA cache ttl`

Set cache expiration time. Cached results older than TTL are invalidated.

**Syntax:**
```bash
#PLASMA cache ttl="duration"
```

**Duration format:**
- `h` - hours (e.g., `2h`)
- `d` - days (e.g., `7d`)
- `m` - minutes (e.g., `30m`)

**Examples:**
```bash
# Expire after 2 hours
#PLASMA cache ttl="2h"

# Expire after 7 days
#PLASMA cache ttl="7d"

# Expire after 30 minutes
#PLASMA cache ttl="30m"
```

**Use cases:**
- Time-sensitive scripts (nightly builds, reports)
- Scripts that fetch external data
- Scripts with large outputs (expire old builds)

### `#PLASMA cache key`

Override cache key with a custom value.

**Syntax:**
```bash
#PLASMA cache key="custom-key"
```

**Example:**
```bash
#PLASMA cache key="v2-build-prod"
```

**Use cases:**
- Manual cache invalidation (change key to bust cache)
- Versioned caching
- Environment-specific keys

**Notes:**
- Custom key is **appended** to computed hash (doesn't replace it)
- Useful for forcing cache invalidation without changing script

## Runtime Configuration

### `#PLASMA runtime`

Override the runtime used to execute the script (defaults to shebang).

**Syntax:**
```bash
#PLASMA runtime command
```

**Examples:**
```bash
# Use specific Node version
#PLASMA runtime node

# Use specific Python interpreter
#PLASMA runtime python3.11

# Use bash
#PLASMA runtime bash
```

**Notes:**
- Runtime is resolved from PATH
- Overrides the shebang line
- Useful when shebang isn't flexible enough

### `#PLASMA runtime-arg`

Pass arguments to the runtime.

**Syntax:**
```bash
#PLASMA runtime-arg "argument"
```

**Examples:**
```bash
# Increase Node.js memory
#PLASMA runtime-arg "--max-old-space-size=4096"

# Python unbuffered output
#PLASMA runtime-arg "-u"

# Multiple arguments
#PLASMA runtime-arg "--max-old-space-size=4096"
#PLASMA runtime-arg "--expose-gc"
```

### `#PLASMA runtime-version`

Include runtime version in the cache key.

**Syntax:**
```bash
#PLASMA runtime-version
```

**Example:**
```bash
#!/usr/bin/env node
#PLASMA runtime-version
#PLASMA output "dist/"

// Build with specific Node version
// Cache invalidates when Node version changes
```

**Use cases:**
- Scripts whose output depends on runtime version
- Prevent cache hits across different runtime versions

**Notes:**
- Runs `runtime --version` and includes output in cache key
- Adds slight overhead on first run

## Execution Control

### `#PLASMA exec cwd`

Set working directory for script execution.

**Syntax:**
```bash
#PLASMA exec cwd="path/to/directory"
```

**Examples:**
```bash
# Run in subdirectory
#PLASMA exec cwd="frontend"

# Run in parent directory
#PLASMA exec cwd=".."
```

**Notes:**
- Path is relative to script location
- Defaults to script's directory if not specified

### `#PLASMA exec timeout`

Set maximum execution time. Script is killed if it exceeds timeout.

**Syntax:**
```bash
#PLASMA exec timeout="duration"
```

**Duration format:**
- `s` - seconds (e.g., `30s`)
- `m` - minutes (e.g., `5m`)
- `h` - hours (e.g., `2h`)

**Examples:**
```bash
# Timeout after 5 minutes
#PLASMA exec timeout="5m"

# Timeout after 30 seconds
#PLASMA exec timeout="30s"
```

**Use cases:**
- Prevent hanging scripts
- CI/CD time limits
- Enforce performance requirements

### `#PLASMA exec shell`

Execute via shell (enables shell features like pipes, redirects, etc.).

**Syntax:**
```bash
#PLASMA exec shell
```

**Example:**
```bash
#PLASMA exec shell

# Now you can use shell features
echo "Building..." | tee build.log
```

**Notes:**
- Without this, script is executed directly by the runtime
- With this, script is executed via shell wrapper
- Slightly slower, but enables shell features

## Complete Example

Here's a comprehensive example using multiple directives:

```bash
#!/usr/bin/env bash
#PLASMA input "src/**/*.ts"
#PLASMA input "package.json"
#PLASMA input "tsconfig.json"
#PLASMA output "dist/"
#PLASMA output "build-stats.json"
#PLASMA env "NODE_ENV"
#PLASMA env "BUILD_TARGET"
#PLASMA depends "./install-deps.sh" use-outputs=#true
#PLASMA cache ttl="24h"
#PLASMA runtime-version
#PLASMA exec timeout="10m"

# Build TypeScript project
echo "Building for $BUILD_TARGET..."
npm run build

# Generate stats
npm run stats > build-stats.json
```

## See Also

- [Introduction](/cache/recipes/standard/) - Overview and quick start
- [Examples](/cache/recipes/standard/examples) - Real-world examples
