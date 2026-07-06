use crate::util::{capture, run, timestamp};
use std::fs::OpenOptions;
use std::io::Write;

/// YOLO: merge a pull request without requesting/getting a review.
/// Must be run from inside a local clone of a repo you own, with an
/// "origin" remote and a clean working tree.
pub fn run_cmd() -> Result<(), String> {
    let default_branch = capture(
        "gh",
        &["repo", "view", "--json", "defaultBranchRef", "-q", ".defaultBranchRef.name"],
    )?;
    let branch = format!("yolo-achievement-{}", timestamp());

    println!("Creating branch {branch} off {default_branch}...");
    run("git", &["fetch", "origin", &default_branch])?;
    run(
        "git",
        &["checkout", "-b", &branch, &format!("origin/{default_branch}")],
    )?;

    {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("YOLO.md")
            .map_err(|e| format!("could not write YOLO.md: {e}"))?;
        writeln!(file, "yolo achievement {}", timestamp())
            .map_err(|e| format!("could not write YOLO.md: {e}"))?;
    }

    run("git", &["add", "YOLO.md"])?;
    run(
        "git",
        &["commit", "-m", "chore: trigger YOLO achievement"],
    )?;
    run("git", &["push", "-u", "origin", &branch])?;

    println!("Opening PR...");
    let pr_url = capture(
        "gh",
        &[
            "pr",
            "create",
            "--title",
            "YOLO achievement PR",
            "--body",
            "Opened to trigger the YOLO achievement. Merging without review.",
            "--base",
            &default_branch,
            "--head",
            &branch,
        ],
    )?;

    println!("Merging without review...");
    run("gh", &["pr", "merge", &pr_url, "--merge", "--delete-branch"])?;

    println!("\nDone. Merged {pr_url} without a review.");
    println!("YOLO should appear on your profile within a few minutes.");
    Ok(())
}
