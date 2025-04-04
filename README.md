# codicat

Display directory tree and file contents from Git repositories.
Pass all code contents to LLM as prompts to understand the context.

## Installation

Install using Homebrew:

```sh
brew install inakam/tap/codicat
```

Download prebuilt binaries from the [Releases page](https://github.com/inakam/codicat/releases).

Or, if you have Rust installed, you can build from source:

```sh
cargo install --path .
```

## Usage

```sh
codicat [options] [path]
```

Run `codicat --help` to see available options.

## Features

- Display directory tree of Git-tracked files
- Show file contents with line numbers (UitHub style)
  - Binary files are omitted by default

### Options

| Option                | Description                                       |
| --------------------- | ------------------------------------------------- |
| `--max-lines`         | Limit the number of lines displayed per file      |
| `--no-tree`           | Disable tree view                                 |
| `--no-content`        | Disable file content display                      |
| `--token-count`       | Show token count                                  |
| `--copy`              | Copy output to clipboard                          |
| `--filter`            | Filter files based on regular expression patterns |
| `--fzf`               | Interactively select files (requires fzf)         |
| `--exclude-generated` | Exclude auto-generated files by checking headers  |
| `--help`              | Show help                                         |

## Example

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

## Development

### Testing

To run all tests:

```sh
cargo test
```

#### Test Structure

- **Unit Tests**: Tests individual module functionality
- **Integration Tests**: Verifies CLI behavior as a whole
- **Golden Tests**: Compares expected output with actual results

#### Generating Test Data

To generate test data:

```sh
cargo run --features=generate_testdata --bin generate_testdata
```

#### Updating Golden Files

To update golden files:

```sh
cargo test -- --ignored generate_golden
```

### CI/CD

This project uses the following GitHub Actions workflows:

#### CI (Continuous Integration)

Automatically runs on pushes to main branch and pull requests:

- Code formatting check
- Clippy lint execution
- All tests execution
- Automatic test data and golden file updates

#### Release

Automatically creates releases when a tag is pushed:

- Builds binaries for the following platforms:
  - Linux (x86_64, aarch64)
  - macOS (x86_64, aarch64)
  - Windows (x86_64)
- Automatic upload to release page
- Automatic upload to Homebrew repository

## License

MIT
