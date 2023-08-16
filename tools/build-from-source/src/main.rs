use std::{
    path::Path,
    process::{Command, Stdio},
};

fn run(mut cmd: Command) -> anyhow::Result<()> {
    Ok(cmd
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .status()
        .map(|_| ())?)
}

const fn yarn_cmd() -> &'static str {
    if cfg!(windows) {
        "yarn.cmd"
    } else {
        "yarn"
    }
}

fn main() -> anyhow::Result<()> {
    let m = Path::new(env!("CARGO_MANIFEST_DIR"));
    let project = m.parent().unwrap().parent().unwrap();

    println!("Running yarn install...");
    let mut cmd = Command::new(yarn_cmd());
    cmd.args(["install"]);
    cmd.current_dir(project.join("frontend"));
    run(cmd)?;

    println!("Running yarn build...");
    let mut cmd = Command::new(yarn_cmd());
    cmd.args(["build"]);
    cmd.current_dir(project.join("frontend"));
    run(cmd)?;

    println!("Running cargo build...");
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--release"]);
    run(cmd)?;

    Ok(())
}
