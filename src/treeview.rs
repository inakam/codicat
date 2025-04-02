use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

use crate::gitutil;

/// ツリーノードを表現する構造体
#[derive(Debug)]
pub struct TreeNode {
    /// ノード名
    name: String,
    /// ファイルかどうか
    is_file: bool,
    /// 子ノード
    children: BTreeMap<String, TreeNode>,
}

impl TreeNode {
    /// 新しいツリーノードを作成する
    fn new(name: &str, is_file: bool) -> Self {
        Self {
            name: name.to_string(),
            is_file,
            children: BTreeMap::new(),
        }
    }
}

/// 指定されたパスからGit管理下のファイルのツリービューを構築して表示する
pub fn tree_view_from_git<P: AsRef<Path>, W: Write>(input_path: P, writer: &mut W) -> Result<()> {
    let tree = build_tree_from_git(input_path)?;
    print_tree_root(&tree, writer)?;
    Ok(())
}

/// Git管理下のファイルからツリー構造を構築する
fn build_tree_from_git<P: AsRef<Path>>(input_path: P) -> Result<TreeNode> {
    let abs_input = input_path
        .as_ref()
        .canonicalize()
        .context("Failed to resolve input path")?;

    let git_root = gitutil::get_git_root(&abs_input)?;

    let rel_input_path = abs_input
        .strip_prefix(&git_root)
        .unwrap_or_else(|_| Path::new("."))
        .to_path_buf();

    let git_files = gitutil::list_git_tracked_files(&git_root)?;

    let relevant_paths = git_files
        .iter()
        .filter_map(|file| {
            let rel_path = file.strip_prefix(&git_root).ok()?;

            // ベースパスが"."の場合は全ファイル、そうでない場合は指定パス配下のファイルのみ
            if rel_input_path == Path::new(".")
                || rel_path == rel_input_path
                || rel_path.starts_with(&rel_input_path)
            {
                // 相対パスの先頭部分（rel_input_path）を除去
                let trimmed = rel_path.strip_prefix(&rel_input_path).unwrap_or(rel_path);
                Some(trimmed.to_path_buf())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if relevant_paths.is_empty() {
        anyhow::bail!("No Git-tracked files found under: {}", abs_input.display());
    }

    let root_name = abs_input
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| ".".to_string());

    let mut root = TreeNode::new(&root_name, false);

    for path in relevant_paths {
        insert_path(&mut root, &path);
    }

    Ok(root)
}

/// パスをツリー構造に挿入する
fn insert_path(node: &mut TreeNode, path: &Path) {
    let components: Vec<_> = path
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect();

    if components.is_empty() {
        return;
    }

    let mut current = node;

    for (i, component) in components.iter().enumerate() {
        if component.is_empty() {
            continue;
        }

        let is_file = i == components.len() - 1;

        if !current.children.contains_key(component) {
            current
                .children
                .insert(component.clone(), TreeNode::new(component, is_file));
        }

        current = current.children.get_mut(component).unwrap();
    }
}

/// ツリーのルートノードを表示する
fn print_tree_root<W: Write>(node: &TreeNode, writer: &mut W) -> Result<()> {
    writeln!(writer, "{}", node.name)?;
    print_tree_children(node, "", writer)?;
    Ok(())
}

/// ツリーの子ノードを再帰的に表示する
fn print_tree_children<W: Write>(node: &TreeNode, prefix: &str, writer: &mut W) -> Result<()> {
    for (i, (_, child)) in node.children.iter().enumerate() {
        let is_last_child = i == node.children.len() - 1;

        let connector = if is_last_child {
            "└──"
        } else {
            "├──"
        };
        let next_prefix = if is_last_child {
            format!("{}  ", prefix)
        } else {
            format!("{}│ ", prefix)
        };

        writeln!(writer, "{}{} {}", prefix, connector, child.name)?;

        if !child.is_file {
            print_tree_children(child, &next_prefix, writer)?;
        }
    }

    Ok(())
}
