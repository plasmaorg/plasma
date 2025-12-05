# Portable Recipe Syntax Reference

Portable recipes are JavaScript files executed in Plasma's embedded QuickJS runtime. They can be run locally or fetched from Git repositories.

## Local Recipes

Run any `.js` file directly with Plasma's embedded QuickJS runtime:

```bash
plasma run <path/to/recipe.js>
```

### Examples

```bash
# Run a recipe in the current directory
plasma run build.js

# Run a recipe in a subdirectory
plasma run scripts/deploy.js

# Run with verbose output
plasma run --verbose build.js

# Absolute path
plasma run /path/to/my/recipe.js
```

### How It Works

When you run a `.js` file with `plasma run`:

1. Plasma detects the `.js` extension
2. Loads the file into the embedded QuickJS runtime
3. Provides access to Plasma APIs (`plasma:cache`, `plasma:kv`, `plasma:fs`)
4. Provides Node.js-compatible APIs (`fs`, `child_process`, `path`)
5. Executes the recipe

### Example Recipe

```javascript
// build.js
import { spawn } from 'child_process';
import { glob } from 'plasma:fs';

console.log("Building project...");

const files = await glob("src/**/*.ts");
console.log(`Found ${files.length} TypeScript files`);

const result = await spawn("npm", ["run", "build"]);
if (result.exitCode !== 0) {
    throw new Error("Build failed!");
}

console.log("Build complete!");
```

---

## Remote Recipes

Fetch and run recipes directly from Git repositories using the `@` prefix syntax:

```bash
plasma run @[host/]org/repo/path/script.js[@ref]
```

### Components

#### Required Components

**`@` Prefix**
All remote recipes must start with `@` to differentiate them from local file paths.

**Organization/User** (`org`)
The GitHub/GitLab organization or username.

**Repository** (`repo`)
The repository name.

**Path** (`path/script.js`)
The path to the recipe file within the repository. Can include subdirectories.

#### Optional Components

**Host** (`host`)
The Git server hostname. Defaults to `github.com` if not specified.

Examples:
- `github.com` (default)
- `gitlab.com`
- `git.company.com` (self-hosted)

**Git Reference** (`@ref`)
A Git branch, tag, or commit SHA. Defaults to `main` if not specified.

Examples:
- `@main` (branch)
- `@v1.0.0` (tag)
- `@abc123def` (commit SHA)

### Remote Syntax Examples

#### GitHub (Default Host)

```bash
# Simple (uses main branch)
plasma run @tuist/recipes/build.js

# With version tag
plasma run @tuist/recipes/build.js@v1.0.0

# Nested path
plasma run @tuist/recipes/scripts/deploy/production.js

# With branch
plasma run @tuist/recipes/build.js@develop
```

#### GitLab

```bash
# Simple
plasma run @gitlab.com/myorg/myrepo/build.js

# With version
plasma run @gitlab.com/myorg/myrepo/build.js@v2.0.0

# Nested path
plasma run @gitlab.com/myorg/myrepo/ci/deploy.js@release
```

#### Self-Hosted Git

```bash
# Company Git server
plasma run @git.company.com/team/project/build.js

# With specific commit
plasma run @git.company.com/team/project/build.js@abc123def

# Nested path
plasma run @git.company.com/team/project/scripts/test.js@main
```

### Cache Directory Structure

Remote recipes are cached following XDG Base Directory conventions:

```
~/.cache/plasma/recipes/
├── github.com/
│   └── tuist/
│       └── recipes/
│           ├── main/              # Default branch
│           │   ├── build.js
│           │   └── deploy.js
│           ├── v1.0.0/            # Tagged version
│           │   └── build.js
│           └── develop/           # Branch
│               └── experimental.js
├── gitlab.com/
│   └── myorg/
│       └── myrepo/
│           └── v2.0.0/
│               └── build.js
└── git.company.com/
    └── team/
        └── project/
            └── main/
                └── build.js
```

### Git URL Generation

Plasma converts the remote recipe syntax to HTTPS Git URLs:

| Syntax | Git URL |
|--------|---------|
| `@tuist/recipes/build.js` | `https://github.com/tuist/recipes.git` |
| `@gitlab.com/org/repo/script.js` | `https://gitlab.com/org/repo.git` |
| `@git.company.com/team/project/build.js` | `https://git.company.com/team/project.git` |

### Cloning Behavior

- **Shallow Clone** - Plasma uses `git clone --depth 1` for efficient fetching - only the latest commit is downloaded.
- **Cached After First Fetch** - Once fetched, the recipe is cached locally. Subsequent runs reuse the cache without re-fetching.
- **Branch Tracking** - When using a branch reference (e.g., `@main`), the cache is specific to that branch. Switching branches fetches a new copy.

---

## Common Patterns

### Local Development, Remote Production

```bash
# During development - iterate on local recipe
plasma run build.js

# In CI/production - use versioned remote recipe
plasma run @myorg/recipes/build.js@v1.0.0
```

### Versioned Releases

```bash
# Production - use stable release
plasma run @tuist/recipes/build.js@v1.0.0

# Development - use latest from main
plasma run @tuist/recipes/build.js@main
```

### Monorepo with Multiple Recipes

```bash
# Different recipes in same repo
plasma run @org/recipes/ci/build.js
plasma run @org/recipes/ci/test.js
plasma run @org/recipes/deploy/staging.js
plasma run @org/recipes/deploy/production.js
```

### Multi-Environment Recipes

```bash
# Select environment via recipe path
plasma run @company/infra/deploy/dev.js
plasma run @company/infra/deploy/staging.js
plasma run @company/infra/deploy/prod.js
```

---

## Quick Reference

| Type | Syntax | Example |
|------|--------|---------|
| Local recipe | `plasma run <file.js>` | `plasma run build.js` |
| Remote (GitHub) | `plasma run @org/repo/file.js` | `plasma run @tuist/recipes/build.js` |
| Remote with version | `plasma run @org/repo/file.js@tag` | `plasma run @tuist/recipes/build.js@v1.0.0` |
| Remote (GitLab) | `plasma run @gitlab.com/org/repo/file.js` | `plasma run @gitlab.com/myorg/recipes/build.js` |
| Remote (Self-hosted) | `plasma run @host/org/repo/file.js` | `plasma run @git.company.com/team/project/build.js` |

## Next Steps

- [Examples](/cache/recipes/portable/examples) - See real-world usage
- [JavaScript API Reference](/cache/recipes/api-reference) - Complete API documentation
- [Standard Recipes](/cache/recipes/standard/) - Learn about standard script recipes (bash, python, etc.)
