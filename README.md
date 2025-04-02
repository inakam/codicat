# codicat

Display directory tree and file contents from Git repositories.

## Usage

```sh
codicat [options] [path]
```

Run `codicat --help` to see available options.

## Features

- Display directory tree of Git-tracked files
- Show file contents with line numbers (GitHub style)
  - Binary files are omitted by default

### Options

| Option         | Description                                       |
| -------------- | ------------------------------------------------- |
| `--max-lines`  | Limit the number of lines displayed per file      |
| `--no-tree`    | Disable tree view                                 |
| `--no-content` | Disable file content display                      |
| `--copy`       | Copy output to clipboard                          |
| `--filter`     | Filter files based on regular expression patterns |
| `--fzf`        | Interactively select files (requires fzf)         |

## Installation

Download prebuilt binaries from the [Releases page](https://github.com/inakam/codicat/releases).

Or, if you have Rust installed, you can build from source:

```sh
cargo install --path .
```

Then you can run it:

```sh
codicat --help
```

## Example

```sh
codicat --max-lines 10 .
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

## License

MIT
