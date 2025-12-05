//! XDG Base Directory support for Plasma
//!
//! Follows the XDG Base Directory Specification:
//! - https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
//!
//! Directory structure:
//! - `$XDG_STATE_HOME/plasma/` (default: `~/.local/state/plasma/`) - Runtime state (daemon PIDs, ports)
//! - `$XDG_DATA_HOME/plasma/` (default: `~/.local/share/plasma/`) - User data (OAuth tokens)
//! - `$XDG_CACHE_HOME/plasma/` (default: `~/.cache/plasma/`) - Cache data
//! - `$XDG_CONFIG_HOME/plasma/` (default: `~/.config/plasma/`) - Configuration files

use std::path::PathBuf;

/// Get the Plasma state directory (for daemon state, PIDs, ports)
///
/// Respects XDG_STATE_HOME environment variable.
/// Falls back to `$HOME/.local/state/plasma` on Unix,  or appropriate path on other platforms.
///
/// # Example
/// ```
/// let state_dir = plasma::xdg::state_dir();
/// // Unix: ~/.local/state/plasma or $XDG_STATE_HOME/plasma
/// ```
pub fn state_dir() -> PathBuf {
    if let Ok(xdg_state) = std::env::var("XDG_STATE_HOME") {
        PathBuf::from(xdg_state).join("plasma")
    } else if let Some(home) = dirs::home_dir() {
        // XDG spec default: $HOME/.local/state
        home.join(".local").join("state").join("plasma")
    } else {
        // Fallback to current directory (should rarely happen)
        PathBuf::from(".plasma-state")
    }
}

/// Get the Plasma data directory (for OAuth tokens, persistent user data)
///
/// Respects XDG_DATA_HOME environment variable.
/// Falls back to `$HOME/.local/share/plasma` on Unix, or appropriate path on other platforms.
///
/// # Example
/// ```
/// let data_dir = plasma::xdg::data_dir();
/// // Unix: ~/.local/share/plasma or $XDG_DATA_HOME/plasma
/// ```
#[allow(dead_code)]
pub fn data_dir() -> PathBuf {
    if let Ok(xdg_data) = std::env::var("XDG_DATA_HOME") {
        PathBuf::from(xdg_data).join("plasma")
    } else if let Some(data) = dirs::data_dir() {
        // Uses platform-appropriate data directory
        data.join("plasma")
    } else if let Some(home) = dirs::home_dir() {
        // XDG spec default: $HOME/.local/share
        home.join(".local").join("share").join("plasma")
    } else {
        PathBuf::from(".plasma-data")
    }
}

/// Get the Plasma cache directory (for build caches)
///
/// Respects XDG_CACHE_HOME environment variable.
/// Falls back to `$HOME/.cache/plasma` on Unix, or appropriate path on other platforms.
///
/// # Example
/// ```
/// let cache_dir = plasma::xdg::cache_dir();
/// // Unix: ~/.cache/plasma or $XDG_CACHE_HOME/plasma
/// ```
#[allow(dead_code)]
pub fn cache_dir() -> PathBuf {
    if let Ok(xdg_cache) = std::env::var("XDG_CACHE_HOME") {
        PathBuf::from(xdg_cache).join("plasma")
    } else if let Some(cache) = dirs::cache_dir() {
        cache.join("plasma")
    } else if let Some(home) = dirs::home_dir() {
        // XDG spec default: $HOME/.cache
        home.join(".cache").join("plasma")
    } else {
        PathBuf::from(".plasma-cache")
    }
}

/// Get the daemon state directory
///
/// Used for storing daemon PIDs, ports, and runtime state.
///
/// # Example
/// ```
/// let daemon_dir = plasma::xdg::daemon_state_dir();
/// // Unix: ~/.local/state/plasma/daemons
/// ```
pub fn daemon_state_dir() -> PathBuf {
    state_dir().join("daemons")
}

/// Get the OAuth tokens directory
///
/// Used by schlussel for file-based OAuth token storage.
///
/// # Example
/// ```
/// let oauth_dir = plasma::xdg::oauth_tokens_dir();
/// // Unix: ~/.local/share/plasma/oauth-tokens
/// ```
#[allow(dead_code)]
pub fn oauth_tokens_dir() -> PathBuf {
    data_dir().join("oauth-tokens")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_dir_respects_xdg_env() {
        std::env::set_var("XDG_STATE_HOME", "/tmp/test-state");
        let dir = state_dir();
        assert_eq!(dir, PathBuf::from("/tmp/test-state/plasma"));
        std::env::remove_var("XDG_STATE_HOME");
    }

    #[test]
    fn test_data_dir_respects_xdg_env() {
        std::env::set_var("XDG_DATA_HOME", "/tmp/test-data");
        let dir = data_dir();
        assert_eq!(dir, PathBuf::from("/tmp/test-data/plasma"));
        std::env::remove_var("XDG_DATA_HOME");
    }

    #[test]
    fn test_cache_dir_respects_xdg_env() {
        std::env::set_var("XDG_CACHE_HOME", "/tmp/test-cache");
        let dir = cache_dir();
        assert_eq!(dir, PathBuf::from("/tmp/test-cache/plasma"));
        std::env::remove_var("XDG_CACHE_HOME");
    }

    #[test]
    fn test_daemon_state_dir() {
        std::env::set_var("XDG_STATE_HOME", "/tmp/test-state");
        let dir = daemon_state_dir();
        assert_eq!(dir, PathBuf::from("/tmp/test-state/plasma/daemons"));
        std::env::remove_var("XDG_STATE_HOME");
    }

    #[test]
    fn test_oauth_tokens_dir() {
        std::env::set_var("XDG_DATA_HOME", "/tmp/test-data");
        let dir = oauth_tokens_dir();
        assert_eq!(dir, PathBuf::from("/tmp/test-data/plasma/oauth-tokens"));
        std::env::remove_var("XDG_DATA_HOME");
    }
}
