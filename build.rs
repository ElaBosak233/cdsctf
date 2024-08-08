use std::process::Command;

fn main() {
    println!(
        "cargo:rustc-env=GIT_COMMIT={}",
        String::from_utf8(
            Command::new("git")
                .args(&["rev-parse", "HEAD"])
                .output()
                .expect("Failed to execute git command")
                .stdout,
        )
        .expect("Invalid UTF-8 sequence")
        .trim()
    );

    println!(
        "cargo:rustc-env=BUILD_AT={}",
        chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string()
    );
}
