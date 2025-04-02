#!/usr/bin/env bash
set -euo pipefail

# package.sh — ビルド済みバイナリを.tar.gzアーカイブにパッケージングするスクリプト
#
# 使い方:
# ./scripts/package.sh <app_name> <output_dir>
#
# 例:
# ./scripts/package.sh codicat dist
#
# 各アーカイブにはバイナリとREADME.mdが含まれます

APP_NAME=${1:-codicat}
DIST=${2:-dist}
ARCHIVES=$DIST/archives
PLATFORMS=("linux-amd64" "linux-arm64" "darwin-amd64" "darwin-arm64")

TARGET_MAP=(
  "linux-amd64:x86_64-unknown-linux-gnu"
  "linux-arm64:aarch64-unknown-linux-gnu"
  "darwin-amd64:x86_64-apple-darwin"
  "darwin-arm64:aarch64-apple-darwin"
)

mkdir -p "$ARCHIVES"

for platform in "${PLATFORMS[@]}"; do
  # ターゲットトリプルを検索
  target=""
  for mapping in "${TARGET_MAP[@]}"; do
    if [[ $mapping == $platform:* ]]; then
      target=${mapping#*:}
      break
    fi
  done
  
  binary="$DIST/$APP_NAME-$target"

  if [[ ! -f "$binary" ]]; then
    echo "!! スキップ: $platform — バイナリが見つかりません"
    continue
  fi

  echo "-> パッケージング: $platform"
  mkdir -p "$DIST/tmp/$platform"
  cp "$binary" "$DIST/tmp/$platform/$APP_NAME"
  cp README.md "$DIST/tmp/$platform/README.md"
  tar -czf "$ARCHIVES/$APP_NAME-$platform.tar.gz" -C "$DIST/tmp/$platform" "$APP_NAME" README.md
  rm -rf "$DIST/tmp/$platform"
done

echo "パッケージングが完了しました。アーカイブは $ARCHIVES ディレクトリにあります。" 