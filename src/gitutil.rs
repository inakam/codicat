use anyhow::{Context, Result};
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(windows)]
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Gitのリポジトリルートディレクトリを取得する
pub fn get_git_root<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let output = Command::new("git")
        .args([
            "-C",
            path.as_ref().to_str().unwrap_or("."),
            "rev-parse",
            "--show-toplevel",
        ])
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        anyhow::bail!("Not a Git repository: {}", path.as_ref().display());
    }

    let root = String::from_utf8(output.stdout)
        .context("Git output is not valid UTF-8")?
        .trim()
        .to_string();

    Ok(PathBuf::from(root))
}

/// 指定ディレクトリ以下のGit管理下のファイル一覧を取得する
pub fn list_git_tracked_files<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>> {
    // 最初にGitリポジトリのルートディレクトリを取得
    let git_root = get_git_root(&dir)?;

    // サブディレクトリからの実行でも全ファイルを取得するため、
    // Gitリポジトリのルートディレクトリから実行する
    #[cfg(unix)]
    let output = Command::new("git")
        .args(["-C", git_root.to_str().unwrap_or("."), "ls-files", "-z"])
        .output()
        .context("Failed to execute git command")?;

    #[cfg(windows)]
    let output = Command::new("git")
        .args(["-C", git_root.to_str().unwrap_or("."), "ls-files"])
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        anyhow::bail!("Failed to list Git-tracked files");
    }

    #[cfg(unix)]
    {
        // -z オプションでNULL文字区切りの出力を取得し、バイナリデータとして処理
        let mut files = Vec::new();
        let mut start = 0;

        for (i, &byte) in output.stdout.iter().enumerate() {
            if byte == 0 {
                if i > start {
                    let file_path = &output.stdout[start..i];
                    // git_rootとファイルパスを結合
                    let path = git_root.join(Path::new(std::ffi::OsStr::from_bytes(file_path)));
                    files.push(path);
                }
                start = i + 1;
            }
        }

        Ok(files)
    }

    #[cfg(windows)]
    {
        // Windowsでは改行区切りの出力を処理
        let output_str = String::from_utf8(output.stdout)
            .context("Git output is not valid UTF-8")?;
        
        let files = output_str
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                // パスを適切に結合
                let path = git_root.join(line);
                path
            })
            .collect();
        
        Ok(files)
    }
}
