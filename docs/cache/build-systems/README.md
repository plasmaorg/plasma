# Build System Integration

Build system-specific integration guides for Plasma.

> **Note:** This section assumes you've already completed the [Getting Started Guide](/getting-started) which covers:
> - Installing Plasma
> - Setting up shell integration
> - Running `plasma init`
> - Verifying with `plasma doctor`

The guides below focus on **build system-specific** configuration and tips.

## Supported Build Systems

### Build Systems

- **[Gradle](./gradle)** - Java, Kotlin, Android projects
- **[Bazel](./bazel)** - Multi-language monorepos *(coming soon)*
- **[Nx](./nx)** - JavaScript/TypeScript monorepos *(coming soon)*
- **[TurboRepo](./turborepo)** - JavaScript/TypeScript monorepos

### Platform-Specific

- **[Xcode](./xcode)** - iOS, macOS, watchOS, tvOS apps *(coming soon)*

### Compiler Caches

- **sccache** - Rust, C, C++ compiler cache *(coming soon)*

## What These Guides Cover

Each build system guide includes:

- **How it works** - How Plasma integrates with the build system
- **Quick start** - Minimal example to get started
- **Configuration examples** - Build system-specific configurations
- **Advanced tips** - Performance optimization and best practices
- **Troubleshooting** - Common issues and solutions
- **CI/CD integration** - Examples for popular CI platforms

## What These Guides DON'T Cover

Setup instructions that are **common to all build systems**:

- ❌ Installing Plasma (see [Getting Started](/getting-started))
- ❌ Shell integration setup (see [Getting Started](/getting-started))
- ❌ Running `plasma init` (see [Getting Started](/getting-started))
- ❌ Basic troubleshooting with `plasma doctor` (see [Getting Started](/getting-started))

## General Pattern

All build systems follow the same pattern:

1. **Navigate to project**: `cd ~/myproject`
2. **Daemon starts automatically**: Plasma detects `plasma.toml`
3. **Environment variables exported**: Build tool-specific URLs
4. **Build tool reads env var**: Connects to daemon automatically
5. **Cache magic happens**: Builds are faster! 🚀

## Environment Variables

Plasma exports these for different build tools:

| Variable | Build Systems | Purpose |
|----------|--------------|---------|
| `PLASMA_HTTP_URL` | All HTTP-based | Generic HTTP cache URL |
| `PLASMA_GRPC_URL` | Bazel, Buck2 | gRPC cache URL |
| `GRADLE_BUILD_CACHE_URL` | Gradle | Gradle-specific cache URL |
| `NX_SELF_HOSTED_REMOTE_CACHE_SERVER` | Nx | Nx-specific cache URL |
| `TURBO_API` | TurboRepo | TurboRepo cache server URL |
| `TURBO_TOKEN` | TurboRepo | Auth token (auto-generated) |
| `TURBO_TEAM` | TurboRepo | Team slug (auto-generated) |
| `XCODE_CACHE_SERVER` | Xcode | Xcode cache URL |

## Example Configuration

A typical `plasma.toml` works for all build systems:

```toml
# Local cache only
[cache]
dir = ".plasma/cache"
max_size = "5GB"
```

Or with remote cache:

```toml
# With remote cache
[cache]
dir = ".plasma/cache"
max_size = "5GB"

[[upstream]]
url = "grpc://cache.example.com:443"
timeout = "30s"

[auth]
token_file = ".plasma.token"
```

## See Also

- [Getting Started Guide](/getting-started) - Setup instructions
- [CLI Reference](/reference/cli) - Command documentation
- [Architecture](/guide/architecture) - Deep dive into Plasma internals
