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

```
codicat
├── .github
│ └── workflows
│   ├── ci.yml
│   ├── format.yml
│   └── release.yml
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── README.ja.md
├── README.md
├── scripts
│ ├── build.sh
│ ├── package.sh
│ └── release.sh
├── src
│ ├── bin
│ │ └── generate_testdata.rs
│ ├── cli.rs
│ ├── fileview.rs
│ ├── gitutil.rs
│ ├── lib.rs
│ ├── main.rs
│ └── treeview.rs
└── tests
  ├── cli_test.rs
  ├── fileview_test.rs
  ├── gitutil_test.rs
  ├── testdata
  │ ├── README.md
  │ ├── golden
  │ │ ├── binary
  │ │ ├── default
  │ │ ├── filter
  │ │ ├── max-lines
  │ │ ├── no-content
  │ │ └── no-tree
  │ └── input
  │   ├── binary
  │   │ ├── a.txt
  │   │ ├── b.txt
  │   │ ├── binary.bin
  │   │ └── sub
  │   │   └── c.txt
  │   ├── default
  │   │ ├── a.txt
  │   │ ├── b.txt
  │   │ └── sub
  │   │   └── c.txt
  │   ├── filter
  │   │ ├── a.txt
  │   │ ├── b.txt
  │   │ ├── keep-me.txt
  │   │ ├── skip-me.txt
  │   │ └── sub
  │   │   ├── c.txt
  │   │   └── keep-also.txt
  │   ├── max-lines
  │   │ ├── a.txt
  │   │ ├── b.txt
  │   │ └── sub
  │   │   └── c.txt
  │   ├── no-content
  │   │ ├── a.txt
  │   │ ├── b.txt
  │   │ └── sub
  │   │   └── c.txt
  │   └── no-tree
  │     ├── a.txt
  │     ├── b.txt
  │     └── sub
  │       └── c.txt
  └── treeview_test.rs


/.github/workflows/ci.yml
--------------------------------------------------------------------------------
   1 | name: CI
   2 |
   3 | on:
   4 |   push:
   5 |     branches: [main]
   6 |   pull_request:
   7 |     branches: [main]
   8 |
   9 | env:
  10 |   CARGO_TERM_COLOR: always


--------------------------------------------------------------------------------
/.github/workflows/format.yml
--------------------------------------------------------------------------------
   1 | name: Format Code
   2 |
   3 | on:
   4 |   push:
   5 |     branches: [main]
   6 |   pull_request:
   7 |     branches: [main]
   8 |   workflow_dispatch:
   9 |
  10 | jobs:


--------------------------------------------------------------------------------
/.github/workflows/release.yml
--------------------------------------------------------------------------------
   1 | name: Release
   2 |
   3 | on:
   4 |   push:
   5 |     tags:
   6 |       - "v*"
   7 |
   8 | jobs:
   9 |   create-release:
  10 |     runs-on: ubuntu-latest

...
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
