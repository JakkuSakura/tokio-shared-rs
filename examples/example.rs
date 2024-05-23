use example_lib::exports::{FnRun, FnSetupTokio};
use std::time::Duration;
use std::{io::Write, path::PathBuf};
use tokio_shared::SharedTokioHandle;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // build example-lib
    let cdylib = build_dylib();

    // log in the normal program
    println!("program println!");

    let handle = SharedTokioHandle::new();
    // log in the rust dylib
    run_dylib(&handle);
    // log in the cdylib, see `example-lib/src/lib.rs`
    run_cdylib(cdylib, &handle);
    // wait for spawned tasks to finish
    tokio::time::sleep(Duration::from_secs(1)).await
}

fn run_dylib(logger: &SharedTokioHandle) {
    let _guard = example_lib::setup_shared_tokio_ref(&logger);
    example_lib::run("dylib");
}
fn run_cdylib(dylib: PathBuf, handle: &SharedTokioHandle) {
    let dylib = unsafe { libloading::Library::new(dylib) }.expect("error loading dylib");

    let setup_tokio: FnSetupTokio = unsafe { *dylib.get(b"setup_shared_tokio_ref").unwrap() };
    let _guard = setup_tokio(&handle);
    let run: FnRun = unsafe { *dylib.get(b"run").unwrap() };
    run("cdylib");
}
fn build_dylib() -> PathBuf {
    print!("building `example-lib`...");
    std::io::stdout().flush().unwrap();

    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("build")
        .arg("--manifest-path")
        .arg("examples/example-lib/Cargo.toml")
        .arg("--message-format")
        .arg("json");

    #[cfg(feature = "log")]
    cmd.arg("--features=log");

    let output = cmd.output().unwrap();
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("{error}");
        panic!("dylib build failed");
    }

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines().rev() {
        if line.starts_with(r#"{"reason":"compiler-artifact""#) {
            let files_start = r#""filenames":[""#;
            let i = line.find(files_start).unwrap();
            let line = &line[i + files_start.len()..];
            let i = line.find('"').unwrap();
            let dylib = &line[..i];

            println!(" done");

            return PathBuf::from(dylib);
        }
    }
    panic!("failed to find get dylib output");
}
