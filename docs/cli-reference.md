# CLI Reference

Complete reference for Plasma's command-line interface.

## Global Options

```
plasma [OPTIONS] <COMMAND>

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Commands

### `plasma activate`

Generate shell integration hook or check daemon status.

```bash
plasma activate <SHELL>     # Generate shell hook
plasma activate --status    # Check/start daemon and export env vars
```

**Arguments:**
- `<SHELL>` - Shell type: `bash`, `zsh`, or `fish`

**Flags:**
- `--status` - Check daemon status and start if needed

**Examples:**

```bash
# Generate shell integration for bash
eval "$(plasma activate bash)"

# Check daemon status (used by shell hook)
plasma activate --status
```

**What it does:**
- **Without flags**: Outputs shell integration code to stdout
- **With `--status`**: 
  1. Detects `plasma.toml` in current directory (walks up tree)
  2. Computes config hash
  3. Checks if daemon is running for this config
  4. If not running: spawns daemon in background
  5. Exports environment variables with daemon ports

**Environment variables exported:**
- `PLASMA_HTTP_URL` - HTTP server URL (e.g., `http://127.0.0.1:54321`)
- `PLASMA_GRPC_URL` - gRPC server URL (e.g., `grpc://127.0.0.1:54322`)
- `PLASMA_CONFIG_HASH` - Config file hash
- `PLASMA_DAEMON_PID` - Daemon process ID
- `GRADLE_BUILD_CACHE_URL` - For Gradle
- `NX_SELF_HOSTED_REMOTE_CACHE_SERVER` - For Nx
- `XCODE_CACHE_SERVER` - For Xcode

---

### `plasma init`

Initialize Plasma configuration for a project.

```bash
plasma init [OPTIONS]
```

**Options:**
- `--non-interactive` - Skip interactive prompts and use defaults
- `--cache-dir <DIR>` - Cache directory (default: `.plasma/cache`)
- `--max-cache-size <SIZE>` - Max cache size (default: `5GB`)
- `--upstream-url <URL>` - Upstream cache URL (optional)

**Examples:**

```bash
# Interactive initialization (recommended)
plasma init

# Non-interactive with defaults
plasma init --non-interactive

# Non-interactive with custom values
plasma init --non-interactive \
  --cache-dir /tmp/cache \
  --max-cache-size 10GB \
  --upstream-url grpc://cache.example.com:443
```

**What it does:**
1. Checks if `plasma.toml` already exists (prompts to overwrite)
2. Asks for configuration values interactively:
   - Cache directory location
   - Maximum cache size
   - Whether you have a remote cache server
   - Remote cache URL (if applicable)
3. Generates `plasma.toml` in current directory
4. Shows configuration summary
5. Displays next steps

**Interactive prompts:**
```
Cache directory [.plasma/cache]:
Max cache size [5GB]:
Do you have a remote cache server? [y/N]
Remote cache URL (e.g., grpc://cache.example.com:443):
```

---

### `plasma daemon`

Manually start a cache daemon.

```bash
plasma daemon [OPTIONS]
```

**Options:**
- `-c, --config <PATH>` - Path to config file
- `--config-cache-dir <DIR>` - Cache directory
- `--config-max-cache-size <SIZE>` - Max cache size (e.g., "5GB")
- `--config-upstream <URL>` - Upstream cache URLs (comma-separated)
- `--config-http-port <PORT>` - HTTP server port (0 = random)
- `--config-grpc-port <PORT>` - gRPC server port (0 = random)
- `--config-log-level <LEVEL>` - Log level (trace|debug|info|warn|error)

**Examples:**

```bash
# Start daemon with config file
plasma daemon --config plasma.toml

# Start daemon with CLI options
plasma daemon --config-cache-dir /tmp/cache --config-http-port 0
```

**What it does:**
1. Loads configuration from file and/or CLI options
2. Binds HTTP server to port 0 (or specified port)
3. Binds gRPC server to port 0 (or specified port)
4. Writes state to `~/.plasma/daemons/{config_hash}/`
5. Starts servers and waits for shutdown signal (Ctrl+C or SIGTERM)
6. On shutdown: waits for in-flight requests, then cleans up state

---

### `plasma deactivate`

Deactivate Plasma and optionally stop the daemon.

```bash
plasma deactivate [OPTIONS]
```

**Options:**
- `--stop-daemon` - Also stop the running daemon

**Examples:**

```bash
# Unset environment variables
plasma deactivate

# Unset env vars and stop daemon
plasma deactivate --stop-daemon
```

