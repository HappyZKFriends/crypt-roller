use std::fs::copy;
use std::path::Path;
use std::path::PathBuf;

use tempfile;

pub fn temporary_test_dir(test_name: &str) -> tempfile::TempDir {
    tempfile::Builder::new()
        .prefix(&format!("crypt-roller-test-{}", test_name))
        .tempdir()
        .unwrap()
}

pub fn fixture_path(fixture_name: &str) -> PathBuf {
    [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "fixtures",
        fixture_name,
    ]
    .iter()
    .collect()
}

pub fn install_fixture(fixture_name: &str, test_dir: &Path) {
    let target_path = test_dir.join(fixture_name);
    copy(fixture_path(fixture_name), target_path).unwrap();
}
