pub fn run() {
    println!(
        r#"GitHub Achievements Checklist

Self-triggerable with this tool:
  [ ] Quickdraw           gh-achievements quickdraw <owner/repo>
  [ ] YOLO                gh-achievements yolo            (run inside a local clone)
  [ ] Pair Extraordinaire gh-achievements pair "Name" email@example.com

Need real activity (no honest shortcut):
  [ ] Pull Shark    merge PRs (own or others') - tiers at 2, 16, 32, 128
  [ ] Starstruck    get a repo to 16+ stars - tiers at 16, 128, 512, 4096
  [ ] Galaxy Brain  get an accepted answer in a repo's Discussions tab
  [ ] Public Sponsor sponsor someone at https://github.com/sponsors

Also worth doing:
  [ ] Enable "Show private contributions & achievements" in
      Settings -> Profile, if you want private repo activity to count
"#
    );
}
