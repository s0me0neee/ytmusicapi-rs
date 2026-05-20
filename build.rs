use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    // Ensure the uv venv and ytmusicapi are installed.
    let status = Command::new("uv")
        .args(["sync", "--quiet"])
        .current_dir(&manifest_dir)
        .status()
        .expect("Failed to run `uv sync`. Install uv: https://docs.astral.sh/uv/");
    assert!(status.success(), "uv sync failed");

    // Embed the absolute site-packages path so the library can inject it into
    // sys.path at runtime, regardless of where the binary is executed from.
    let lib_dir = manifest_dir.join(".venv/lib");
    if let Ok(entries) = std::fs::read_dir(&lib_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("python") {
                let site_packages = entry.path().join("site-packages");
                if site_packages.exists() {
                    println!(
                        "cargo:rustc-env=YTMUSICAPI_SITE_PACKAGES={}",
                        site_packages.display()
                    );
                    break;
                }
            }
        }
    }

    println!("cargo:rerun-if-changed=pyproject.toml");
    println!("cargo:rerun-if-changed=uv.lock");
}
