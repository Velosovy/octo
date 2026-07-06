use crate::util::{capture, run, timestamp};
use std::fs::OpenOptions;
use std::io::Write;

/// Pair Extraordinaire: merge a PR containing a commit co-authored by a
/// second GitHub account. The email must belong to a real, verified
/// GitHub account you control -- GitHub checks this, it can't be faked
/// with an arbitrary address.
pub fn run_cmd(coauthor_name: &str, coauthor_email: &str) -> Result<(), String> {
    let default_branch = capture(
        "gh",
        &["repo", "view", "--json", "defaultBranchRef", "-q", ".defaultBranchRef.name"],
    )?;
    let branch = format!("pair-achievement-{}", timestamp());

    run("git", &["fetch", "origin", &default_branch])?;
    run(
        "git",
        &["checkout", "-b", &branch, &format!("origin/{default_branch}")],
    )?;

    {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("PAIR.md")
            .map_err(|e| format!("could not write PAIR.md: {e}"))?;
        writeln!(file, "pair achievement {}", timestamp())
            .map_err(|e| format!("could not write PAIR.md: {e}"))?;
    }

    run("git", &["add", "PAIR.md"])?;

    let message = format!(
        "chore: trigger Pair Extraordinaire achievement\n\nCo-authored-by: {coauthor_name} <{coauthor_email}>"
    );
    run("git", &["commit", "-m", &message])?;
    run("git", &["push", "-u", "origin", &branch])?;

    let pr_url = capture(
        "gh",
        &[
            "pr",
            "create",
            "--title",
            "Pair Extraordinaire achievement PR",
            "--body",
            "Co-authored commit to trigger the Pair Extraordinaire achievement.",
            "--base",
            &default_branch,
            "--head",
            &branch,
        ],
    )?;

    run("gh", &["pr", "merge", &pr_url, "--merge", "--delete-branch"])?;

    println!("\nDone. Merged {pr_url} with a Co-authored-by trailer for {coauthor_email}.");
    println!("Both accounts should see Pair Extraordinaire within a few minutes.");
    Ok(())
}
