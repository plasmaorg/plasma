use std::path::PathBuf;

/// Get default cache directory following XDG conventions
///
/// - Linux/Unix: $XDG_CACHE_HOME/plasma or ~/.cache/plasma
/// - macOS: ~/Library/Caches/plasma
/// - Windows: %LOCALAPPDATA%/plasma/cache
#[allow(dead_code)]
pub fn default_cache_dir() -> PathBuf {
    if let Some(cache_dir) = dirs::cache_dir() {
        cache_dir.join("plasma")
    } else {
        // Fallback to current directory if we can't determine cache dir
        PathBuf::from(".plasma/cache")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_cache_dir() {
        let cache_dir = default_cache_dir();
        assert!(cache_dir.to_string_lossy().contains("plasma"));
    }
}
