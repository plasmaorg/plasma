# Portable Recipes

Portable recipes are JavaScript files executed in Plasma's embedded QuickJS runtime. They can be run locally or fetched directly from Git repositories, enabling easy sharing and reuse of build automation across teams and projects.

> [!IMPORTANT]
> **Why JavaScript?** Portable recipes use JavaScript because Plasma embeds the **QuickJS runtime** (with AWS LLRT modules) directly into the binary. This means:
> - **Zero external dependencies** - No need to install Node.js, Deno, or any other runtime
> - **Guaranteed cross-platform** - Works identically on macOS, Linux, and Windows
> - **Fast startup** - QuickJS starts in milliseconds (~1ms vs ~50ms for Node.js)
> - **Small binary size** - Embedded runtime adds minimal overhead to Plasma's binary
>
> Unlike standard recipes (which require bash, node, python, etc. to be installed), portable recipes work out-of-the-box on any system with Plasma installed.

## Overview

Portable recipes can be run locally:

```bash
plasma run build.js
```

Or fetched from Git repositories using the `@` prefix syntax:

```bash
plasma run @org/repo/path/script.js
```

> [!NOTE] Git as Distribution Mechanism
> Git repositories are used purely as a distribution mechanism for sharing recipes. Recipes are self-contained JavaScript files that cannot depend on other recipes or import external modules. Each recipe runs independently with access to Plasma's built-in APIs only.

Plasma automatically:
- Fetches the repository using `git clone --depth 1`
- Caches it locally following XDG conventions
- Executes the recipe using the embedded QuickJS runtime

## Comparison with CI Reusable Steps

Portable recipes share similarities with CI reusable steps (like GitHub Actions, GitLab CI Components, and Forgejo Actions) but are designed for a different purpose:

**CI Reusable Steps** (GitHub Actions, GitLab CI Components, Forgejo Actions) are CI/CD workflows that:
- Run in cloud infrastructure (provider-specific runners)
- Tightly coupled to specific CI/CD platforms
- Require platform-specific YAML configuration
- Execute in response to repository events (push, pull request, etc.)
- Ideal for automated testing, deployment, and release workflows

**Portable Recipes** are portable automation scripts that:
- Run locally on developer machines or in any CI environment
- Not coupled to any specific CI provider
- Use simple JavaScript with Plasma's embedded runtime
- Execute on-demand via `plasma run` command
- Ideal for cached build steps, code generation, and reproducible automation

Think of portable recipes as **lightweight, portable actions** that work anywhere Plasma is installed, with the added benefit of content-addressed caching for fast, incremental builds.

## Why Portable Recipes?

- **Easy Sharing** - Share recipes across teams by publishing them in Git repositories. No need to copy files manually.
- **Version Control** - Pin recipes to specific versions using Git tags:
  ```bash
  plasma run @tuist/recipes/build.js@v1.0.0
  ```
- **Always Up-to-Date** - Reference `@main` or `@latest` to always use the newest version.
- **Cross-Platform** - Remote recipes run on any platform with the embedded QuickJS runtime - no external dependencies needed.

## Quick Example

```bash
# Run a remote recipe from GitHub (default host)
plasma run @tuist/recipes/typescript-build.js

# With a specific version
plasma run @tuist/recipes/typescript-build.js@v1.0.0

# From GitLab
plasma run @gitlab.com/myorg/recipes/deploy.js

# Verbose mode to see what's happening
plasma run --verbose @tuist/recipes/build.js
```

## How It Works

1. **Parse** - Plasma parses the `@org/repo/path/script.js[@ref]` syntax
2. **Fetch** - Clones the repository to `~/.cache/plasma/recipes/{host}/{org}/{repo}/{ref}/`
3. **Cache** - Subsequent runs reuse the cached version (no re-fetch)
4. **Execute** - Runs the recipe with QuickJS runtime and Plasma APIs

## Supported Git Hosts

- **GitHub** (default) - `@org/repo/script.js`
- **GitLab** - `@gitlab.com/org/repo/script.js`
- **Self-hosted** - `@git.company.com/team/project/script.js`

## Next Steps

- [JavaScript API Reference](/cache/recipes/api-reference) - Complete API documentation for recipe development
- [Syntax Reference](/cache/recipes/portable/syntax) - Learn the full `@` prefix syntax
- [Examples](/cache/recipes/portable/examples) - See real-world portable recipe examples
