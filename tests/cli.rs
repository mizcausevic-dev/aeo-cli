//! Integration tests that invoke the compiled `aeo` binary.
use std::path::PathBuf;
use std::process::Command;

fn bin() -> PathBuf {
    let target = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let exe = if cfg!(windows) { "aeo.exe" } else { "aeo" };
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(target)
        .join(exe)
}

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/aeo-person.json")
}

#[test]
fn validate_canonical_example_succeeds() {
    let output = Command::new(bin())
        .arg("validate")
        .arg(fixture())
        .output()
        .expect("run aeo binary");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Miz Causevic"),
        "stdout missing entity name: {stdout}"
    );
    assert!(
        stdout.contains("6 claims"),
        "stdout missing claim count: {stdout}"
    );
}

#[test]
fn validate_rejects_malformed_json() {
    let bad = tempfile_with_contents("{ not json");
    let output = Command::new(bin())
        .arg("validate")
        .arg(bad.path())
        .output()
        .expect("run aeo binary");
    assert!(!output.status.success(), "expected non-zero exit");
}

#[test]
fn inspect_prints_summary_for_local_file() {
    let output = Command::new(bin())
        .arg("inspect")
        .arg(fixture())
        .output()
        .expect("run aeo binary");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Person"), "missing entity type");
    assert!(stdout.contains("Miz Causevic"), "missing entity name");
    assert!(stdout.contains("Claims"), "missing claims header");
}

#[test]
fn claim_extracts_by_id() {
    let output = Command::new(bin())
        .arg("claim")
        .arg(fixture())
        .arg("years-experience")
        .output()
        .expect("run aeo binary");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"value\": 30"),
        "stdout missing claim value: {stdout}"
    );
}

#[test]
fn claim_returns_error_for_missing_id() {
    let output = Command::new(bin())
        .arg("claim")
        .arg(fixture())
        .arg("does-not-exist")
        .output()
        .expect("run aeo binary");
    assert!(
        !output.status.success(),
        "expected non-zero exit for missing claim"
    );
}

struct TempFile {
    path: PathBuf,
}

impl TempFile {
    fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

fn tempfile_with_contents(s: &str) -> TempFile {
    use std::io::Write;
    let dir = std::env::temp_dir();
    let path = dir.join(format!("aeo-cli-test-{}.json", std::process::id()));
    let mut f = std::fs::File::create(&path).expect("create temp file");
    f.write_all(s.as_bytes()).expect("write temp");
    TempFile { path }
}
