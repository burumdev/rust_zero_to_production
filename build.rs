use std::process::Command;

fn main() {
    println!("cargo:warning=Running 'pnpm run build'...");
    // Running the build command
    let status = Command::new("pnpm")
        .arg("--prefix")
        .arg("frontend")
        .arg("run")
        .arg("build")
        .status()
        .expect("Failed to execute pnpm. Is pnpm installed and in your PATH?");

    if !status.success() {
        panic!("Frontend build failed with exit code: {:?}", status.code());
    }
}
