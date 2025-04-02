use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use tempfile::TempDir;

use codicat::treeview;

// テスト用のGitリポジトリをセットアップする
fn setup_git_repo() -> Result<TempDir> {
    let temp_dir = TempDir::new()?;

    // Gitリポジトリを初期化
    Command::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .output()?;

    // Gitユーザーを設定（コミットに必要）
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(temp_dir.path())
        .output()?;

    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(temp_dir.path())
        .output()?;

    // ファイル構造を作成
    let files = vec![
        ("a.txt", "content"),
        ("b.txt", "content"),
        ("sub/c.txt", "content"),
        ("sub/sub2/d.txt", "content"),
    ];

    for (path, content) in files {
        let file_path = temp_dir.path().join(path);
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut file = File::create(&file_path)?;
        file.write_all(content.as_bytes())?;
    }

    // ファイルをGitに追加
    Command::new("git")
        .args(["add", "."])
        .current_dir(temp_dir.path())
        .output()?;

    // コミット
    Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(temp_dir.path())
        .output()?;

    Ok(temp_dir)
}

#[test]
fn test_tree_view_from_git() -> Result<()> {
    let repo = setup_git_repo()?;

    let mut buf = Vec::new();
    treeview::tree_view_from_git(repo.path(), &mut buf)?;

    let output = String::from_utf8(buf)?;

    // ツリー構造にルートディレクトリ名が含まれていることを確認
    let repo_name = repo
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    assert!(output.contains(&repo_name));

    // ファイルとディレクトリが正しく表示されていることを確認
    assert!(output.contains("a.txt"));
    assert!(output.contains("b.txt"));
    assert!(output.contains("sub"));

    // ツリー構造の表現が正しいことを確認
    assert!(output.contains("├──") || output.contains("└──"));

    Ok(())
}

#[test]
fn test_non_git_directory() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let mut buf = Vec::new();

    // Git管理下でないディレクトリに対してエラーが発生することを確認
    let result = treeview::tree_view_from_git(tmp_dir.path(), &mut buf);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_empty_git_repo() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Gitリポジトリを初期化（ファイルなし）
    Command::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .output()?;

    let mut buf = Vec::new();
    let result = treeview::tree_view_from_git(temp_dir.path(), &mut buf);

    // Git管理下のファイルがないためエラーになることを確認
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No Git-tracked files found"));

    Ok(())
}

#[test]
fn test_subdirectory_tree() -> Result<()> {
    let repo = setup_git_repo()?;

    // サブディレクトリのツリーをテスト
    let sub_dir = repo.path().join("sub");
    let mut buf = Vec::new();
    treeview::tree_view_from_git(&sub_dir, &mut buf)?;

    let output = String::from_utf8(buf)?;

    // サブディレクトリの名前がルートとして表示されていることを確認
    assert!(output.contains("sub"));

    // サブディレクトリ以下のファイルだけが表示されていることを確認
    assert!(output.contains("c.txt"));
    assert!(output.contains("sub2"));
    assert!(!output.contains("a.txt"));
    assert!(!output.contains("b.txt"));

    Ok(())
}
