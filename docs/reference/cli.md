# CLI Reference

Complete reference for all Plasma CLI commands.

## Overview

Plasma uses an activation-based approach for managing build caches. Instead of wrapping your build commands, Plasma runs as a background daemon that your build tools connect to automatically.

**Two main workflows:**

1. **Shell Integration** (`plasma activate`) - Automatic daemon management for development
2. **Explicit Execution** (`plasma exec`) - Manual daemon management for CI/CD

See the [Getting Started Guide](/getting-started) for detailed usage examples.

## `plasma activate`

Set up shell integration to automatically manage the cache daemon.

### Usage

```bash
# Initial setup - outputs shell hook code
plasma activate <SHELL>

# Check status and start daemon if needed
plasma activate --status
```

### Shells

- `bash` - Bash shell
- `zsh` - Zsh shell  
- `fish` - Fish shell

### Examples

```bash
# One-time shell setup (add to .bashrc or .zshrc)
echo 'eval "$(plasma activate bash)"' >> ~/.bashrc
source ~/.bashrc

# For zsh
echo 'eval "$(plasma activate zsh)"' >> ~/.zshrc

# For fish
echo 'plasma activate fish | source' >> ~/.config/fish/config.fish

# Manual activation (check/start daemon)
plasma activate --status
```

### What It Does

When you `cd` into a directory:

1. **Searches** for `.plasma.toml` up the directory tree
2. **Computes** configuration hash to identify unique daemon
3. **Checks** if daemon with that config is running
4. **Starts** daemon if not running
5. **Exports** environment variables for build tools:
   ```bash
   PLASMA_HTTP_URL=http://127.0.0.1:58234
   PLASMA_GRPC_URL=grpc://127.0.0.1:58235
   GRADLE_BUILD_CACHE_URL=http://127.0.0.1:58234
   NX_SELF_HOSTED_REMOTE_CACHE_SERVER=http://127.0.0.1:58234
   XCODE_CACHE_SERVER=http://127.0.0.1:58234
   ```

### Configuration

Searches for configuration in this order:

1. `$PWD/.plasma.toml`
2. `$PWD/../.plasma.toml` (continues up to root)
3. `~/.config/plasma/config.toml` (global fallback)

Different configurations = different daemon instances.

