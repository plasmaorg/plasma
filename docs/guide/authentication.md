# Authentication

Learn how to authenticate Plasma with remote cache servers using token-based or OAuth2 authentication.

## Overview

Plasma supports two authentication methods for connecting to remote cache servers:

- **Token-based**: Simple token authentication for CI/CD and automated workflows
- **OAuth2 with PKCE**: Secure, user-friendly authentication for interactive use

> [!TIP]
> Plasma automatically detects which method to use, so the same configuration works seamlessly in both local development (OAuth2) and CI/CD (token-based).

## Auto-Detection (Zero Config)

Plasma automatically detects which authentication method to use based on what's available:

```toml
# plasma.toml (works everywhere!)
url = "https://example.com"

[auth]
# No provider needed - auto-detects!

[auth.oauth2]
client_id = "plasma-cli"
scopes = "cache:read cache:write"
storage = "file"
```

**How it works:**

1. **PLASMA_AUTH_PROVIDER** env var set → Use specified provider (`token` or `oauth2`)
2. **PLASMA_TOKEN** env var present → Use token authentication
3. **OAuth2 token in storage** → Use OAuth2 (from previous `plasma auth login`)
4. **Config file `provider`** → Use explicit config setting
5. **Nothing available** → Error with helpful message

**Examples:**

```bash
# Local development: Login once, auto-uses OAuth2 thereafter
plasma auth login
plasma daemon  # ✅ Uses OAuth2 automatically

# CI/CD: Just set token, works automatically
export PLASMA_TOKEN=${{ secrets.PLASMA_TOKEN }}
plasma daemon  # ✅ Uses token automatically

# Explicit override (if needed)
export PLASMA_AUTH_PROVIDER=token
plasma daemon  # ✅ Forces token auth
```

> [!IMPORTANT]
> Auto-detection allows the same config file to work in both local development (OAuth2) and CI/CD (token-based):
> - ✅ Same config file for local dev and CI
> - ✅ No hardcoded auth methods
> - ✅ Works naturally with existing workflows
> - ✅ Explicit override when needed

## Authentication Methods

### Token-Based Authentication

Simple authentication using a static token. Best for CI/CD pipelines or when OAuth2 is not available.

#### Zero-Configuration (Convention-Based)

Plasma automatically checks for tokens in the standard environment variable:

```bash
# Use PLASMA_TOKEN (no config needed!)
export PLASMA_TOKEN="your-token-here"

# Verify authentication
plasma auth status
```

**Minimal config** (not even required with auto-detection):
```toml
[auth]
# That's it! PLASMA_TOKEN auto-detected

# Optional: Explicit provider (useful for debugging)
# provider = "token"
```

#### Custom Configuration

Override the default behavior if needed:

```toml
[auth]
provider = "token"

[auth.token]
# Option 1: Custom environment variable
env_var = "MY_CUSTOM_TOKEN_VAR"

# Option 2: File path (recommended for local development)
file = "~/.plasma/token"
```

#### Usage Examples

```bash
# Zero-config: Just set the env var
export PLASMA_TOKEN="your-token-here"
plasma daemon  # Works automatically!

# Custom env var
export MY_CUSTOM_TOKEN_VAR="your-token-here"
plasma daemon --config plasma.toml  # Uses custom env var from config

# File-based token
echo "your-token-here" > ~/.plasma/token
chmod 600 ~/.plasma/token
plasma daemon --config plasma.toml  # Reads from file

# Verify authentication
plasma auth status
```

### OAuth2 with PKCE Authentication

Secure authentication with automatic token refresh. Best for interactive use and development workflows.

#### Configuration

```toml
# Service URL (used for OAuth2, service discovery, etc.)
url = "https://example.com"

[auth]
# No provider needed - auto-detects OAuth2 after login!

# Optional: Explicit provider (useful for debugging)
# provider = "oauth2"

[auth.oauth2]
client_id = "plasma-cli"
scopes = "cache:read cache:write"
storage = "file"  # or "keychain" or "memory"

# Optional: Override service URL for OAuth2 specifically
# url = "https://custom-auth.example.com"

# Optional: Custom endpoints (defaults use url)
# authorization_endpoint = "https://example.com/oauth/authorize"
# token_endpoint = "https://example.com/oauth/token"
# device_authorization_endpoint = "https://example.com/oauth/device/code"
```

#### Storage Backends

Choose where to store OAuth2 tokens:

| Backend | Description | Use Case |
|---------|-------------|----------|
| `keychain` | OS credential manager (Keychain, Credential Manager, Secret Service) | **Recommended** for local development |
| `file` | File-based storage (XDG compliant: `~/.local/share/plasma/`) | Cross-process safe with file locking |
| `memory` | In-memory only | Temporary sessions, tokens lost on restart |

> [!TIP]
> Use `file` storage for maximum compatibility across platforms and processes. It follows XDG Base Directory Specification on Linux/Unix systems.

#### Login Flow

```bash
# Login with OAuth2
plasma auth login --config .plasma.toml
```

**Output:**
```
[plasma] Starting OAuth2 device code flow
[plasma] Please visit: https://example.com/activate
[plasma] Enter code: ABCD-EFGH
[plasma] Waiting for authorization...
✓ Successfully authenticated!
```

The device code flow:
1. Plasma generates a user code
2. You visit the activation URL in your browser
3. Enter the code and authorize
4. Token is securely stored

#### Token Refresh

OAuth2 tokens are automatically refreshed when:
- Token has 20% or less of its lifetime remaining (80% threshold)
- A request is made with an expired token

Token refresh is:
- **Cross-process safe**: Uses file locking to prevent concurrent refreshes
- **Transparent**: Happens automatically without user intervention
- **Efficient**: Proactive refresh prevents request delays

## Environment Variables Reference

| Variable | Purpose | Example |
|----------|---------|---------|
| `PLASMA_AUTH_PROVIDER` | Override auto-detection | `token` or `oauth2` |
| `PLASMA_TOKEN` | Provide authentication token | `eyJ0eXAi...` |
| `SCHLUSSEL_NO_BROWSER` | Disable browser opening (OAuth2) | `1` |
