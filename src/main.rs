use std::process::Command;
use glob::glob;

fn main() {

    //pass through --release flag
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "-p", "graph-notes", "--target", "wasm32-wasi"]);

    #[cfg(release)] 
    cmd.arg("--release");

    cmd.status()
    .expect("Failed to execute command");



    // get wasm files
    let mut pattern = "./target/wasm32-wasi/".to_owned();
    #[cfg(debug)]{
       pattern = pattern + "debug"; 
    }
    #[cfg(release)]{
        pattern = pattern + "release";
    }
    pattern = pattern + "/*.wasm";
    let paths = glob(&pattern).expect("Failed to read glob pattern");

    for e in paths {
        println!("{}", e.unwrap().display());
    }

}
