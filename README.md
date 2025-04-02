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

## License

MIT
