mod commands;
mod util;

use std::env;
use std::process::ExitCode;

const USAGE: &str = "\
gh-achievements - trigger GitHub profile achievements on repos you own

USAGE:
    gh-achievements doctor
    gh-achievements checklist
    gh-achievements quickdraw <owner/repo>
    gh-achievements yolo
    gh-achievements pair <coauthor-name> <coauthor-email>

NOTES:
    - Requires `git` and `gh` (GitHub CLI) on PATH, with `gh auth login` done.
    - `yolo` and `pair` must be run from inside a local clone of a repo you
      own, with an `origin` remote and a clean working tree.
    - Quickdraw and YOLO are one-time badges; running twice won't earn twice.
";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let result = match args.get(1).map(String::as_str) {
        Some("doctor") => commands::doctor::run(),
        Some("checklist") => {
            commands::checklist::run();
            Ok(())
        }
        Some("quickdraw") => match args.get(2) {
            Some(repo) => commands::quickdraw::run_cmd(repo),
            None => Err("quickdraw needs a repo argument, e.g. `gh-achievements quickdraw owner/repo`".into()),
        },
        Some("yolo") => commands::yolo::run_cmd(),
        Some("pair") => match (args.get(2), args.get(3)) {
            (Some(name), Some(email)) => commands::pair::run_cmd(name, email),
            _ => Err("pair needs a name and email, e.g. `gh-achievements pair \"Jane Doe\" jane@example.com`".into()),
        },
        _ => {
            println!("{USAGE}");
            return ExitCode::SUCCESS;
        }
    };

    if let Err(e) = result {
        eprintln!("\nError: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
