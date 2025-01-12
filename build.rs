use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Failed to retrieve Git commit hash");

    let git_hash = String::from_utf8(output.stdout).expect("Failed to convert Git hash to string");

    println!("cargo:rustc-env=INTEL_CPU_MANAGER_VERSION={}", git_hash.trim());
}

