# Examples

Real-world examples of script caching for common build workflows.

## TypeScript Compilation

Cache TypeScript compilation with automatic invalidation when source files change.

```bash
#!/usr/bin/env bash
#PLASMA input "src/**/*.ts"
#PLASMA input "src/**/*.tsx"
#PLASMA input "tsconfig.json"
#PLASMA input "package.json"
#PLASMA output "dist/"
#PLASMA env "NODE_ENV"

echo "Compiling TypeScript..."
npx tsc
```

**Run:**
```bash
plasma run compile-ts.sh
```

## Code Generation

Cache code generation that depends on schema files.

```bash
#!/usr/bin/env node
#PLASMA input "schema/**/*.graphql"
#PLASMA input "codegen.yml"
#PLASMA output "src/generated/"

// Generate TypeScript types from GraphQL schema
const { exec } = require('child_process');
exec('graphql-codegen --config codegen.yml', (error, stdout, stderr) => {
  if (error) {
    console.error(stderr);
    process.exit(1);
  }
  console.log(stdout);
});
```

**Run:**
```bash
plasma run codegen.js
```

## Image Optimization

Cache image processing with size-based input tracking (faster for large files).

```bash
#!/usr/bin/env bash
#PLASMA input "assets/images/**/*.png" hash=size
#PLASMA input "assets/images/**/*.jpg" hash=size
#PLASMA output "dist/images/"

echo "Optimizing images..."
mkdir -p dist/images

# Optimize all images
for img in assets/images/*; do
  filename=$(basename "$img")
  magick "$img" -quality 85 -strip "dist/images/$filename"
done
```

**Run:**
```bash
plasma run optimize-images.sh
```

## Test Runner

Cache test execution when source files haven't changed.

```bash
#!/usr/bin/env bash
#PLASMA input "src/**/*.ts"
#PLASMA input "tests/**/*.test.ts"
#PLASMA input "jest.config.js"
#PLASMA output "coverage/"
#PLASMA env "CI"

echo "Running tests..."
npm test -- --coverage
```

**Run:**
```bash
plasma run test.sh
```

**Note:** Only cache if tests are deterministic. Skip flaky tests or tests with external dependencies.

## Dependency Installation

Cache `node_modules` when `package.json` or lock file hasn't changed.

```bash
#!/usr/bin/env bash
#PLASMA input "package.json"
#PLASMA input "package-lock.json"
#PLASMA output "node_modules/"

echo "Installing dependencies..."
npm ci
```

**Run:**
```bash
plasma run install-deps.sh
```

## Docker Image Build

Cache Docker image builds with multi-stage caching.

```bash
#!/usr/bin/env bash
#PLASMA input "Dockerfile"
#PLASMA input "src/**/*.go"
#PLASMA input "go.mod"
#PLASMA input "go.sum"
#PLASMA output "image-digest.txt"
#PLASMA env "DOCKER_BUILDKIT"

echo "Building Docker image..."
docker build -t myapp:latest . --iidfile=image-digest.txt
```

**Run:**
```bash
plasma run docker-build.sh
```

## Multi-Step Pipeline

Chain multiple scripts with dependencies.

**Step 1: Install dependencies**
```bash
#!/usr/bin/env bash
# install.sh
#PLASMA input "package.json"
#PLASMA input "package-lock.json"
#PLASMA output "node_modules/"

npm ci
```

**Step 2: Build application**
```bash
#!/usr/bin/env bash
# build.sh
#PLASMA input "src/**/*.ts"
#PLASMA input "tsconfig.json"
#PLASMA depends "./install.sh" use-outputs=#true
#PLASMA output "dist/"

npm run build
```

**Step 3: Run tests**
```bash
#!/usr/bin/env bash
# test.sh
#PLASMA input "tests/**/*.test.ts"
#PLASMA depends "./build.sh" use-outputs=#true
#PLASMA output "coverage/"

npm test -- --coverage
```

**Run:**
```bash
# Run the entire pipeline (automatically resolves dependencies)
plasma run test.sh
```

## Linting & Formatting

Cache linting results when source files haven't changed.

```bash
#!/usr/bin/env bash
#PLASMA input "src/**/*.ts"
#PLASMA input ".eslintrc.js"
#PLASMA input ".prettierrc"
#PLASMA output ".lint-cache/"

echo "Running ESLint..."
npx eslint src/ --cache --cache-location .lint-cache/

echo "Running Prettier..."
npx prettier src/ --check
```

