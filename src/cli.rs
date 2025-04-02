use anyhow::{Context, Result};
use arboard::Clipboard;
use regex::Regex;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::fileview;
use crate::gitutil;
use crate::treeview;

/// アプリケーション構造体
pub struct App;

#[allow(clippy::too_many_arguments)]
impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// 新しいアプリケーションを作成する
    pub fn new() -> Self {
        App
    }

    /// コマンドを実行する
    pub fn execute<P: AsRef<Path>>(
        &self,
        input_path: P,
        max_lines: usize,
        no_tree: bool,
        no_content: bool,
        copy_to_clipboard: bool,
        use_fzf: bool,
        filter_pattern: Option<String>,
    ) -> Result<()> {
        let mut output = Vec::new();

        // ツリービューの表示
        if !no_tree {
            let mut writer = &mut output;
            if let Ok(()) = treeview::tree_view_from_git(&input_path, &mut writer) {
                writeln!(writer, "\n")?;
            }
        }

        // ファイル内容を表示しない場合は終了
        if no_content {
            self.finalize_output(&output, copy_to_clipboard)?;
            return Ok(());
        }

        // ファイル一覧を取得して内容を表示
        let path = input_path.as_ref();

        if path.is_file() {
            let mut writer = &mut output;
            fileview::file_view_with_lines(path, &mut writer, max_lines)?;
        } else {
            let files = self.list_git_files(path)?;
            let filtered_files = self.filter_files(files, filter_pattern)?;

            let selected_files = if use_fzf && self.is_fzf_installed() {
                self.select_files_with_fzf(&filtered_files)?
            } else {
                filtered_files
            };

            for file in selected_files {
                let mut writer = &mut output;
                fileview::file_view_with_lines(&file, &mut writer, max_lines)?;
            }
        }

        self.finalize_output(&output, copy_to_clipboard)?;

        Ok(())
    }

    /// 出力を標準出力とクリップボードに書き込む
    fn finalize_output(&self, output: &[u8], copy_to_clipboard: bool) -> Result<()> {
        // 出力を標準出力にコピー
        let stdout = io::stdout();
        let mut stdout_handle = stdout.lock();
        stdout_handle.write_all(output)?;

        // クリップボードにコピー
        if copy_to_clipboard {
            self.copy_to_clipboard(String::from_utf8_lossy(output).to_string())?;
        }

        Ok(())
    }

    /// Git管理下のファイル一覧を取得する
    fn list_git_files<P: AsRef<Path>>(&self, path: P) -> Result<Vec<PathBuf>> {
        match gitutil::list_git_tracked_files(path.as_ref()) {
            Ok(files) => {
                if files.is_empty() {
                    anyhow::bail!("No Git-tracked files found in: {}", path.as_ref().display());
                }
                Ok(files)
            }
            Err(err) => {
                if err.to_string().contains("Not a Git repository") {
                    anyhow::bail!(
                        "This directory is not inside a Git repository: {}",
                        path.as_ref().display()
                    );
                }
                anyhow::bail!("Failed to list files: {}", err);
            }
        }
    }

    /// 正規表現パターンに基づいてファイルをフィルタリングする
    fn filter_files(&self, files: Vec<PathBuf>, pattern: Option<String>) -> Result<Vec<PathBuf>> {
        if let Some(pattern) = pattern {
            let re = Regex::new(&pattern).context("Invalid regex pattern")?;

            let filtered = files
                .into_iter()
                .filter(|f| {
                    let path_str = f.to_string_lossy();
                    re.is_match(&path_str)
                })
                .collect();

            Ok(filtered)
        } else {
            Ok(files)
        }
    }

    /// fzfがインストールされているかチェックする
    fn is_fzf_installed(&self) -> bool {
        Command::new("which")
            .arg("fzf")
            .stdout(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }

    /// fzfを使ってファイルを選択する
    fn select_files_with_fzf(&self, files: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let input = files
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join("\n");

        let mut child = Command::new("fzf")
            .arg("--multi")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .context("Failed to spawn fzf")?;

        {
            let stdin = child.stdin.as_mut().context("Failed to open stdin")?;
            stdin.write_all(input.as_bytes())?;
        }

        let output = child.wait_with_output().context("Failed to wait for fzf")?;

        if !output.status.success() {
            anyhow::bail!("fzf returned with non-zero status");
        }

        let selection = String::from_utf8(output.stdout).context("Invalid UTF-8 in fzf output")?;

        let selected_files = selection
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(PathBuf::from)
            .collect();

        Ok(selected_files)
    }

    /// クリップボードにテキストをコピーする
    fn copy_to_clipboard(&self, text: String) -> Result<()> {
        let mut clipboard = Clipboard::new().context("Failed to access clipboard")?;
        clipboard
            .set_text(text)
            .context("Failed to copy to clipboard")?;

        eprintln!("✔️ Copied to clipboard.");
        Ok(())
    }
}
