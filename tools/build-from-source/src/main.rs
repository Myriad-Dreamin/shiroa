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

const fn pnpm_cmd() -> &'static str {
    if cfg!(windows) {
        "pnpm.cmd"
    } else {
        "pnpm"
    }
}

fn main() -> anyhow::Result<()> {
    let m = Path::new(env!("CARGO_MANIFEST_DIR"));
    let project = m.parent().unwrap().parent().unwrap();

    println!("Running pnpm build...");
    let mut cmd = Command::new(pnpm_cmd());
    cmd.args(["run", "build"]);
    cmd.current_dir(project.join("frontend"));
    run(cmd)?;

    // copy to assets\artifacts\book.mjs
    let src = project.join("frontend/dist/book.mjs");
    let dst = project.join("assets/artifacts/shiroa.js");
    std::fs::copy(src, dst)?;

    // copy typst ts renderer wasm module
    let src = project.join(
        "frontend/node_modules/@myriaddreamin/typst-ts-renderer/pkg/typst_ts_renderer_bg.wasm",
    );
    let dst = project.join("assets/artifacts/typst_ts_renderer_bg.wasm");
    std::fs::copy(src, dst)?;

    println!("Running cargo build...");
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--release"]);
    run(cmd)?;

    Ok(())
}
