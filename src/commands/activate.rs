use anyhow::{Context, Result};
use std::env;
use std::time::Duration;

use crate::cli::ActivateArgs;
use crate::config_discovery::{discover_config, hash_config, DaemonState};

pub fn run(args: ActivateArgs) -> Result<()> {
    // If shell specified, output shell integration hook
    if let Some(shell) = args.shell {
        output_shell_hook(&shell)?;
        return Ok(());
    }

    // If --status, check/start daemon and output env vars
    if args.status {
        activate_current_directory()?;
        return Ok(());
    }

    // Default: show help
    println!("Usage:");
    println!("  plasma activate <shell>    Generate shell integration hook");
    println!("  plasma activate --status   Check/start daemon and export env vars");
    println!();
    println!("Shells: bash, zsh, fish");

    Ok(())
}

fn output_shell_hook(shell: &str) -> Result<()> {
    match shell {
        "bash" => {
            println!(
                r#"_plasma_hook() {{
  eval "$(plasma activate --status 2>/dev/null)"
}}

# Run on directory change
if [[ -n "${{PROMPT_COMMAND}}" ]]; then
  PROMPT_COMMAND="_plasma_hook;${{PROMPT_COMMAND}}"
else
  PROMPT_COMMAND="_plasma_hook"
fi
"#
            );
        }
        "zsh" => {
            println!(
                r#"_plasma_hook() {{
  eval "$(plasma activate --status 2>/dev/null)"
}}

# Run on directory change
autoload -U add-zsh-hook
add-zsh-hook chpwd _plasma_hook

# Run now
_plasma_hook
"#
            );
        }
        "fish" => {
            println!(
                r#"function _plasma_hook --on-variable PWD
  plasma activate --status 2>/dev/null | source
end

# Run now
_plasma_hook
"#
            );
        }
        _ => {
            anyhow::bail!("Unsupported shell: {}. Use bash, zsh, or fish", shell);
        }
    }

    Ok(())
}

fn activate_current_directory() -> Result<()> {
    let current_dir = env::current_dir().context("Failed to get current directory")?;

    // Find config
    let config_path = match discover_config(&current_dir)? {
        Some(path) => path,
        None => {
            // No config found, unset variables
            println!("# No plasma.toml found");
            output_unset_env_vars("bash");
            return Ok(());
        }
    };

    // Compute config hash
    let config_hash = hash_config(&config_path)?;

    // Check if daemon already running
    if let Some(state) = DaemonState::load(&config_hash)? {
        if state.is_running() {
            // Daemon running, export env vars
            println!("{}", state.generate_env_exports("bash"));
            return Ok(());
        } else {
            // Daemon state exists but process is dead, clean it up
            let _ = state.cleanup();
        }
    }

    // Need to start daemon
    println!(
        "# Starting Plasma daemon for config: {}",
        config_path.display()
    );

    // Start daemon process in background
    start_daemon_background(&config_path, &config_hash)?;

    // Load the state and export env vars
    if let Some(state) = DaemonState::load(&config_hash)? {
        println!("{}", state.generate_env_exports("bash"));
    } else {
        println!("# Warning: Daemon started but state not found");
    }

    Ok(())
}

fn start_daemon_background(config_path: &std::path::Path, config_hash: &str) -> Result<()> {
    use std::process::{Command, Stdio};

    // Get the current executable path
    let exe = env::current_exe().context("Failed to get current executable path")?;

    // Spawn plasma daemon with the config file
    Command::new(&exe)
        .arg("daemon")
        .arg("--config")
        .arg(config_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .context("Failed to spawn daemon process")?;

    // Wait for daemon to write its state file (with timeout)
    // The daemon needs to:
    // 1. Start up
    // 2. Bind to port 0 (get actual port)
    // 3. Write state file with actual ports
    let max_wait = Duration::from_secs(5);
    let check_interval = Duration::from_millis(100);
    let start = std::time::Instant::now();

    loop {
        // Try to load the state
        if let Some(state) = DaemonState::load(config_hash)? {
            // Verify the daemon is actually running
            if state.is_running() {
                // Success! Daemon is running and state is valid
                return Ok(());
            }
        }

        // Check if we've exceeded the timeout
        if start.elapsed() > max_wait {
            anyhow::bail!("Timeout waiting for daemon to start (5 seconds)");
        }

        // Wait a bit before checking again
        std::thread::sleep(check_interval);
    }
}

fn output_unset_env_vars(shell: &str) {
    match shell {
        "fish" => {
            println!("set -e PLASMA_HTTP_URL 2>/dev/null");
            println!("set -e PLASMA_GRPC_URL 2>/dev/null");
            println!("set -e PLASMA_UNIX_SOCKET 2>/dev/null");
            println!("set -e PLASMA_CONFIG_HASH 2>/dev/null");
            println!("set -e PLASMA_DAEMON_PID 2>/dev/null");
            println!("set -e GRADLE_BUILD_CACHE_URL 2>/dev/null");
            println!("set -e NX_SELF_HOSTED_REMOTE_CACHE_SERVER 2>/dev/null");
            println!("set -e XCODE_CACHE_SERVER 2>/dev/null");
            println!("set -e TURBO_API 2>/dev/null");
            println!("set -e TURBO_TEAM 2>/dev/null");
            println!("set -e TURBO_TOKEN 2>/dev/null");
        }
        _ => {
            println!("unset PLASMA_HTTP_URL");
            println!("unset PLASMA_GRPC_URL");
            println!("unset PLASMA_UNIX_SOCKET");
            println!("unset PLASMA_CONFIG_HASH");
            println!("unset PLASMA_DAEMON_PID");
            println!("unset GRADLE_BUILD_CACHE_URL");
            println!("unset NX_SELF_HOSTED_REMOTE_CACHE_SERVER");
            println!("unset XCODE_CACHE_SERVER");
            println!("unset TURBO_API");
            println!("unset TURBO_TEAM");
            println!("unset TURBO_TOKEN");
        }
    }
}
