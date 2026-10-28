use std::process::Command;

fn main() {
    // Skip frontend build if running containerized in Docker
    if std::env::var("DOCKER_BUILD_SKIP_FRONTEND").is_ok() {
        println!(
            "cargo:warning=Skipping frontend build in build.rs. This is already handled by earlier Docker stage."
        );
        return;
    }

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
