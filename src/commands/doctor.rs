use crate::util::{capture, require_tool};

/// Sanity-checks that git and gh are installed and gh is authenticated,
/// before any of the achievement commands try to use them.
pub fn run() -> Result<(), String> {
    println!("Checking prerequisites...\n");

    require_tool("git", "https://git-scm.com/downloads")?;
    println!("✔ git found");

    require_tool("gh", "https://cli.github.com")?;
    println!("✔ gh found");

    let status = capture("gh", &["auth", "status"]);
    match status {
        Ok(_) => println!("✔ gh is authenticated"),
        Err(e) => {
            return Err(format!(
                "gh is not authenticated. Run `gh auth login` first.\n({e})"
            ))
        }
    }

    println!("\nAll good. You can run: quickdraw, yolo, pair");
    Ok(())
}
