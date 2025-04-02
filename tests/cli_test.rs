use anyhow::{Context, Result};
use assert_cmd::Command;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;

// テスト実行用のヘルパー関数
fn run_codicat_with_args(args: &[&str], current_dir: Option<&Path>) -> Result<(String, String)> {
    let mut cmd = Command::cargo_bin("codicat")?;

    if let Some(dir) = current_dir {
        cmd.current_dir(dir);
    }

    cmd.args(args);
    let output = cmd.output()?;

    let stdout = String::from_utf8(output.stdout).context("Failed to parse stdout")?;
    let stderr = String::from_utf8(output.stderr).context("Failed to parse stderr")?;

    Ok((stdout, stderr))
}

// 一時的なGitリポジトリを作成するヘルパー関数
fn setup_git_repo() -> Result<TempDir> {
    let temp_dir = TempDir::new()?;

    // Gitリポジトリを初期化
    Command::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .output()
        .context("Failed to initialize git repo")?;

    // Gitユーザーを設定（コミットに必要）
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(temp_dir.path())
        .output()
        .context("Failed to set git user.name")?;

    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(temp_dir.path())
        .output()
        .context("Failed to set git user.email")?;

    Ok(temp_dir)
}

// テスト用のディレクトリ構造を作成しGitに追加するヘルパー関数
fn create_test_files(repo_dir: &Path) -> Result<()> {
    // ディレクトリ構造を作成
    fs::create_dir_all(repo_dir.join("sub"))?;

    // ファイルを作成
    let files = vec![
        ("a.txt", "line 1\nline 2\nline 3\nline 4\nline 5\n"),
        ("b.txt", "line 1\nline 2\nline 3\nline 4\nline 5\n"),
        ("sub/c.txt", "line 1\nline 2\nline 3\nline 4\nline 5\n"),
    ];

    for (path, content) in files {
        let file_path = repo_dir.join(path);
        let mut file = File::create(&file_path)?;
        file.write_all(content.as_bytes())?;
    }

    // ファイルをGitに追加
    Command::new("git")
        .args(["add", "."])
        .current_dir(repo_dir)
        .output()
        .context("Failed to git add files")?;

    // コミット
    Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(repo_dir)
        .output()
        .context("Failed to commit")?;

    Ok(())
}

// バイナリファイルを作成するヘルパー関数
fn create_binary_file(repo_dir: &Path) -> Result<()> {
    let file_path = repo_dir.join("binary.bin");
    let mut file = File::create(&file_path)?;

    // バイナリデータ（NULLバイトを含む）
    let binary_data = b"\x00This is binary data";
    file.write_all(binary_data)?;

    // Gitに追加
    Command::new("git")
        .args(["add", "binary.bin"])
        .current_dir(repo_dir)
        .output()
        .context("Failed to git add binary file")?;

    // コミット
    Command::new("git")
        .args(["commit", "-m", "Add binary file"])
        .current_dir(repo_dir)
        .output()
        .context("Failed to commit binary file")?;

    Ok(())
}

#[test]
fn test_help_output() -> Result<()> {
    let (stdout, _) = run_codicat_with_args(&["--help"], None)?;

    // ヘルプ出力に期待される文字列が含まれていることを確認
    assert!(stdout.contains("codicat"));
    assert!(stdout.contains("--max-lines"));
    assert!(stdout.contains("--no-tree"));
    assert!(stdout.contains("--no-content"));

    Ok(())
}

#[test]
fn test_default_output() -> Result<()> {
    let repo = setup_git_repo()?;
    create_test_files(repo.path())?;

    let (stdout, _) = run_codicat_with_args(&[], Some(repo.path()))?;

    // デフォルト出力には、ツリー表示とファイル内容の両方が含まれている
    assert!(stdout.contains("a.txt"));
    assert!(stdout.contains("b.txt"));
    assert!(stdout.contains("sub"));
    assert!(stdout.contains("c.txt"));
    assert!(stdout.contains("line 1"));

    Ok(())
}

