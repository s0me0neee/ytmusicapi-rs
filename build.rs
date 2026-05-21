use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    // Venv goes in OUT_DIR so build.rs never writes to the source tree,
    // which is required for `cargo publish` compliance.
    let venv_dir = out_dir.join("venv");

    let mut uv = Command::new("uv");
    // --frozen: use the committed uv.lock without re-resolving; this ensures
    // the build is reproducible and prevents uv.lock from being written to
    // the source tree (required for `cargo publish` compliance).
    uv.args(["sync", "--quiet", "--frozen"])
        .current_dir(&manifest_dir)
        .env("UV_PROJECT_ENVIRONMENT", &venv_dir);

    // pyo3-build-config selects Python via PYO3_PYTHON → VIRTUAL_ENV → PATH (in
    // that order).  Pass the same interpreter to uv so the venv packages always
    // match the Python version that pyo3 will link against.
    uv.env("UV_PYTHON", pyo3_python());

    let status = uv
        .status()
        .expect("Failed to run `uv sync`. Install uv: https://docs.astral.sh/uv/");
    assert!(status.success(), "uv sync failed");

    let site_packages = find_site_packages(&venv_dir)
        .expect("Could not find site-packages in venv — run `uv sync` manually and retry.");
    println!(
        "cargo:rustc-env=YTMUSICAPI_SITE_PACKAGES={}",
        site_packages.display()
    );

    println!("cargo:rerun-if-changed=pyproject.toml");
}

/// Return the Python interpreter that pyo3-build-config will choose, so uv
/// creates the runtime venv with the same Python version that gets linked.
/// Mirrors pyo3-build-config's detection order: PYO3_PYTHON → VIRTUAL_ENV → PATH.
fn pyo3_python() -> String {
    if let Ok(p) = std::env::var("PYO3_PYTHON") {
        return p;
    }
    if let Ok(venv) = std::env::var("VIRTUAL_ENV") {
        let py = if cfg!(windows) {
            format!("{venv}/Scripts/python.exe")
        } else {
            format!("{venv}/bin/python3")
        };
        if std::path::Path::new(&py).exists() {
            return py;
        }
    }
    // Fall through to whatever is on PATH — same as pyo3-build-config's last resort.
    if Command::new("python3")
        .arg("-c")
        .arg("1")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        return "python3".to_string();
    }
    "python".to_string()
}

fn find_site_packages(venv_dir: &PathBuf) -> Option<PathBuf> {
    // Windows: venv/Lib/site-packages (capital L, no pythonX.Y subdirectory)
    let win = venv_dir.join("Lib/site-packages");
    if win.exists() {
        return Some(win);
    }
    // Unix: venv/lib/pythonX.Y/site-packages
    if let Ok(entries) = std::fs::read_dir(venv_dir.join("lib")) {
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
