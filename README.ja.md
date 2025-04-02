# codicat

Git リポジトリからディレクトリツリーとファイル内容を表示します。

## 使い方

```sh
codicat [options] [path]
```

利用可能なオプションを確認するには `codicat --help` を実行してください。

## 機能

- Git 管理下にあるファイルのディレクトリツリーを表示
- ファイル内容を行番号付きで表示（uithHub 方式）
  - バイナリファイルはデフォルトで非表示

### オプション

| オプション     | 説明                                               |
| -------------- | -------------------------------------------------- |
| `--max-lines`  | ファイルごとの表示行数を制限                       |
| `--no-tree`    | ツリー表示を無効化                                 |
| `--no-content` | ファイル内容表示を無効化                           |
| `--copy`       | 出力をクリップボードにコピー                       |
| `--filter`     | 正規表現パターンに基づいてファイルをフィルタリング |
| `--fzf`        | 対話的にファイルを選択（fzf のインストールが必要） |

## インストール

[リリースページ](https://github.com/inakam/codicat/releases)から事前ビルド済みバイナリをダウンロードしてください。

あるいは、Rust がインストールされている場合はソースからビルドすることもできます：

```sh
cargo install --path .
```

その後、以下のように実行できます：

```sh
codicat --help
```

## 使用例

```sh
codicat --max-lines 10 .
```

## 開発

### テスト

すべてのテストを実行するには：

```sh
cargo test
```

#### テスト構造

- **Unit Tests**: テストは各モジュール機能を個別に検証
- **Integration Tests**: CLI ツール全体の動作を確認
- **Golden Tests**: 期待される出力と実際の出力を比較

#### テストデータの生成

テストデータを生成するには：

```sh
cargo run --features=generate_testdata --bin generate_testdata
```

#### ゴールデンファイルの更新

ゴールデンファイルを更新するには：

```sh
cargo test -- --ignored generate_golden
```

### CI/CD

このプロジェクトは以下の GitHub Actions ワークフローを使用しています：

#### CI (Continuous Integration)

PR や main ブランチへのプッシュで自動的に実行されます：

- コードフォーマットのチェック
- Clippy Lint の実行
- 全テストの実行
- テストデータとゴールデンファイルの自動更新

#### リリース

タグをプッシュすると自動的にリリースが作成されます：

- 次のプラットフォーム用のバイナリがビルドされます：
  - Linux (x86_64, aarch64)
  - macOS (x86_64, aarch64)
  - Windows (x86_64)
- リリースページへの自動アップロード

## ライセンス

MIT
