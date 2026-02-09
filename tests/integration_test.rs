use rusty_warden::scan_directory;
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_integration_full_scan() {
    let dir = tempdir().expect("failed to create temp dir");
    let root_path = dir.path();

    {
        let safe_path = root_path.join("safe.txt");
        let mut safe_file = File::create(&safe_path).unwrap();
        writeln!(safe_file, "Nothing to see here").unwrap();
    }

    {
        let secret_path = root_path.join("secrets.env");
        let mut secret_file = File::create(&secret_path).unwrap();
        writeln!(secret_file, "AWS_ACCESS_ID=AKIAIOSFODNN7EXAMPLE").unwrap();
    }

    {
        let hidden_dir = root_path.join(".ssh");
        fs::create_dir(&hidden_dir).unwrap();
        let hidden_secret_path = hidden_dir.join("id_rsa");
        let mut hidden_file = File::create(&hidden_secret_path).unwrap();
        writeln!(hidden_file, "BEGIN RSA PRIVATE KEY").unwrap();
    }

    let findings = scan_directory(root_path).expect("Scanner failed");

    assert_eq!(findings.len(), 1, "Expected exactly 1 finding");

    let finding = &findings[0];
    assert!(finding.content.contains("AWS_ACCESS_ID"));
    assert!(finding.file.ends_with("secrets.env"));
}
