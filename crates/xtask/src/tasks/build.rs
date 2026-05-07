use super::check::run;
use super::test::test;
use crate::flags::{Build, Test};

pub fn build(cmd: Build) {
    test(Test { filter: None });
    run(&["build", "--workspace", "--release"]);
    let mut install_args = vec!["install", "--path", "crates/korvex-cli"];
    if cmd.force {
        install_args.push("--force");
    }
    run(&install_args);
}
