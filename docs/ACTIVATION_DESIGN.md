# Plasma Activation-Based Architecture

## Overview

Drawing inspiration from Mise, Plasma uses an activation-based approach for managing cache daemons, avoiding the need to wrap build tool commands.

## Commands

### `plasma activate`

**Purpose**: Shell integration for automatic daemon management

**Behavior**:
1. Traverses directory tree upward to find `.plasma.toml`
2. Computes configuration hash
3. Checks if daemon with that config is already running
4. If not running, starts daemon as background process
5. Exports environment variables for build tools to consume
6. Cleans up daemons from previous directories (optional)

**Shell Integration**:
```bash
# In ~/.bashrc or ~/.zshrc
eval "$(plasma activate bash)"  # or zsh, fish
```

**Generated Hook** (example):
```bash
_plasma_hook() {
  eval "$(plasma activate --status)"
}

# Run on directory change
if [[ -n "${ZSH_VERSION}" ]]; then
  chpwd_functions+=(_plasma_hook)
elif [[ -n "${BASH_VERSION}" ]]; then
  PROMPT_COMMAND="_plasma_hook${PROMPT_COMMAND:+;$PROMPT_COMMAND}"
fi
```

### `plasma exec <command>`

**Purpose**: Execute command with guaranteed daemon lifecycle

**Behavior**:
1. Finds `.plasma.toml` in current directory tree
2. Starts daemon if not running
3. Sets environment variables
4. Executes command
5. **Optionally** kills daemon when command exits (configurable)

**Usage**:
```bash
plasma exec bazel build //...
plasma exec nx build demo
plasma exec gradle build
```

**Lifecycle Options**:
- `--keep-alive` - Don't kill daemon after command exits (default)
- `--kill-after` - Kill daemon when command completes

## Configuration Discovery

**Search Path**:
1. `$PWD/.plasma.toml`
2. `$PWD/../.plasma.toml`
3. Continue up to root
4. Fallback to `~/.config/plasma/config.toml` (global)

**Config Hash**:
- SHA256 of canonical config content
- Used to uniquely identify daemon instances
- Different configs = different daemons

## Daemon Management

### Daemon State Directory

```
~/.plasma/daemons/<config-hash>/
├── pid              # Process ID (for signal sending)
├── ports.json       # HTTP/gRPC ports allocated
├── env              # Environment variables to export
├── config.toml      # Resolved configuration
└── socket           # Unix socket path (alternative to HTTP)
```

### Daemon Lifecycle

1. **Start**: `plasma activate` or `plasma exec` starts daemon
2. **Health Check**: Periodic HTTP GET to `/health`
3. **Stop**: 
   - `plasma deactivate` (explicit)
   - `plasma activate` in directory with different config (optional cleanup)
   - `plasma exec --kill-after` (when command exits)

### Port Allocation

- **Random ports** assigned on daemon start
- Stored in `~/.plasma/daemons/<hash>/ports.json`:
```json
{
  "http": 58234,
  "grpc": 58235,
  "metrics": 58236
}
```

## Environment Variables

### Standard Variables (Always Exported)

```bash
PLASMA_HTTP_URL=http://127.0.0.1:58234
PLASMA_GRPC_URL=grpc://127.0.0.1:58235
PLASMA_CONFIG_HASH=abc123def456
PLASMA_DAEMON_PID=12345
```

### Build Tool Convenience Variables (Optional)

```bash
# Gradle
GRADLE_BUILD_CACHE_URL=$PLASMA_HTTP_URL

# Nx
NX_SELF_HOSTED_REMOTE_CACHE_SERVER=$PLASMA_HTTP_URL

# Bazel (requires .bazelrc usage since no env var support)
# Users add to .bazelrc:
# build --remote_cache=$PLASMA_GRPC_URL

# Xcode (custom)
XCODE_CACHE_SERVER=$PLASMA_HTTP_URL
```

### Authentication

If `.plasma.toml` contains `auth.token`, export:
```bash
PLASMA_TOKEN=eyJ0eXAi...
```

Build tools can then use:
```bash
# Gradle (via init script or gradle.properties)
# Nx (via nx.json or NX_CLOUD_AUTH_TOKEN)
# etc.
```

## Shell Integration Output

When `plasma activate` runs, it outputs shell commands to eval:

```bash
$ plasma activate --status
export PLASMA_HTTP_URL=http://127.0.0.1:58234
export PLASMA_GRPC_URL=grpc://127.0.0.1:58235
export PLASMA_CONFIG_HASH=abc123
export GRADLE_BUILD_CACHE_URL=http://127.0.0.1:58234
export NX_SELF_HOSTED_REMOTE_CACHE_SERVER=http://127.0.0.1:58234
# Started daemon with PID 12345
```

## User Workflows

### Workflow 1: Shell Activation (Automatic)

```bash
# Setup (once)
echo 'eval "$(plasma activate bash)"' >> ~/.bashrc

# Usage (automatic on cd)
cd ~/my-project          # Daemon starts automatically
bazel build //...        # Uses PLASMA_GRPC_URL from env
nx build demo            # Uses NX_SELF_HOSTED_REMOTE_CACHE_SERVER
gradle build             # Uses GRADLE_BUILD_CACHE_URL

cd ~/other-project       # Different daemon starts
cd ~                     # Daemons cleaned up (optional)
```

### Workflow 2: Explicit Execution (CI-friendly)

```bash
# CI or one-off builds
plasma exec bazel build //...
plasma exec nx build demo
plasma exec gradle build

# Keeps daemon alive for subsequent commands
plasma exec --keep-alive nx build demo
nx test demo  # Reuses daemon
plasma deactivate  # Explicit cleanup
```

### Workflow 3: Long-running Daemon

```bash
# Start daemon explicitly
plasma daemon start

# Use in multiple terminals/commands
bazel build //...  # Terminal 1
nx build demo      # Terminal 2

# Stop when done
plasma daemon stop
```

## Implementation Phases

### Phase 1: Basic Daemon Management
- [ ] Config discovery (traverse directories)
- [ ] Config hashing
- [ ] Daemon start/stop
- [ ] PID/port file management
- [ ] Health checks

### Phase 2: Activation Command
- [ ] `plasma activate --status` (check/start daemon)
- [ ] Environment variable generation
- [ ] Shell integration (`plasma activate bash/zsh/fish`)

### Phase 3: Exec Command
- [ ] `plasma exec <command>`
- [ ] Environment variable passing
- [ ] Lifecycle management (--keep-alive, --kill-after)

### Phase 4: Daemon Cleanup
- [ ] Orphan daemon detection
- [ ] Automatic cleanup on `activate` (optional)
- [ ] `plasma daemon list/stop/clean` commands

## Benefits Over Wrapper Approach

1. **No Command Wrapping**: Build tools run as-is
2. **Flexible**: Works with shell activation OR explicit exec
3. **CI-Friendly**: `plasma exec` for CI, `plasma activate` for dev
4. **Multi-Tool**: One daemon serves all build tools
5. **Configuration-Driven**: Different projects, different daemons
6. **Clean State**: Daemons tracked and cleanable

## Migration from Wrapper Approach

Old approach:
```bash
plasma bazel -- build //...
plasma nx -- build demo
```

New approach (shell activation):
```bash
eval "$(plasma activate bash)"
bazel build //...  # Just works
nx build demo      # Just works
```

New approach (explicit):
```bash
plasma exec bazel build //...
plasma exec nx build demo
```
