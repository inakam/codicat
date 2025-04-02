#!/usr/bin/env bash
set -e

# build.sh — マルチプラットフォーム向けのビルドを行うスクリプト
#
# 使い方:
# ./scripts/build.sh <app_name> <output_dir> [version]
#
# 例:
# ./scripts/build.sh codicat dist v2025.03.28
#
# 引数:
# <app_name> バイナリの名前（例: "codicat"）
# <output_dir> 出力ディレクトリ（例: "dist"）
# [version] オプションのバージョン文字列（デフォルト: "dev"）

APP_NAME="${1:-codicat}"
DIST="${2:-dist}"
VERSION="${3:-dev}"

PLATFORMS=(
  "x86_64-unknown-linux-gnu"
  "aarch64-unknown-linux-gnu"
  "x86_64-apple-darwin"
  "aarch64-apple-darwin"
)

mkdir -p "$DIST"

for target in "${PLATFORMS[@]}"; do
  echo "  -> Building for $target"
  
  # Linuxターゲットのビルド
  if [[ $target == *linux* ]]; then
    cargo build --release --target "$target"
    cp "target/$target/release/$APP_NAME" "$DIST/$APP_NAME-$target"
  
  # macOSターゲットのビルド（crossが必要）
  elif [[ $target == *apple* ]]; then
    if ! command -v cross &> /dev/null; then
      echo "crossが見つかりません。インストールしてください: cargo install cross"
      exit 1
    fi
    
    cross build --release --target "$target"
    cp "target/$target/release/$APP_NAME" "$DIST/$APP_NAME-$target"
  fi
done

echo "完了しました。バイナリは $DIST ディレクトリにあります。" 