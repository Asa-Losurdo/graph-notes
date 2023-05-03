use std::env;

pub fn main() {
    
    // exposes the profile to the main build script  
    // for conditional compilation 
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cfg
    // https://users.rust-lang.org/t/conditional-compilation-for-debug-release/1098/7
    // #[cfg(debug)] or #[cfg(release)] 
    let profile = env::var("PROFILE").expect("pack's build.rs failed to get env var 'PROFILE'");
    println!("cargo:rustc-cfg={profile}");
}