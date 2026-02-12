use anyhow::Result;
use rayon::prelude::*;
use regex::Regex;
use serde::Serialize;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
use walkdir::{DirEntry, WalkDir};

#[derive(Serialize)]
pub struct Finding {
    pub file: String,
    pub line: usize,
    pub content: String,
}
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with(".") && s != ".")
        .unwrap_or(false)
}
fn scan_file(path: &Path, re: &Regex) -> Result<Vec<Finding>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut res = Vec::new();
    for (index, line_result) in reader.lines().enumerate() {
        let line = match line_result {
            Ok(l) => l,
            Err(_) => break,
        };
        if re.is_match(&line) {
            let finding = Finding {
                file: path.display().to_string(),
                line: index + 1,
                content: line.trim().to_string(),
            };
            res.push(finding)
        }
    }
    Ok(res)
}

pub fn scan_directory(path: &Path) -> Result<Vec<Finding>> {
    let secret_regex = Regex::new(r"(?i)AWS_ACCESS_ID|password=|BEGIN RSA PRIVATE KEY").unwrap();
    let paths: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !is_hidden(&e))
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_owned())
        .collect();

    let findings: Vec<Finding> = paths
        .par_iter()
        .flat_map(|path| match scan_file(path, &secret_regex) {
            Ok(findings) => findings,
            Err(_) => Vec::new(),
        })
        .collect();
    Ok(findings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(file, "{}", content).expect("Failed to write to temp file");
        file
    }

    #[test]
    fn test_finding_struct_fields() {
        let finding = Finding {
            file: "test.txt".to_string(),
            line: 10,
            content: "secret".to_string(),
        };
        assert_eq!(finding.file, "test.txt");
        assert_eq!(finding.line, 10);
        assert_eq!(finding.content, "secret");
    }

    #[test]
    fn test_detects_dummy_aws_key() {
        let temp_file = create_test_file("Here is a key: AWS_ACCESS_ID=AKIAIOSFODNN7EXAMPLE");
        let re = Regex::new(r"(?i)AWS_ACCESS_ID|password=|BEGIN RSA PRIVATE KEY").unwrap();

        let findings = scan_file(temp_file.path(), &re).unwrap();

        assert_eq!(findings.len(), 1);
        assert!(findings[0].content.contains("AWS_ACCESS_ID"));
    }

    #[test]
    fn test_ignores_safe_text() {
        let temp_file = create_test_file("This is a safe file with no secrets.");
        let re = Regex::new(r"(?i)AWS_ACCESS_ID|password=|BEGIN RSA PRIVATE KEY").unwrap();

        let findings = scan_file(temp_file.path(), &re).unwrap();
        assert_eq!(findings.len(), 0);
    }
}