See the [Getting Started Guide](/getting-started#shell-integration-recommended-for-development) for complete setup.

## `plasma exec`

Execute a command with guaranteed daemon lifecycle.

### Usage

```bash
plasma exec [OPTIONS] <COMMAND> [ARGS...]
```

### Options

| Option | Description |
|--------|-------------|
| `--keep-alive` | Don't stop daemon after command exits (default) |
| `--kill-after` | Stop daemon when command completes |
| `--config <PATH>` | Path to configuration file |

### Examples

```bash
# Basic usage - daemon keeps running after
plasma exec bazel build //...
plasma exec nx build my-app
plasma exec gradle build

# Keep daemon alive for subsequent commands
plasma exec --keep-alive nx build my-app
nx test my-app  # Reuses the same daemon
plasma deactivate  # Clean up when done

# Stop daemon after command
plasma exec --kill-after bazel test //...

# With custom configuration
plasma exec --config .plasma.toml bazel build //...
```

### What It Does

1. **Finds** `.plasma.toml` in current directory tree
2. **Starts** daemon if not running (or reuses existing)
3. **Exports** environment variables (PLASMA_HTTP_URL, etc.)
4. **Executes** your command with those variables set
5. **Optionally** stops daemon after completion (if `--kill-after`)

### When to Use

- **CI/CD pipelines** - Ensures consistent cache behavior
- **One-off builds** - Don't want permanent shell integration
- **Scripts** - Programmatic daemon management

See the [Getting Started Guide](/getting-started#workflow-2-explicit-execution-ci-friendly) for complete examples.

## `plasma daemon`

Manually manage cache daemons.

### Commands

```bash
# Start daemon for current directory's config
plasma daemon start [OPTIONS]

# Stop daemon for current config  
plasma daemon stop

# List all running daemons
plasma daemon list

# Stop all daemons
plasma daemon stop --all

# Clean up orphaned daemons
plasma daemon clean
```

### Options (for `start`)

| Option | Description |
|--------|-------------|
| `--config <PATH>` | Path to configuration file |

### Examples

```bash
# Start daemon explicitly
plasma daemon start

# Start with custom config
plasma daemon start --config .plasma.toml

# Use in multiple terminals
bazel build //...    # Terminal 1
nx build demo        # Terminal 2
gradle build         # Terminal 3

# List running daemons
plasma daemon list

# Stop specific daemon (for current directory)
plasma daemon stop

# Stop all daemons
plasma daemon stop --all

# Clean up orphaned daemons
plasma daemon clean
```

### Daemon State

Daemons are tracked in `~/.plasma/daemons/<config-hash>/`:

```
~/.plasma/daemons/<config-hash>/
├── pid              # Process ID
├── ports.json       # HTTP/gRPC/metrics ports
├── config_path.txt  # Path to config file
```

Each unique configuration gets its own daemon instance.

## `plasma deactivate`

Remove Plasma environment variables and optionally stop daemons.

### Usage

```bash
plasma deactivate [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--stop-daemon` | Also stop the daemon for current directory |

### Examples

```bash
# Unset environment variables only
plasma deactivate

# Also stop the daemon
plasma deactivate --stop-daemon
```

### What It Does

Removes Plasma environment variables from your shell:

- `PLASMA_HTTP_URL`
- `PLASMA_GRPC_URL`
- `PLASMA_CONFIG_HASH`
- `PLASMA_DAEMON_PID`
- `GRADLE_BUILD_CACHE_URL`
- `NX_SELF_HOSTED_REMOTE_CACHE_SERVER`
- `XCODE_CACHE_SERVER`

## `plasma server`

Run a regional/cloud cache server (Layer 2 cache).

**Note:** This is for running Plasma as a remote cache server that multiple developers/CI runners connect to. Most users should use `plasma activate` or `plasma exec` instead.

### Usage

```bash
plasma server --config <CONFIG_FILE>
```

### Options

| Option | Description |
|--------|-------------|
| `--config <PATH>` | Path to server configuration file (required) |

### Examples

```bash
# Start Layer 2 server
plasma server --config /etc/plasma/server.toml

# Generate example server config
plasma config generate --template server > server.toml
plasma server --config server.toml
```

### Server Configuration

Server configuration requires more settings than local daemon:

```toml
[cache]
dir = "/data/plasma/cache"
max_size = "500GB"

[[upstream]]
url = "s3://my-bucket/cache/"
region = "us-east-1"
permanent = true

[auth]
public_key_file = "/etc/plasma/jwt-public-key.pem"

[server]
layer = "regional"
bind = "0.0.0.0:7070"  # gRPC server for Plasma protocol

[observability]
metrics_bind = "0.0.0.0:9091"
health_bind = "0.0.0.0:8888"
```

See [Architecture](/guide/architecture) for complete server setup.

## `plasma config`

Configuration utilities.

### Commands

```bash
# Validate configuration file
plasma config validate <PATH>

# Generate example configuration
plasma config generate --template <TEMPLATE>

# Show effective configuration
plasma config show
```

### Examples

```bash
# Validate project config
plasma config validate .plasma.toml

# Generate project config template
plasma config generate --template project > .plasma.toml

# Generate server config template
plasma config generate --template server > server.toml

# Show current effective configuration
plasma config show
```

### Templates

- `project` - Local daemon configuration (for `.plasma.toml`)
- `server` - Remote server configuration (for Layer 2)

### Project Config Example

```toml
[cache]
dir = ".plasma/cache"
max_size = "10GB"

# Optional: Connect to remote cache
[[upstream]]
url = "grpc://cache.example.com:7070"
timeout = "30s"

# Optional: Authentication
[auth]
token = "${PLASMA_TOKEN}"
```

## `plasma health`

Health check and diagnostics.

### Usage

```bash
plasma health [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--url <URL>` | Health endpoint URL (default: `http://localhost:8888/health`) |
| `--timeout <DURATION>` | Request timeout (default: "5s") |

### Examples

```bash
# Check local daemon
plasma health

# Check remote server
plasma health --url https://cache.example.com:8888/health

# With custom timeout
plasma health --timeout 10s
```

## `plasma run`

Execute scripts with automatic caching based on KDL annotations.

### Usage

```bash
# Execute script with caching
plasma run <SCRIPT> [-- SCRIPT_ARGS...]

# Script management operations
plasma run --status <SCRIPT>    # Check cache status
plasma run --list               # List all cached scripts
plasma run --stats              # Show cache statistics
```

### Options

| Option | Description |
|--------|-------------|
| `--status` | Check cache status for a script |
| `--list` | List all cached scripts |
| `--stats` | Show script cache statistics |
| `--no-cache` | Force execution without checking cache |
| `--clean` | Remove cached outputs before running |
| `--dry-run` | Show what would happen without executing |
| `--cache-only` | Fail if cache miss (for CI validation) |
| `--verbose`, `-v` | Verbose output |

### Examples

```bash
# Execute script with caching
plasma run build.sh

# Check cache status
plasma run --status build.sh

# List all cached scripts
plasma run --list

# Show cache statistics
plasma run --stats

# Force re-execution
plasma run --no-cache build.sh

# Clean cache and re-run
plasma run --clean build.sh
```

See [Standard Recipes Documentation](/cache/recipes/standard/) for details on PLASMA annotations and script caching.

## `plasma cas`

Content-Addressed Storage operations for blob storage.

CAS operations work with content hashes (SHA256) to store and retrieve arbitrary binary data.

### Commands

```bash
# Get a blob by hash
plasma cas get <HASH> [--output <FILE>]

# Store a file (returns hash)
plasma cas put <FILE> [--hash <EXPECTED_HASH>]

# Check if blob exists
plasma cas exists <HASH>

# Delete a blob
plasma cas delete <HASH> [--force]

# Show blob information
plasma cas info <HASH>

# List all blobs
plasma cas list [--verbose]

# Show storage statistics
plasma cas stats
```

### Examples

```bash
# Store a file in CAS
plasma cas put myfile.bin
# Output: abc123def456... (hash)

# Retrieve blob by hash
plasma cas get abc123def456... --output restored.bin

# Check if blob exists
plasma cas exists abc123def456...

# Get blob information
plasma cas info abc123def456...

# List all blobs
plasma cas list --verbose

# Show CAS statistics
plasma cas stats

# Delete a blob
plasma cas delete abc123def456... --force
```

### JSON Output

Most commands support `--json` flag for machine-readable output:

```bash
plasma cas put file.bin --json
# {"hash":"abc123...","size_bytes":1024,"success":true}

plasma cas get abc123... --output file.bin --json
# {"hash":"abc123...","output_path":"file.bin","size_bytes":1024,"success":true}
```

## `plasma kv`

Key-Value storage operations for action cache and metadata.

KV operations use arbitrary string keys (not content hashes) to store and retrieve data.

### Commands

```bash
# Get value by key
plasma kv get <KEY> [--output <FILE>]

# Store key-value pair
plasma kv put <KEY> <VALUE>
plasma kv put <KEY> --file <FILE>

# Check if key exists
plasma kv exists <KEY>

# Delete key-value pair
plasma kv delete <KEY> [--force]

# List all keys
plasma kv list [--prefix <PREFIX>]

# Show storage statistics
plasma kv stats
```

### Examples

```bash
# Store a value
plasma kv put build-result "success"

# Store from file
plasma kv put build-metadata --file metadata.json

# Retrieve value
plasma kv get build-result

# Retrieve to file
plasma kv get build-metadata --output metadata.json

# Check if key exists
plasma kv exists build-result

# List all keys
plasma kv list

# List keys with prefix
plasma kv list --prefix build-

# Show KV statistics
plasma kv stats

# Delete a key
plasma kv delete build-result --force
```

### JSON Output

All commands support `--json` flag:

```bash
plasma kv put mykey "myvalue" --json
# {"key":"mykey","value_bytes":7,"success":true}

plasma kv list --json
# [{"key":"build-result"},{"key":"build-metadata"}]

plasma kv stats --json
# {"total_keys":10,"total_bytes":5242880}
```

### Use Cases

**Action Cache**: Store build results keyed by input hash
```bash
# Bazel/Gradle-style action cache
INPUT_HASH=$(sha256sum inputs.txt | cut -d' ' -f1)
plasma kv put "action:$INPUT_HASH" --file result.json
```

**Build Metadata**: Store timestamps, versions, etc.
```bash
plasma kv put "last-build-time" "$(date -Iseconds)"
plasma kv put "app-version" "1.2.3"
```

## `plasma p2p`

Manage peer-to-peer cache sharing on local networks.

P2P cache sharing (Layer 0.5) allows automatic discovery and sharing of build caches between machines on the same network, with 1-5ms latency.

### Commands

```bash
# Generate a secure random secret
plasma p2p secret [--length <BYTES>]

# List discovered peers
plasma p2p list [--verbose] [--json]

# Show P2P status
plasma p2p status [--json]

# Approve a peer (grant cache access)
plasma p2p approve <PEER> [--permanent]

# Deny a peer (revoke cache access)
plasma p2p deny <PEER>

# Clear all consent records
plasma p2p clear [--force]
```

### Examples

```bash
# Generate a secure random secret (default: 32 bytes = 64 hex chars)
plasma p2p secret
# Output: 2295b4779c0fee78a732f249a32e25a03b7b3329db51719058b56aabae426d43

# Generate shorter secret (minimum 16 bytes for security)
plasma p2p secret --length 16
# Output: fc5d669b4220c90e3ac0e48c3c8fcaac

# Generate and save secret to environment
export P2P_SECRET=$(plasma p2p secret)

# Create config that uses the environment variable
cat >> .plasma.toml <<EOF
[p2p]
enabled = true
secret = "\${P2P_SECRET}"  # Uses environment variable
consent_mode = "notify-once"
EOF

# Start daemon with P2P enabled
plasma daemon start

# List discovered peers
plasma p2p list
# Output:
# [plasma] Discovered 2 peer(s):
#
#   • alice-macbook (192.168.1.100:7071)
#   • bob-desktop (192.168.1.101:7071)

# List peers with details
plasma p2p list --verbose
# Output:
# [plasma] Discovered 2 peer(s):
#
#   • alice-macbook (192.168.1.100:7071)
#     Machine ID: a3f5d9c2b1e8f7a4
#     Port: 7071
#     Accepting requests: true
#
#   • bob-desktop (192.168.1.101:7071)
#     Machine ID: b7e4a1f9c8d2e3f6
#     Port: 7071
#     Accepting requests: true

# Show P2P status
plasma p2p status
# Output:
# [plasma] P2P Cache Sharing Status
#
#   Enabled: true
#   Advertise: true
#   Discovery: true
#   Port: 7071
#   Consent mode: notify-once
#   Max peers: 10
#
#   Peers discovered: 2

# Approve peer permanently
plasma p2p approve alice-macbook --permanent
# Output: [plasma] Permanently approved peer: alice-macbook

# Approve peer for current session only
plasma p2p approve bob-desktop
# Output: [plasma] Approved peer for this session: bob-desktop

# Deny a peer
plasma p2p deny charlie-laptop
# Output: [plasma] Denied peer: charlie-laptop

# Clear all consent records
plasma p2p clear
# Output:
# [plasma] This will clear all stored P2P consents.
# [plasma] You will need to re-approve peers next time they request access.
# [plasma] Continue? [y/N] y
# [plasma] Cleared all P2P consents

# Force clear without confirmation
plasma p2p clear --force
```

### JSON Output

All P2P commands support `--json` for machine-readable output:

```bash
plasma p2p list --json
# [
#   {
#     "machine_id": "a3f5d9c2b1e8f7a4",
#     "hostname": "alice-macbook",
#     "address": "192.168.1.100",
#     "port": 7071,
#     "accepting_requests": true
#   },
#   {
#     "machine_id": "b7e4a1f9c8d2e3f6",
#     "hostname": "bob-desktop",
#     "address": "192.168.1.101",
#     "port": 7071,
#     "accepting_requests": true
#   }
# ]

plasma p2p status --json
# {
#   "enabled": true,
#   "advertise": true,
#   "discovery": true,
#   "bind_port": 7071,
#   "consent_mode": "notify-once",
#   "peers_discovered": 2,
#   "max_peers": 10
# }
```

### Configuration

P2P must be enabled in your `.plasma.toml`:

```toml
[p2p]
enabled = true
secret = "${P2P_SECRET}"        # Use env var (min 16 chars, shared across team)
consent_mode = "notify-once"    # notify-once | notify-always | always-allow
bind_port = 7071                # Port for P2P server (default: 7071)
advertise = true                # Advertise this machine to peers
discovery = true                # Discover other peers
max_peers = 10                  # Maximum number of peers to connect to
```

**Generate and set the secret:**

```bash
# Generate a secure secret
plasma p2p secret

# Add to your shell profile (~/.bashrc, ~/.zshrc, etc.)
export P2P_SECRET=<generated-secret>
```

### Consent Modes

- `notify-once` - System notification on first access, remembered
- `notify-always` - System notification every time
- `always-allow` - No notifications, always allow (use with caution)

### Security

- All P2P communication authenticated via HMAC-SHA256 with shared secret
- Replay protection with 5-minute time window
- User consent required before cache access
- Consent records stored in `~/.local/share/plasma/p2p/consents.json`

### Use Cases

**Team collaboration:**
```bash
# Same office network
# Developer A builds feature → Developer B instantly gets cached artifacts
# 1-5ms latency vs 20-50ms from cloud cache
```

**Multi-machine development:**
```bash
# MacBook + Linux desktop on same home network
# Build on one machine → cache available on other
```

**CI/CD optimization:**
```bash
# Multiple CI runners on same LAN
# First runner builds → subsequent runners use P2P cache
# Reduces cloud cache bandwidth costs
```

## Global Options

Available for all commands:

| Option | Description |
|--------|-------------|
| `--help`, `-h` | Show help information |
| `--version`, `-V` | Show version information |
| `--verbose`, `-v` | Enable verbose logging |
| `--quiet`, `-q` | Suppress non-error output |

## Environment Variables

### Authentication

- `TUIST_TOKEN` - Shorthand for authentication token
- `TUIST_CONFIG_AUTH_TOKEN` - Full form for authentication token

### AWS Credentials (for S3 upstream)

- `AWS_ACCESS_KEY_ID` - AWS access key
- `AWS_SECRET_ACCESS_KEY` - AWS secret key
- `AWS_REGION` - AWS region

### Configuration Prefix

Any configuration option can be set via `TUIST_CONFIG_*` environment variables:

```bash
export TUIST_CONFIG_CACHE_DIR=/tmp/cache
export TUIST_CONFIG_CACHE_MAX_SIZE=10GB
export TUIST_CONFIG_UPSTREAM_0_URL=grpc://cache.example.com:7070
```

## Exit Codes

| Code | Description |
|------|-------------|
| 0 | Success |
| 1 | General error |
| 2 | Configuration error |
| 3 | Authentication error |
| 130 | Interrupted by user (Ctrl+C) |
