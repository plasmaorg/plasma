/// `plasma cache` command implementation (DEPRECATED)
///
/// This module is kept for backward compatibility during migration.
/// The `plasma cache` command has been split into:
/// - `plasma cas` - Content-Addressed Storage operations
/// - `plasma kv` - Key-Value storage operations
/// - `plasma run --status/--list/--stats` - Script cache management
///
/// This stub prints a deprecation warning.
use anyhow::Result;

#[allow(dead_code)]
pub async fn cache_deprecated() -> Result<()> {
    eprintln!("WARNING: The `plasma cache` command is deprecated.");
    eprintln!();
    eprintln!("Please use the new commands:");
    eprintln!("  - `plasma cas` - Content-Addressed Storage operations");
    eprintln!("  - `plasma kv` - Key-Value storage operations");
    eprintln!("  - `plasma run --status <script>` - Check script cache status");
    eprintln!("  - `plasma run --list` - List cached scripts");
    eprintln!("  - `plasma run --stats` - Show cache statistics");
    eprintln!();
    eprintln!("See `plasma cas --help`, `plasma kv --help`, or `plasma run --help` for details.");

    std::process::exit(1);
}
