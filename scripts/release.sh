#!/usr/bin/env bash
set -euo pipefail

# release.sh — GitHubリリースをトリガーするバージョンをタグ付けしてプッシュするスクリプト
#
# 使い方:
# ./scripts/release.sh vYYYY.MM.DD
#
# このスクリプトは以下を行います:
# - mainブランチにいることを確認
# - 作業ツリーがクリーンであることを確認
# - 最後のタグからの変更ログを表示
# - リリースタグを付けてプッシュ

VERSION="${1:-}"

if [[ -z "$VERSION" ]]; then
  echo "使い方: $0 vYYYY.MM.DD[-X]"
  exit 1
fi

branch=$(git rev-parse --abbrev-ref HEAD)
if [[ "$branch" != "main" ]]; then
  echo "❌ 'main'ブランチにいる必要があります（現在は'$branch'）"
  exit 1
fi

if [[ -n "$(git status --porcelain)" ]]; then
  echo "❌ 作業ディレクトリがクリーンではありません。変更をコミットまたはスタッシュしてください。"
  exit 1
fi

echo "📋 最後のタグからの変更ログ:"
LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")
if [[ -n "$LAST_TAG" ]]; then
  git log "$LAST_TAG"..HEAD --pretty=format:"- %s (%h)"
else
  git log --pretty=format:"- %s (%h)"
fi

echo
echo "🏷️ バージョンにタグ付け: $VERSION"
git tag "$VERSION"
git push origin main --tags

echo
echo "🚀 リリースが開始されました！GitHub Actionsがバイナリを公開します。" 