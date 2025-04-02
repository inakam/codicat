use anyhow::Result;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

use codicat::fileview;

#[test]
fn test_render_file_with_line_limit() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let text_file = tmp_dir.path().join("sample-head.txt");

    let content = "line 1\nline 2\nline 3\nline 4\nline 5\n";
    std::fs::write(&text_file, content)?;

    let mut buf = Vec::new();
    fileview::file_view_with_lines(&text_file, &mut buf, 3)?;

    let output = String::from_utf8(buf)?;

    // 最初の3行だけ表示されていることを確認
    assert!(output.contains("1 | line 1"));
    assert!(output.contains("2 | line 2"));
    assert!(output.contains("3 | line 3"));
    assert!(!output.contains("4 | line 4"));
    assert!(!output.contains("5 | line 5"));

    Ok(())
}

#[test]
fn test_render_full_file() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let text_file = tmp_dir.path().join("sample-full.txt");

    let content = "line A\nline B\nline C\n";
    std::fs::write(&text_file, content)?;

    let mut buf = Vec::new();
    fileview::file_view_with_lines(&text_file, &mut buf, 0)?;

    let output = String::from_utf8(buf)?;

    // 全行表示されていることを確認
    assert!(output.contains("1 | line A"));
    assert!(output.contains("2 | line B"));
    assert!(output.contains("3 | line C"));

    Ok(())
}

#[test]
fn test_directory_rejection() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let mut buf = Vec::new();

    // ディレクトリに対してエラーが発生することを確認
    let result = fileview::file_view_with_lines(tmp_dir.path(), &mut buf, 0);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("cannot render directory as file"));

    Ok(())
}

#[test]
fn test_binary_file_detection() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let binary_file = tmp_dir.path().join("binary.bin");

    // バイナリデータ（NULLバイトを含む）を作成
    let mut file = File::create(&binary_file)?;
    file.write_all(b"\x00This is binary data")?;

    let mut buf = Vec::new();
    fileview::file_view_with_lines(&binary_file, &mut buf, 0)?;

    let output = String::from_utf8(buf)?;

    // バイナリファイルとして検出され、内容が省略されていることを確認
    assert!(output.contains("[binary file omitted]"));
    assert!(!output.contains("This is binary data"));

    Ok(())
}

#[test]
fn test_file_header_format() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let text_file = tmp_dir.path().join("test-header.txt");

    std::fs::write(&text_file, "test content")?;

    let mut buf = Vec::new();
    fileview::file_view_with_lines(&text_file, &mut buf, 0)?;

    let output = String::from_utf8(buf)?;

    // ヘッダーが正しく表示されていることを確認
    assert!(output.contains(&format!(
        "/{}",
        text_file.file_name().unwrap().to_string_lossy()
    )));
    assert!(output.contains(&"-".repeat(80)));

    Ok(())
}

#[test]
fn test_file_footer_format() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let text_file = tmp_dir.path().join("test-footer.txt");

    std::fs::write(&text_file, "test content")?;

    let mut buf = Vec::new();
    fileview::file_view_with_lines(&text_file, &mut buf, 0)?;

    let output = String::from_utf8(buf)?;

    // フッターが正しく表示されていることを確認
    assert!(output.contains(&format!("\n\n{}", "-".repeat(80))));

    Ok(())
}
