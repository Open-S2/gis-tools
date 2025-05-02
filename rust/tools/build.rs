use std::{env, path::Path};

pub fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = Path::new(&manifest_dir).ancestors().nth(2).unwrap();
    println!("cargo:rustc-env=CARGO_WORKSPACE_DIR={}", workspace_root.display());
}
