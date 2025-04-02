use anyhow::{Result, Context};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

use crate::gitutil;

/// ファイルの内容を行番号付きで出力する
pub fn file_view_with_lines<P: AsRef<Path>, W: Write>(
    path: P,
    writer: &mut W,
    max_lines: usize,
) -> Result<()> {
    let abs_path = path.as_ref().canonicalize()
        .context(format!("Failed to resolve absolute path: {}", path.as_ref().display()))?;
    
    if abs_path.is_dir() {
        anyhow::bail!("Cannot render directory as file: {}", abs_path.display());
    }
    
    if is_binary_file(&abs_path)? {
        print_file_header(&path, writer)?;
        writeln!(writer, "[binary file omitted]")?;
        print_file_footer(writer)?;
        return Ok(());
    }
    
    let file = File::open(&abs_path)
        .context(format!("Failed to open file: {}", abs_path.display()))?;
    
    print_file_header(&path, writer)?;
    print_file_body_with_lines(file, writer, max_lines)?;
    print_file_footer(writer)?;
    
    Ok(())
}

/// ファイルヘッダーを出力する
fn print_file_header<P: AsRef<Path>, W: Write>(path: P, writer: &mut W) -> Result<()> {
    let path = path.as_ref();
    
    // パスをGitルートからの相対パスに変換する
    if let Ok(git_root) = gitutil::get_git_root(path) {
        if let Ok(rel_to_git_root) = path.strip_prefix(&git_root) {
            writeln!(writer, "\n\n/{}", rel_to_git_root.to_string_lossy().replace('\\', "/"))?;
        } else {
            writeln!(writer, "\n\n/{}", path.to_string_lossy().replace('\\', "/"))?;
        }
    } else {
        // カレントディレクトリからの相対パスに変換
        let rel_path = if let Ok(cwd) = std::env::current_dir() {
            if let Ok(rel) = path.strip_prefix(&cwd) {
                rel.to_path_buf()
            } else {
                path.to_path_buf()
            }
        } else {
            path.to_path_buf()
        };
        
        writeln!(writer, "/{}", rel_path.to_string_lossy().replace('\\', "/"))?;
    }
    
    writeln!(writer, "{}", "-".repeat(80))?;
    Ok(())
}

/// ファイル内容を行番号付きで出力する
fn print_file_body_with_lines<R: Read, W: Write>(
    reader: R,
    writer: &mut W,
    max_lines: usize,
) -> Result<()> {
    let buf_reader = BufReader::new(reader);
    
    for (line_num, line) in buf_reader.lines().enumerate() {
        if max_lines > 0 && line_num >= max_lines {
            break;
        }
        
        let line = line.context("Error reading file")?;
        writeln!(writer, "{:4} | {}", line_num + 1, line)?;
    }
    
    Ok(())
}

/// ファイルフッターを出力する
fn print_file_footer<W: Write>(writer: &mut W) -> Result<()> {
    writeln!(writer, "\n\n{}", "-".repeat(80))?;
    Ok(())
}

/// バイナリファイルかどうかを判定する（最初の8000バイトにnull文字が含まれるかどうかで判定）
fn is_binary_file<P: AsRef<Path>>(path: P) -> Result<bool> {
    let mut file = File::open(path.as_ref())
        .context(format!("Failed to open file: {}", path.as_ref().display()))?;
    
    const MAX_BYTES: usize = 8000;
    let mut buf = vec![0u8; MAX_BYTES];
    
    let n = file.read(&mut buf)
        .context(format!("Failed to read file: {}", path.as_ref().display()))?;
    
    Ok(buf[..n].contains(&0))
} 