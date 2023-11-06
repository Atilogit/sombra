use std::process::Command;

fn main() {
    let release = std::env::var("PROFILE").unwrap() == "release";
    println!("cargo:rerun-if-changed=../sombra-lookup");
    std::env::set_current_dir("../sombra-lookup").unwrap();
    let mut c = Command::new("trunk");
    c.arg("build");
    // trunk causes deadlock when crate and trunk are building the same profile
    // see Cargo.toml
    if !release {
        c.arg("--release");
    }
    c.status().unwrap();
    if release {
        for f in std::fs::read_dir("dist").unwrap() {
            let f = f.unwrap();
            if f.file_name().to_str().unwrap().ends_with(".wasm") {
                wasm_opt::OptimizationOptions::new_optimize_for_size_aggressively()
                    .run(f.path(), f.path())
                    .unwrap();
            }
        }
    }
}
