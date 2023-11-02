use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../sombra-lookup");
    std::env::set_current_dir("../sombra-lookup").unwrap();
    Command::new("trunk")
        .args(["build", "--release"])
        .status()
        .unwrap();
}
