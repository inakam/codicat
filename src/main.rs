use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

mod cli;
mod fileview;
mod gitutil;
mod treeview;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to render (defaults to current directory)
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Limit the number of lines printed per file
    #[arg(long, default_value_t = 500)]
    max_lines: usize,

    /// Do not render the tree view
    #[arg(long)]
    no_tree: bool,

    /// Do not render file contents
    #[arg(long)]
    no_content: bool,

    /// Copy output to clipboard
    #[arg(long)]
    copy: bool,

    /// Interactively select files via fzf (if installed)
    #[arg(long)]
    fzf: bool,

    /// Filter file paths with a regular expression
    #[arg(long)]
    filter: Option<String>,

    /// Display token count at the end of output
    #[arg(long)]
    token_count: bool,
}

fn main() -> Result<()> {
    let args = if std::env::args().len() <= 1 {
        Args::parse_from(vec![std::env::args().next().unwrap(), "--help".to_string()])
    } else {
        Args::parse()
    };

    let app = cli::App::new();
    app.execute(
        args.path,
        args.max_lines,
        args.no_tree,
        args.no_content,
        args.copy,
        args.fzf,
        args.filter,
        args.token_count,
    )
    .context("Failed to execute command")
}
