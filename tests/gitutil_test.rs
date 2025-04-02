use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use tempfile::TempDir;

use codicat::gitutil;

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
fn test_get_git_root() -> Result<()> {
    let repo = setup_git_repo()?;

    // リポジトリルートパスの取得
    let git_root = gitutil::get_git_root(repo.path())?;

    // 取得したGitルートパスが正しいことを確認
    assert_eq!(git_root, repo.path().canonicalize()?);

    // サブディレクトリからも同じGitルートを取得できることを確認
    let sub_dir = repo.path().join("sub");
    let git_root_from_sub = gitutil::get_git_root(&sub_dir)?;
    assert_eq!(git_root_from_sub, repo.path().canonicalize()?);

    Ok(())
}

#[test]
fn test_get_git_root_error() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Git管理下でないディレクトリに対してエラーになることを確認
    let result = gitutil::get_git_root(temp_dir.path());
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Not a Git repository"));

    Ok(())
}

#[test]
fn test_list_git_tracked_files() -> Result<()> {
    let repo = setup_git_repo()?;

    // Git管理下のファイル一覧を取得
    let files = gitutil::list_git_tracked_files(repo.path())?;

    // 作成したファイルがすべて含まれていることを確認
    assert_eq!(files.len(), 3);

    let file_paths: Vec<_> = files
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .collect();

    assert!(file_paths.contains(&"a.txt".to_string()));
    assert!(file_paths.contains(&"b.txt".to_string()));
    assert!(file_paths.contains(&"c.txt".to_string()));

    Ok(())
}

#[test]
fn test_list_git_tracked_files_from_subdirectory() -> Result<()> {
    let repo = setup_git_repo()?;

    // サブディレクトリからGit管理下のファイル一覧を取得
    let sub_dir = repo.path().join("sub");
    let files = gitutil::list_git_tracked_files(&sub_dir)?;

    // 全ファイルがパス付きで取得できることを確認（サブディレクトリから実行しても）
    assert_eq!(files.len(), 3);

    Ok(())
}

#[test]
fn test_list_git_tracked_files_error() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Git管理下でないディレクトリに対してエラーになることを確認
    let result = gitutil::list_git_tracked_files(temp_dir.path());
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

    // 空のGitリポジトリからファイル一覧を取得
    let files = gitutil::list_git_tracked_files(temp_dir.path())?;

    // ファイルが空であることを確認
    assert!(files.is_empty());

    Ok(())
}