**Run:**
```bash
plasma run lint.sh
```

## Asset Bundling

Cache webpack/rollup builds with proper input tracking.

```bash
#!/usr/bin/env bash
#PLASMA input "src/**/*.js"
#PLASMA input "src/**/*.css"
#PLASMA input "webpack.config.js"
#PLASMA input "package.json"
#PLASMA output "dist/"
#PLASMA env "NODE_ENV"
#PLASMA exec timeout="10m"

echo "Bundling assets..."
npx webpack --mode production
```

**Run:**
```bash
NODE_ENV=production plasma run bundle.sh
```

## Python Data Processing

Cache Python script execution with virtual environment.

```python
#!/usr/bin/env python3
#PLASMA input "data/*.csv"
#PLASMA input "requirements.txt"
#PLASMA output "processed/"
#PLASMA env "PROCESSING_MODE"

import pandas as pd
import os

mode = os.getenv('PROCESSING_MODE', 'standard')
print(f"Processing data in {mode} mode...")

# Read all CSV files
for file in os.listdir('data'):
    if file.endswith('.csv'):
        df = pd.read_csv(f'data/{file}')
        # Process data...
        df.to_csv(f'processed/{file}', index=False)
```

**Run:**
```bash
plasma run process-data.py
```

## Protobuf Generation

Cache protobuf compilation.

```bash
#!/usr/bin/env bash
#PLASMA input "proto/**/*.proto"
#PLASMA output "src/generated/"

echo "Generating protobuf code..."
mkdir -p src/generated

protoc \
  --proto_path=proto \
  --go_out=src/generated \
  --go_opt=paths=source_relative \
  proto/**/*.proto
```

**Run:**
```bash
plasma run gen-proto.sh
```

## Environment-Specific Builds

Use environment variables to create separate caches per environment.

```bash
#!/usr/bin/env bash
#PLASMA input "src/**/*.ts"
#PLASMA input "config/${BUILD_ENV}.json"
#PLASMA output "dist/"
#PLASMA env "BUILD_ENV"
#PLASMA env "API_URL"

echo "Building for $BUILD_ENV environment..."
npm run build -- --env=$BUILD_ENV
```

**Run:**
```bash
# Each environment gets its own cache
BUILD_ENV=development plasma run build.sh
BUILD_ENV=staging plasma run build.sh
BUILD_ENV=production plasma run build.sh
```

## Custom Cache Keys

Manually version your cache for breaking changes.

```bash
#!/usr/bin/env bash
#PLASMA input "src/**/*.ts"
#PLASMA output "dist/"
#PLASMA cache key="v2"  # Bump this to invalidate all caches

# After major refactoring, bump key to v3 to bust all caches
npm run build
```

**Run:**
```bash
plasma run build.sh
```

## Time-Based Cache Expiration

Cache nightly reports with 24-hour expiration.

```bash
#!/usr/bin/env bash
#PLASMA input "data/*.log"
#PLASMA output "reports/"
#PLASMA cache ttl="24h"

echo "Generating daily report..."
./generate-report.sh > reports/$(date +%Y-%m-%d).html
```

**Run:**
```bash
plasma run daily-report.sh
```

## CI/CD Integration

Example GitHub Actions workflow using script caching.

```yaml
name: Build

on: [push]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      # Install Plasma
      - run: curl -fsSL https://plasma.sh/install | bash

      # All scripts use caching automatically
      - run: plasma run install-deps.sh
      - run: plasma run build.sh
      - run: plasma run test.sh

      # Check cache statistics
      - run: plasma cache stats
```

## Advanced: Conditional Caching

Disable caching for release builds.

```bash
#!/usr/bin/env bash
if [ "$RELEASE_BUILD" = "true" ]; then
  #PLASMA cache disable
fi

#PLASMA input "src/**/*.ts"
#PLASMA output "dist/"

npm run build
```

**Run:**
```bash
# Development build (cached)
plasma run build.sh

# Release build (not cached)
RELEASE_BUILD=true plasma run build.sh
```

## Debugging

Use verbose mode to see exactly what's happening.

```bash
# See input/output tracking and cache operations
plasma run --verbose build.sh

# Dry run to see cache key without executing
plasma run --dry-run build.sh

# Force re-execution
plasma run --no-cache build.sh

# Clean cache and re-execute
plasma run --clean build.sh
```

## See Also

- [Introduction](/cache/recipes/standard/) - Overview and quick start
- [Annotations Reference](/cache/recipes/standard/annotations) - Complete directive reference
