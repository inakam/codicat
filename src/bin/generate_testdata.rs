use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() -> Result<()> {
    println!("🔧 テストデータ生成を開始します...");
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
    
    // テストケースごとにディレクトリを作成
    for case in &test_cases {
        let case_dir = test_dir.join(case);
        fs::create_dir_all(&case_dir)?;
        fs::create_dir_all(case_dir.join("sub"))?;
        
        println!("📁 作成: {}", case_dir.display());
        
        // 標準テキストファイル作成
        let files = vec![
            ("a.txt", "line 1\nline 2\nline 3\nline 4\nline 5\n"),
            ("b.txt", "line 1\nline 2\nline 3\nline 4\nline 5\n"),
            ("sub/c.txt", "line 1\nline 2\nline 3\nline 4\nline 5\n"),
        ];
        
        for (path, content) in &files {
            let file_path = case_dir.join(path);
            fs::write(&file_path, content)?;
        }
        
        // filterケースのみ追加ファイル
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
        
        // binaryケースのみバイナリファイル
        if *case == "binary" {
            let binary_file = case_dir.join("a.txt");
            let mut file = File::create(&binary_file)
                .with_context(|| format!("Failed to create file: {}", binary_file.display()))?;
            file.write_all(b"\x00This is binary data")?;
        }
    }
    
    println!("✅ テストデータの生成が完了しました: {}", test_dir.display());
    println!("ゴールデンファイルを生成するには: cargo test -- --ignored generate_golden");
    
    Ok(())
} 