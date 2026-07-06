use crate::util::{capture, run};

/// Quickdraw: close an issue or PR within 5 minutes of opening it.
/// Opens an issue on the given repo and closes it immediately.
pub fn run_cmd(repo: &str) -> Result<(), String> {
    println!("Opening issue on {repo}...");

    let issue_url = capture(
        "gh",
        &[
            "issue",
            "create",
            "--repo",
            repo,
            "--title",
            "Quickdraw achievement issue",
            "--body",
            "Opened to trigger the Quickdraw achievement. Safe to close/delete.",
        ],
    )?;
    println!("Created: {issue_url}");

    let issue_number = issue_url
        .rsplit('/')
        .next()
        .ok_or("could not parse issue number from URL")?;

    println!("Closing immediately...");
    run("gh", &["issue", "close", issue_number, "--repo", repo])?;

    println!("\nDone. Closed issue #{issue_number} within seconds of opening it.");
    println!("Quickdraw should appear on your profile within a few minutes.");
    Ok(())
}
