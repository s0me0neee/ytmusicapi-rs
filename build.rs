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
    let site_packages = find_site_packages(&manifest_dir)
        .expect("Could not find site-packages in .venv — run `uv sync` and retry.");
    println!(
        "cargo:rustc-env=YTMUSICAPI_SITE_PACKAGES={}",
        site_packages.display()
    );

    println!("cargo:rerun-if-changed=pyproject.toml");
    println!("cargo:rerun-if-changed=uv.lock");
}

fn find_site_packages(manifest_dir: &PathBuf) -> Option<PathBuf> {
    // Windows: .venv/Lib/site-packages (capital L, no pythonX.Y subdirectory)
    let win = manifest_dir.join(".venv/Lib/site-packages");
    if win.exists() {
        return Some(win);
    }
    // Unix: .venv/lib/pythonX.Y/site-packages
    if let Ok(entries) = std::fs::read_dir(manifest_dir.join(".venv/lib")) {
        for entry in entries.flatten() {
            if entry.file_name().to_string_lossy().starts_with("python") {
                let p = entry.path().join("site-packages");
                if p.exists() {
                    return Some(p);
                }
            }
        }
    }
    None
}
