use std::process::Command;

/// Runs a command, streaming its output live, and returns an error if it
/// exits non-zero. Used for steps where the user should see progress
/// (push, merge, etc).
pub fn run(cmd: &str, args: &[&str]) -> Result<(), String> {
    println!("$ {} {}", cmd, args.join(" "));
    let status = Command::new(cmd)
        .args(args)
        .status()
        .map_err(|e| format!("failed to run `{cmd}`: {e}"))?;

    if !status.success() {
        return Err(format!(
            "`{} {}` exited with status {:?}",
            cmd,
            args.join(" "),
            status.code()
        ));
    }
    Ok(())
}

/// Runs a command and captures trimmed stdout. Used when we need the
/// output value (e.g. a PR URL or issue number) rather than just to see it.
pub fn capture(cmd: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| format!("failed to run `{cmd}`: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "`{} {}` failed: {}",
            cmd,
            args.join(" "),
            stderr.trim()
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Checks a required CLI tool is on PATH and gives a friendly error if not.
pub fn require_tool(name: &str, install_hint: &str) -> Result<(), String> {
    match Command::new(name).arg("--version").output() {
        Ok(o) if o.status.success() => Ok(()),
        _ => Err(format!(
            "`{name}` is not installed or not on PATH.\n  Install it with: {install_hint}"
        )),
    }
}

pub fn timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
