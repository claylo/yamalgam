//! Generate C baseline cache files for compliance testing.
//!
//! Runs all YAML Test Suite cases through `fyaml-tokenize --batch` (2 process
//! spawns total instead of ~700) and writes JSON cache files to `target/`.
//!
//! Usage: `cargo run -p yamalgam-compare --bin generate_baseline`

fn main() {
    let workspace_root = find_workspace_root();
    let test_dir = workspace_root.join("vendor/yaml-test-suite");
    let output_dir = workspace_root.join("target");

    if !test_dir.exists() {
        eprintln!("Test suite not found at {}", test_dir.display());
        std::process::exit(1);
    }

    if let Err(e) = yamalgam_compare::c_baseline::generate(&test_dir, &output_dir) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }

    eprintln!("Baseline generation complete.");
}

fn find_workspace_root() -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string()),
    );
    // Walk up from crates/yamalgam-compare to workspace root.
    let mut dir = manifest_dir.as_path();
    loop {
        if dir.join("Cargo.lock").exists() {
            return dir.to_path_buf();
        }
        dir = match dir.parent() {
            Some(p) => p,
            None => return manifest_dir,
        };
    }
}
