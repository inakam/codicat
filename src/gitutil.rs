use anyhow::{Context, Result};
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
    let output = Command::new("git")
        .args(["-C", dir.as_ref().to_str().unwrap_or("."), "ls-files"])
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        anyhow::bail!("Failed to list Git-tracked files");
    }

    let content = String::from_utf8(output.stdout).context("Git output is not valid UTF-8")?;

    let files = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let path = dir.as_ref().join(line);
            path
        })
        .collect();

    Ok(files)
}
