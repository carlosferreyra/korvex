mod flags;
mod tasks;

fn main() {
    let flags = flags::Xtask::from_env_or_exit();
    match flags.subcommand {
        flags::XtaskCmd::Check(_) => tasks::check(),
        flags::XtaskCmd::Test(cmd) => tasks::test(cmd),
        flags::XtaskCmd::Build(cmd) => tasks::build(cmd),
    }
}
