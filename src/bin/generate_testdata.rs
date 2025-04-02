use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() -> Result<()> {
    println!("🔧 Start test data generation...");
    generate_testdata()
}

fn generate_testdata() -> Result<()> {
    let test_dir = Path::new("tests/testdata/input");

    let test_cases = vec![
        "default",
        "max-lines",
        "no-tree",
        "no-content",
        "filter",
        "binary",
    ];

    // Create directories for each test case
    for case in &test_cases {
        let case_dir = test_dir.join(case);
        fs::create_dir_all(&case_dir)?;
        fs::create_dir_all(case_dir.join("sub"))?;

        println!("📁 Created: {}", case_dir.display());

        // Create standard text files
        let files = vec![
            ("a.txt", "line 1\nline 2\nline 3\nline 4\nline 5\n"),
            ("b.txt", "line 1\nline 2\nline 3\nline 4\nline 5\n"),
            ("sub/c.txt", "line 1\nline 2\nline 3\nline 4\nline 5\n"),
        ];

        for (path, content) in &files {
            let file_path = case_dir.join(path);
            fs::write(&file_path, content)?;
        }

        // Only add files for filter case
        if *case == "filter" {
            fs::write(
                case_dir.join("keep-me.txt"),
                "line 1\nline 2\nline 3\nline 4\nline 5\n",
            )?;
            fs::write(
                case_dir.join("skip-me.txt"),
                "line 1\nline 2\nline 3\nline 4\nline 5\n",
            )?;
            fs::write(
                case_dir.join("sub").join("keep-also.txt"),
                "line 1\nline 2\nline 3\nline 4\nline 5\n",
            )?;
        }

        // Only add binary file for binary case
        if *case == "binary" {
            let binary_file = case_dir.join("a.txt");
            let mut file = File::create(&binary_file)
                .with_context(|| format!("Failed to create file: {}", binary_file.display()))?;
            file.write_all(b"\x00This is binary data")?;
        }
    }

    println!("✅ Test data generation completed: {}", test_dir.display());
    println!("To generate golden files: cargo test -- --ignored generate_golden");

    Ok(())
}