#[test]
fn test_max_lines_option() -> Result<()> {
    let repo = setup_git_repo()?;
    create_test_files(repo.path())?;

    let (stdout, _) = run_codicat_with_args(&["--max-lines", "2"], Some(repo.path()))?;

    // 各ファイルの最初の2行だけが表示されているか確認
    assert!(stdout.contains("1 | line 1"));
    assert!(stdout.contains("2 | line 2"));
    assert!(!stdout.contains("3 | line 3"));

    Ok(())
}

#[test]
fn test_no_tree_option() -> Result<()> {
    let repo = setup_git_repo()?;
    create_test_files(repo.path())?;

    let (stdout, _) = run_codicat_with_args(&["--no-tree"], Some(repo.path()))?;

    // ツリー表示がないが、ファイル内容は表示されているか確認
    assert!(!stdout.contains("├──"));
    assert!(!stdout.contains("└──"));
    assert!(stdout.contains("line 1"));

    Ok(())
}

#[test]
fn test_no_content_option() -> Result<()> {
    let repo = setup_git_repo()?;
    create_test_files(repo.path())?;

    let (stdout, _) = run_codicat_with_args(&["--no-content"], Some(repo.path()))?;

    // ツリー表示はあるが、ファイル内容がないか確認
    assert!(stdout.contains("├──") || stdout.contains("└──"));
    assert!(!stdout.contains("line 1"));

    Ok(())
}

#[test]
fn test_filter_option() -> Result<()> {
    let repo = setup_git_repo()?;
    create_test_files(repo.path())?;

    // a.txtのみをフィルタリング
    let (stdout, _) = run_codicat_with_args(&["--filter", "a\\.txt"], Some(repo.path()))?;

    // a.txtは含まれるが、b.txtやc.txtは含まれないか確認
    assert!(stdout.contains("a.txt"));
    assert!(!stdout.contains("/b.txt:"));
    assert!(!stdout.contains("/sub/c.txt:"));

    Ok(())
}

#[test]
fn test_binary_file_handling() -> Result<()> {
    let repo = setup_git_repo()?;
    create_test_files(repo.path())?;
    create_binary_file(repo.path())?;

    let (stdout, _) = run_codicat_with_args(&[], Some(repo.path()))?;

    // バイナリファイルは含まれるが、その内容は「omitted」と表示されるか確認
    assert!(stdout.contains("binary.bin"));
    assert!(stdout.contains("[binary file omitted]"));

    Ok(())
}

#[test]
fn test_non_git_directory() -> Result<()> {
    let temp_dir = TempDir::new()?;

    let (_, stderr) = run_codicat_with_args(&[], Some(temp_dir.path()))?;

    // Git管理下でないディレクトリに対するエラーメッセージが表示されるか確認
    assert!(
        stderr.contains("not inside a Git repository") || stderr.contains("Not a Git repository")
    );

    Ok(())
}

#[test]
fn test_file_specified() -> Result<()> {
    let repo = setup_git_repo()?;
    create_test_files(repo.path())?;

    let file_path = repo.path().join("a.txt");
    let (stdout, _) = run_codicat_with_args(&[file_path.to_str().unwrap()], Some(repo.path()))?;

    // 指定したファイルのみが表示されるか確認
    assert!(stdout.contains("a.txt"));
    assert!(!stdout.contains("/b.txt:"));

    Ok(())
}

// ゴールデンファイルを生成するテスト（通常はignore）
#[test]
#[ignore]
fn generate_golden() -> Result<()> {
    let golden_dir = Path::new("tests/testdata/golden");
    fs::create_dir_all(golden_dir)?;

    // 各テストケースに対してゴールデンファイルを生成
    let test_cases = vec![
        ("default", &[] as &[&str]),
        ("max-lines", &["--max-lines", "3"]),
        ("no-tree", &["--no-tree"]),
        ("no-content", &["--no-content"]),
        ("filter", &["--filter", "a\\.txt"]),
        ("binary", &[]),
    ];

    let repo = setup_git_repo()?;
    create_test_files(repo.path())?;
    create_binary_file(repo.path())?;

    for (name, args) in test_cases {
        let (stdout, _) = run_codicat_with_args(args, Some(repo.path()))?;

        let golden_file = golden_dir.join(name);
        fs::write(&golden_file, stdout)?;

        println!("Generated golden file: {}", golden_file.display());
    }

    Ok(())
}
