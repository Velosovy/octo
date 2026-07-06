# gh-achievements

A small Rust CLI that automates the same legitimate actions you'd do by hand
to trigger self-service GitHub profile Achievements on a repo you own —
open/close an issue fast, merge a PR without review, make a co-authored
commit. It shells out to `git` and `gh` (GitHub CLI); it doesn't touch other
people's repos or do anything outside your own GitHub permissions.

## Requirements

- Rust toolchain (`cargo`) to build
- [`git`](https://git-scm.com/downloads)
- [`gh`](https://cli.github.com) (GitHub CLI), authenticated via `gh auth login`

## Build

```bash
cargo build --release
# binary at target/release/gh-achievements
```

Optionally install it to your PATH:

```bash
cargo install --path .
```

## Usage

```bash
gh-achievements doctor
```
Checks git/gh are installed and gh is authenticated.

```bash
gh-achievements checklist
```
Prints the full list of achievements, which ones this tool can trigger, and
which ones need real activity.

```bash
gh-achievements quickdraw <owner/repo>
```
**Quickdraw** — opens an issue on the given repo and closes it within
seconds. You already have this one, but it's here for a second account or
repo if you want it again there.

```bash
gh-achievements yolo
```
**YOLO** — merges a pull request without a review. Run this from inside a
local clone of a repo you own, on a clean working tree, with an `origin`
remote pointing at GitHub.

```bash
gh-achievements pair "Jane Doe" jane@example.com
```
**Pair Extraordinaire** — makes a commit with a `Co-authored-by:` trailer
and merges it in a PR. The email has to belong to a real, verified GitHub
account you control — GitHub checks this against actual accounts, so it
can't be earned with a made-up address. If you don't have a second account,
this one isn't fakeable and that's intentional.

## What this tool intentionally does NOT automate

- **Pull Shark** — needs real merged PRs, ideally into other people's repos.
  Contributing to real projects is the honest path.
- **Starstruck** — needs 16+ real stargazers on a repo you own. Build
  something worth starring.
- **Galaxy Brain** — needs a real accepted answer in a repo's Discussions
  tab. No shortcut, and manufacturing fake Q&A would just be spam.
- **Public Sponsor** — needs an actual sponsorship via
  [GitHub Sponsors](https://github.com/sponsors).

These are left out on purpose: automating them would mean either violating
GitHub's terms (fake stars/bots) or spamming other people's projects, and
this tool won't do either.

## Notes

- Achievements only reflect public activity by default. Enable "Show
  private contributions & achievements" in your profile settings if you
  want private repo work to count.
- Quickdraw and YOLO are one-time badges — running the command again on an
  account that already has the badge is a no-op as far as your profile
  goes.
- If a badge doesn't appear within ~24 hours of meeting the criteria,
  GitHub Support is the next step.
