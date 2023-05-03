use std::process::Command;

fn main() {
    //pass through --release flag
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "-p", "graph-notes", "--target", "wasm32-wasi"]);

    #[cfg(release)] 
    cmd.arg("--release");

    cmd.status()
    .expect("Failed to execute command");

    // encode wasm
}