---

### `plasma doctor`

Check system configuration and shell integration.

```bash
plasma doctor [OPTIONS]
```

**Options:**
- `-v, --verbose` - Show verbose output

**Examples:**

```bash
# Quick check
plasma doctor

# Detailed check
plasma doctor --verbose
```

**What it checks:**
- ✅ Plasma binary exists and is accessible
- ✅ Shell detected (bash, zsh, fish)
- ✅ Shell integration configured (checks rc file)
- ✅ State directory exists
- ✅ `plasma.toml` in current directory
- ✅ Daemon running for current config
- ✅ Environment variables set (verbose mode)

**Exit codes:**
- `0` - All checks passed
- `1` - Some checks failed

---

### `plasma server`

Run a remote cache server (Layer 2).

```bash
plasma server [OPTIONS]
```

**Options:**
- `-c, --config <PATH>` - Path to config file
- `--config-cache-dir <DIR>` - Cache directory
- `--config-max-cache-size <SIZE>` - Max cache size
- `--config-upstream <URL>` - Upstream cache URLs
- `--config-http-port <PORT>` - HTTP server port
- `--config-grpc-port <PORT>` - gRPC server port
- `--config-metrics-port <PORT>` - Metrics server port

**Examples:**

```bash
# Run server with config file
plasma server --config /etc/plasma/config.toml

# Run server on specific ports
plasma server --config-http-port 8080 --config-grpc-port 9090
```

**Use case:**
- Deploy as a long-running service
- Shared cache for team members
- Regional cache instances

---

### `plasma config`

Configuration management utilities.

```bash
plasma config <SUBCOMMAND>
```

**Subcommands:**
- `validate` - Validate configuration file
- `generate` - Generate example configuration
- `show` - Show effective configuration

**Examples:**

```bash
# Validate config file
plasma config validate plasma.toml

# Generate example config
plasma config generate --template=server > config.toml

# Show effective configuration
plasma config show --config plasma.toml
```

---

### `plasma health`

Health check and diagnostics.

```bash
plasma health [OPTIONS]
```

**Options:**
- `--url <URL>` - URL of Plasma instance to check
- `--timeout <DURATION>` - Request timeout (default: 5s)
- `--format <FORMAT>` - Output format: text or json (default: text)

**Examples:**

```bash
# Check local daemon
plasma health --url http://127.0.0.1:54321

# Check remote server
plasma health --url https://cache.example.com --timeout 10s

# JSON output
plasma health --url http://127.0.0.1:54321 --format json
```

---

## Configuration File

Plasma can be configured via a `plasma.toml` file:

```toml
# Cache settings
[cache]
dir = ".plasma/cache"
max_size = "5GB"
eviction_policy = "lfu"  # lru, lfu, or ttl

# Upstream caches (optional)
[[upstream]]
url = "grpc://cache.example.com:443"
timeout = "30s"

# Authentication (optional)
[auth]
token_file = ".plasma.token"  # Path to JWT token file

# Daemon settings (optional)
[daemon]
http_port = 0  # 0 = random port (recommended)
grpc_port = 0
metrics_port = 9091

# Logging (optional)
[observability]
log_level = "info"
log_format = "json"
```

## Environment Variables

All configuration options can be set via environment variables with the `PLASMA_CONFIG_*` prefix:

| Config Option | Environment Variable |
|--------------|---------------------|
| `cache.dir` | `PLASMA_CONFIG_CACHE_DIR` |
| `cache.max_size` | `PLASMA_CONFIG_MAX_CACHE_SIZE` |
| `upstream[0].url` | `PLASMA_CONFIG_UPSTREAM_0_URL` |
| `auth.token` | `PLASMA_CONFIG_AUTH_TOKEN` or `PLASMA_TOKEN` |
| `daemon.http_port` | `PLASMA_CONFIG_HTTP_PORT` |
| `daemon.grpc_port` | `PLASMA_CONFIG_GRPC_PORT` |

## Configuration Precedence

Configuration is loaded in this order (highest to lowest priority):

1. **Command-line arguments** - `--config-*` flags
2. **Environment variables** - `PLASMA_CONFIG_*` variables
3. **Configuration file** - `plasma.toml`

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error |
| `2` | Configuration error |
| `3` | Network error |
| `4` | Storage error |

## See Also

- [Getting Started](/getting-started) - Getting started guide
- [Architecture](/guide/architecture) - Architecture documentation
