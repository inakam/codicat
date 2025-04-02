以下の Go 製のツールを Rust に移行したいです。

以下のような機能を備えています：

- Git 管理下のファイルのみを対象に、ディレクトリツリーとファイル内容を表示
- 各ファイルの内容には行番号を付与
- バイナリファイルはデフォルトで非表示
- --max-lines オプションで、ファイルごとの表示行数を制限
- --no-tree や --no-content オプションで、ツリーや内容の表示を省略可能

このツールは以下のようなファイル群やコードで生成されており、最終的に出力される形式も以下のようなテキスト形式です。

├── .github
└── workflows
│ ├── ci.yml
│ └── release.yml
├── Makefile
├── README.md
├── RELEASE.md
├── cmd
└── uit
│ ├── main.go
│ └── main_test.go
├── go.mod
├── go.sum
├── internal
├── cli
│ ├── cli.go
│ ├── cli_test.go
│ └── testdata
│ │ ├── README.md
│ │ ├── golden
│ │ ├── .keep
│ │ ├── binary
│ │ ├── copy
│ │ ├── default
│ │ ├── filter
│ │ ├── max-lines
│ │ ├── no-content
│ │ └── no-tree
│ │ └── input
│ │ ├── .keep
│ │ ├── binary
│ │ ├── a.txt
│ │ ├── b.txt
│ │ └── sub
│ │ │ └── c.txt
│ │ ├── copy
│ │ ├── a.txt
│ │ ├── b.txt
│ │ └── sub
│ │ │ └── c.txt
│ │ ├── default
│ │ ├── a.txt
│ │ ├── b.txt
│ │ └── sub
│ │ │ └── c.txt
│ │ ├── filter
│ │ ├── a.txt
│ │ ├── b.txt
│ │ ├── keep-me.txt
│ │ ├── skip-me.txt
│ │ └── sub
│ │ │ ├── c.txt
│ │ │ └── keep-also.txt
│ │ ├── max-lines
│ │ ├── a.txt
│ │ ├── b.txt
│ │ └── sub
│ │ │ └── c.txt
│ │ ├── no-content
│ │ ├── a.txt
│ │ ├── b.txt
│ │ └── sub
│ │ │ └── c.txt
│ │ └── no-tree
│ │ ├── a.txt
│ │ ├── b.txt
│ │ └── sub
│ │ └── c.txt
├── fileview
│ ├── fileview.go
│ └── fileview_test.go
├── gitutil
│ └── gitutil.go
└── treeview
│ └── treeview.go
└── scripts
├── build.sh
├── empty-release-commit.sh
├── mk-testdata.sh
├── package.sh
└── release.sh

## /.github/workflows/ci.yml:

1 | name: CI
2 |
3 | on:
4 | push:
5 | pull_request:
6 |
7 | jobs:
8 | test:
9 | name: Run Tests
10 | runs-on: ubuntu-latest
11 |
12 | steps:
13 | - name: Checkout code
14 | uses: actions/checkout@v4
15 |
16 | - name: Set up Go
17 | uses: actions/setup-go@v5
18 | with:
19 | go-version-file: go.mod
20 |
21 | - name: Run tests
22 | run: go test ./...
23 |

---

## /.github/workflows/release.yml:

1 | name: Release
2 |
3 | on:
4 | push:
5 | tags:
6 | - 'v*'
7 |
8 | jobs:
9 | test:
10 | name: Run Tests
11 | runs-on: ubuntu-latest
12 |
13 | steps:
14 | - name: Checkout code
15 | uses: actions/checkout@v4
16 |
17 | - name: Set up Go
18 | uses: actions/setup-go@v5
19 | with:
20 | go-version-file: go.mod
21 |
22 | - name: Run tests
23 | run: go test ./...
24 |
25 | release:
26 | name: Build & Upload Release
27 | runs-on: ubuntu-latest
28 | needs: test
29 |
30 | permissions:
31 | contents: write
32 |
33 | steps:
34 | - name: Checkout code
35 | uses: actions/checkout@v4
36 |
37 | - name: Set up Go
38 | uses: actions/setup-go@v5
39 | with:
40 | go-version-file: go.mod
41 |
42 | - name: Set VERSION from tag
43 | run: echo "VERSION=${GITHUB_REF##*/}" >> $GITHUB_ENV
44 | 
45 |       - name: Build binaries
46 |         run: |
47 |           chmod +x scripts/build.sh
48 |           ./scripts/build.sh uit dist $VERSION
49 | 
50 |       - name: Package binaries
51 |         run: |
52 |           chmod +x scripts/package.sh
53 |           ./scripts/package.sh uit dist
54 | 
55 |       - name: Upload release assets
56 |         uses: softprops/action-gh-release@v2
57 |         with:
58 |           name: "${{ env.VERSION }}"
59 | files: dist/archives/\*.tar.gz
60 |

---

## /Makefile:

1 | APP_NAME := uit
2 | DIST := dist
3 |
4 | all: build
5 |
6 | clean:
7 | @rm -rf $(DIST)
8 |
9 | build:
10 | @chmod +x scripts/build.sh
11 | @scripts/build.sh $(APP_NAME) $(DIST)
12 |
13 | package: build
14 | @chmod +x scripts/package.sh
15 | @scripts/package.sh $(APP_NAME) $(DIST)
16 |
17 |

---

## /README.md:

1 | # uit
2 |
3 | Render directory tree and file contents from a Git repository.
4 |
5 | ## Usage
6 |
7 | `sh
 8 | uit [options] [path]
 9 | `
10 |
11 | Use `uit --help` to see available options.
12 |
13 | ## Installation
14 |
15 | Download a prebuilt binary from the [Releases](https://github.com/mnishiguchi/uit/releases) page.
16 |
17 | Or, build from source if you have Go installed:
18 |
19 | `sh
20 | go build -o uit ./cmd/uit
21 | `
22 |
23 | You can now run it locally:
24 |
25 | `sh
26 | ./uit --help
27 | `
28 |
29 | ## License
30 |
31 | MIT
32 |

---

## /RELEASE.md:

1 | # Release Process
2 |
3 | This guide describes how to publish a new version of `uit`.
4 |
5 | ---
6 |
7 | ## 1. Prepare a Release Pull Request
8 |
9 | Create a PR titled like:
10 |
11 | `12 | vYYYY.MM.DD release
13 |`
14 |
15 | Include:
16 |
17 | - Updates to `README.md` if needed
18 | - A summary of changes (changelog) in the PR description or commit
19 | - Any final cleanups before release
20 |
21 | Once the PR is approved and merged to `main`, proceed to the next step.
22 |
23 | You can optionally create a placeholder release commit (without changes) using:
24 |
25 | `sh
26 | ./scripts/empty-release-commit.sh vYYYY.MM.DD
27 | `
28 |
29 | This is useful when you want to reserve a version number or clearly mark the release point.
30 |
31 | ---
32 |
33 | ## 2. Tag the Release
34 |
35 | After merging to `main`, create a version tag:
36 |
37 | `sh
38 | git checkout main
39 | git pull origin main
40 | git tag v2025.03.28
41 | git push origin v2025.03.28
42 | `
43 |
44 | This triggers GitHub Actions to:
45 |
46 | - Build binaries for multiple platforms
47 | - Package them into `.tar.gz` archives (includes `README.md`)
48 | - Upload the archives to a new GitHub Release
49 |
50 | Alternatively, the same steps can be done with:
51 |
52 | `sh
53 | ./scripts/release.sh vYYYY.MM.DD
54 | `
55 |
56 | This script will:
57 |
58 | - Ensure you’re on the `main` branch with a clean working directory
59 | - Show the changelog since the last tag
60 | - Create and push the tag
61 | - Trigger the GitHub release workflow
62 |
63 | ---
64 |
65 | ## 3. Verify
66 |
67 | After the GitHub Actions workflow completes, confirm:
68 |
69 | - The release appears under [Releases](https://github.com/mnishiguchi/uit/releases)
70 | - Each `.tar.gz` archive includes:
71 | - The `uit` binary
72 | - `README.md`
73 |
74 | ---
75 |
76 | ## 4. Unrelease (if needed)
77 |
78 | To delete a mistakenly pushed tag:
79 |
80 | `sh
81 | git tag -d v2025.03.28-X
82 | git push origin :refs/tags/v2025.03.28-X
83 | `
84 |
85 | Then delete the corresponding release on GitHub manually.
86 |
87 | ---
88 |
89 | Happy releasing!
90 |

---

## /cmd/uit/main.go:

1 | package main
2 |
3 | import (
4 | "fmt"
5 | "os"
6 |
7 | "github.com/mnishiguchi/uit/internal/cli"
8 | )
9 |
10 | var version = "v-dev"
11 |
12 | func main() {
13 | if len(os.Args) == 1 {
14 | os.Args = append(os.Args, "--help")
15 | }
16 |
17 | app := cli.NewApp(version)
18 |
19 | if err := app.Run(os.Args); err != nil {
20 | fmt.Fprintf(os.Stderr, "Error: %v\n", err)
21 | os.Exit(1)
22 | }
23 | }
24 |

---

## /cmd/uit/main_test.go:

1 | package main
2 |

---

## /go.mod:

1 | module github.com/mnishiguchi/uit
2 |
3 | go 1.23.5
4 |
5 | require (
6 | github.com/stretchr/testify v1.10.0
7 | github.com/urfave/cli/v2 v2.27.6
8 | )
9 |
10 | require (
11 | github.com/atotto/clipboard v0.1.4 // indirect
12 | github.com/cpuguy83/go-md2man/v2 v2.0.5 // indirect
13 | github.com/davecgh/go-spew v1.1.1 // indirect
14 | github.com/pmezard/go-difflib v1.0.0 // indirect
15 | github.com/russross/blackfriday/v2 v2.1.0 // indirect
16 | github.com/xrash/smetrics v0.0.0-20240521201337-686a1a2994c1 // indirect
17 | gopkg.in/yaml.v3 v3.0.1 // indirect
18 | )
19 |

---

## /go.sum:

1 | github.com/atotto/clipboard v0.1.4 h1:EH0zSVneZPSuFR11BlR9YppQTVDbh5+16AmcJi4g1z4=
2 | github.com/atotto/clipboard v0.1.4/go.mod h1:ZY9tmq7sm5xIbd9bOK4onWV4S6X0u6GY7Vn0Yu86PYI=
3 | github.com/cpuguy83/go-md2man/v2 v2.0.5 h1:ZtcqGrnekaHpVLArFSe4HK5DoKx1T0rq2DwVB0alcyc=
4 | github.com/cpuguy83/go-md2man/v2 v2.0.5/go.mod h1:tgQtvFlXSQOSOSIRvRPT7W67SCa46tRHOmNcaadrF8o=
5 | github.com/davecgh/go-spew v1.1.1 h1:vj9j/u1bqnvCEfJOwUhtlOARqs3+rkHYY13jYWTU97c=
6 | github.com/davecgh/go-spew v1.1.1/go.mod h1:J7Y8YcW2NihsgmVo/mv3lAwl/skON4iLHjSsI+c5H38=
7 | github.com/pmezard/go-difflib v1.0.0 h1:4DBwDE0NGyQoBHbLQYPwSUPoCMWR5BEzIk/f1lZbAQM=
8 | github.com/pmezard/go-difflib v1.0.0/go.mod h1:iKH77koFhYxTK1pcRnkKkqfTogsbg7gZNVY4sRDYZ/4=
9 | github.com/russross/blackfriday/v2 v2.1.0 h1:JIOH55/0cWyOuilr9/qlrm0BSXldqnqwMsf35Ld67mk=
10 | github.com/russross/blackfriday/v2 v2.1.0/go.mod h1:+Rmxgy9KzJVeS9/2gXHxylqXiyQDYRxCVz55jmeOWTM=
11 | github.com/stretchr/testify v1.10.0 h1:Xv5erBjTwe/5IxqUQTdXv5kgmIvbHo3QQyRwhJsOfJA=
12 | github.com/stretchr/testify v1.10.0/go.mod h1:r2ic/lqez/lEtzL7wO/rwa5dbSLXVDPFyf8C91i36aY=
13 | github.com/urfave/cli/v2 v2.27.6 h1:VdRdS98FNhKZ8/Az8B7MTyGQmpIr36O1EHybx/LaZ4g=
14 | github.com/urfave/cli/v2 v2.27.6/go.mod h1:3Sevf16NykTbInEnD0yKkjDAeZDS0A6bzhBH5hrMvTQ=
15 | github.com/xrash/smetrics v0.0.0-20240521201337-686a1a2994c1 h1:gEOO8jv9F4OT7lGCjxCBTO/36wtF6j2nSip77qHd4x4=
16 | github.com/xrash/smetrics v0.0.0-20240521201337-686a1a2994c1/go.mod h1:Ohn+xnUBiLI6FVj/9LpzZWtj1/D6lUovWYBkxHVV3aM=
17 | gopkg.in/check.v1 v0.0.0-20161208181325-20d25e280405 h1:yhCVgyC4o1eVCa2tZl7eS0r+SDo693bJlVdllGtEeKM=
18 | gopkg.in/check.v1 v0.0.0-20161208181325-20d25e280405/go.mod h1:Co6ibVJAznAaIkqp8huTwlJQCZ016jof/cbN4VW5Yz0=
19 | gopkg.in/yaml.v3 v3.0.1 h1:fxVm/GzAzEWqLHuvctI91KS9hhNmmWOoWu0XTYJS7CA=
20 | gopkg.in/yaml.v3 v3.0.1/go.mod h1:K4uyk7z7BCEPqu6E+C64Yfv1cQ7kz7rIZviUmN+EgEM=
21 |

---

## /internal/cli/cli.go:

1 | package cli
2 |
3 | import (
4 | "bytes"
5 | "fmt"
6 | "io"
7 | "os"
8 | "os/exec"
9 | "regexp"
10 | "strings"
11 |
12 | "github.com/atotto/clipboard"
13 | "github.com/urfave/cli/v2"
14 |
15 | "github.com/mnishiguchi/uit/internal/fileview"
16 | "github.com/mnishiguchi/uit/internal/gitutil"
17 | "github.com/mnishiguchi/uit/internal/treeview"
18 | )
19 |
20 | // NewApp returns a CLI app instance for uit.
21 | func NewApp(version string) *cli.App {
22 | return &cli.App{
23 | Name: "uit",
24 | Usage: "Render directory tree and file contents from a Git repo",
25 | UsageText: "uit [options] [path]",
26 | Version: version,
27 | Authors: []*cli.Author{
28 | {
29 | Name: "Masatoshi Nishiguchi",
30 | },
31 | },
32 | Flags: []cli.Flag{
33 | &cli.IntFlag{
34 | Name: "max-lines",
35 | Usage: "limit the number of lines printed per file",
36 | Value: 500,
37 | },
38 | &cli.BoolFlag{
39 | Name: "no-tree",
40 | Usage: "do not render the tree view",
41 | },
42 | &cli.BoolFlag{
43 | Name: "no-content",
44 | Usage: "do not render file contents",
45 | },
46 | &cli.BoolFlag{
47 | Name: "copy",
48 | Usage: "copy output to clipboard",
49 | },
50 | &cli.BoolFlag{
51 | Name: "fzf",
52 | Usage: "interactively select files via fzf (if installed)",
53 | },
54 | &cli.StringFlag{
55 | Name: "filter",
56 | Usage: "filter file paths with a regular expression",
57 | },
58 | },
59 | Action: func(c \*cli.Context) error {
60 | inputPath := "."
61 | if c.Args().Len() > 0 {
62 | inputPath = c.Args().First()
63 | }
64 |
65 | return Execute(
66 | inputPath,
67 | c.Int("max-lines"),
68 | c.Bool("no-tree"),
69 | c.Bool("no-content"),
70 | c.Bool("copy"),
71 | c.Bool("fzf"),
72 | c.String("filter"),
73 | c.App.Writer,
74 | )
75 | },
76 | }
77 | }
78 |
79 | func Execute(
80 | inputPath string,
81 | maxLines int,
82 | noTree bool,
83 | noContent bool,
84 | copyToClipboard bool,
85 | useFZF bool,
86 | filterPattern string,
87 | writer io.Writer,
88 | ) error {
89 | var clipboardBuf bytes.Buffer
90 | out := io.MultiWriter(writer, &clipboardBuf)
91 |
92 | if !noTree {
93 | if err := treeview.TreeViewFromGit(inputPath, out); err == nil {
94 | fmt.Fprintln(out)
95 | fmt.Fprintln(out)
96 | }
97 | }
98 |
99 | if noContent {
100 | return finalizeOutput(clipboardBuf, copyToClipboard)
101 | }
102 |
103 | if err := renderFiles(inputPath, maxLines, useFZF, filterPattern, out); err != nil {
104 | return err
105 | }
106 |
107 | return finalizeOutput(clipboardBuf, copyToClipboard)
108 | }
109 |
110 | func renderFiles(path string, maxLines int, useFZF bool, filter string, out io.Writer) error {
111 | info, err := os.Stat(path)
112 | if err != nil {
113 | return fmt.Errorf("invalid path: %w", err)
114 | }
115 |
116 | if !info.IsDir() {
117 | if err := fileview.FileViewWithLines(path, out, maxLines); err != nil {
118 | return fmt.Errorf("failed to render file %s: %w", path, err)
119 | }
120 | return nil
121 | }
122 |
123 | files, err := listGitFiles(path)
124 | if err != nil {
125 | return err
126 | }
127 |
128 | files = filterFiles(files, filter)
129 |
130 | if useFZF && isFZFInstalled() {
131 | files, err = selectFilesWithFZF(files)
132 | if err != nil {
133 | return err
134 | }
135 | }
136 |
137 | for _, f := range files {
138 | if err := fileview.FileViewWithLines(f, out, maxLines); err != nil {
139 | return fmt.Errorf("failed to render file %s: %w", f, err)
140 | }
141 | }
142 |
143 | return nil
144 | }
145 |
146 | func listGitFiles(path string) ([]string, error) {
147 | files, err := gitutil.ListGitTrackedFiles(path)
148 | if err != nil {
149 | if strings.Contains(err.Error(), "not a Git repository") {
150 | return nil, fmt.Errorf("this directory is not inside a Git repository: %s", path)
151 | }
152 |
153 | return nil, fmt.Errorf("failed to list files: %w", err)
154 | }
155 |
156 | if len(files) == 0 {
157 | return nil, fmt.Errorf("no Git-tracked files found in: %s", path)
158 | }
159 |
160 | return files, nil
161 | }
162 |
163 | func filterFiles(files []string, pattern string) []string {
164 | if pattern == "" {
165 | return files
166 | }
167 |
168 | re, err := regexp.Compile(pattern)
169 | if err != nil {
170 | return []string{} // Or log invalid regex warning?
171 | }
172 |
173 | var filtered []string
174 | for _, f := range files {
175 | if re.MatchString(f) {
176 | filtered = append(filtered, f)
177 | }
178 | }
179 |
180 | return filtered
181 | }
182 |
183 | func finalizeOutput(buf bytes.Buffer, enabled bool) error {
184 | if enabled {
185 | if err := clipboard.WriteAll(buf.String()); err != nil {
186 | return fmt.Errorf("failed to copy to clipboard: %w", err)
187 | }
188 |
189 | fmt.Fprintln(os.Stderr, "✔️ Copied to clipboard.")
190 | }
191 |
192 | return nil
193 | }
194 |
195 | func isFZFInstalled() bool {
196 | \_, err := exec.LookPath("fzf")
197 | return err == nil
198 | }
199 |
200 | func selectFilesWithFZF(files []string) ([]string, error) {
201 | cmd := exec.Command("fzf", "--multi")
202 | cmd.Stdin = strings.NewReader(strings.Join(files, "\n"))
203 |
204 | out, err := cmd.Output()
205 | if err != nil {
206 | return nil, fmt.Errorf("fzf failed: %w", err)
207 | }
208 |
209 | selection := strings.Split(strings.TrimSpace(string(out)), "\n")
210 |
211 | return selection, nil
212 | }
213 |

---

## /internal/cli/cli_test.go:

1 | package cli_test
2 |
3 | import (
4 | "bytes"
5 | "flag"
6 | "io"
7 | "os"
8 | "path/filepath"
9 | "testing"
10 |
11 | "github.com/atotto/clipboard"
12 | "github.com/stretchr/testify/assert"
13 | "github.com/stretchr/testify/require"
14 |
15 | "github.com/mnishiguchi/uit/internal/cli"
16 | )
17 |
18 | var updateGolden = flag.Bool("update", false, "update golden files")
19 |
20 | func TestRun(t *testing.T) {
21 | cases := map[string]struct {
22 | maxLines int
23 | noTree bool
24 | noContent bool
25 | copyToClip bool
26 | useFZF bool
27 | filter string
28 | }{
29 | "default": {
30 | maxLines: 500,
31 | },
32 | "max-lines": {
33 | maxLines: 3,
34 | },
35 | "no-tree": {
36 | noTree: true,
37 | },
38 | "no-content": {
39 | noContent: true,
40 | },
41 | "filter": {
42 | filter: "a\\.txt
quot;,
43 | },
44 | "copy": {
45 | copyToClip: true,
46 | },
47 | "binary": {
48 | maxLines: 500,
49 | },
50 | }
51 |
52 | for label, tt := range cases {
53 | // Skip if clipboard isn't supported
54 | if err := clipboard.WriteAll("test"); err != nil {
55 | t.Skip("Skipping clipboard test: clipboard not available")
56 | }
57 |
58 | t.Run(label, func(t *testing.T) {
59 | inputDir := filepath.Join("testdata", "input", label)
60 | goldenFile := filepath.Join("testdata", "golden", label)
61 |
62 | var buf bytes.Buffer
63 | err := cli.Execute(
64 | inputDir,
65 | tt.maxLines,
66 | tt.noTree,
67 | tt.noContent,
68 | tt.copyToClip,
69 | tt.useFZF,
70 | tt.filter,
71 | &buf,
72 | )
73 | require.NoError(t, err)
74 |
75 | actual := buf.String()
76 |
77 | if *updateGolden {
78 | err := os.WriteFile(goldenFile, []byte(actual), 0644)
79 | require.NoError(t, err)
80 | }
81 |
82 | expected, err := os.ReadFile(goldenFile)
83 | require.NoError(t, err)
84 |
85 | assert.Equal(t, string(expected), actual)
86 | })
87 | }
88 | }
89 |
90 | func TestCopyConfirmationMessage(t *testing.T) {
91 | // Skip if clipboard isn't supported
92 | if err := clipboard.WriteAll("test"); err != nil {
93 | t.Skip("Skipping clipboard test: clipboard not available")
94 | }
95 |
96 | inputDir := filepath.Join("testdata", "input", "copy")
97 | var stdoutBuf bytes.Buffer
98 |
99 | // Capture stderr
100 | stderrReader, stderrWriter, err := os.Pipe()
101 | require.NoError(t, err)
102 | originalStderr := os.Stderr
103 | os.Stderr = stderrWriter
104 | defer func() {
105 | os.Stderr = originalStderr
106 | stderrWriter.Close()
107 | }()
108 |
109 | done := make(chan string)
110 | go func() {
111 | var buf bytes.Buffer
112 | io.Copy(&buf, stderrReader)
113 | done <- buf.String()
114 | }()
115 |
116 | err = cli.Execute(
117 | inputDir,
118 | 500, // maxLines
119 | false, // noTree
120 | false, // noContent
121 | true, // copyToClip
122 | false, // useFZF
123 | "", // filter
124 | &stdoutBuf,
125 | )
126 | require.NoError(t, err)
127 |
128 | stderrWriter.Close()
129 | stderrOutput := <-done
130 |
131 | assert.Contains(t, stderrOutput, "Copied to clipboard")
132 | }
133 |

---

## /internal/cli/testdata/README.md:

1 | # internal/cli/testdata
2 |
3 | This directory contains test input and expected output files used for verifying
4 | the `uit` CLI tool via golden file tests.
5 |
6 | ## Structure
7 |
8 | ` 9 | testdata/
10 | ├── input/       # Input files organized by scenario
11 | └── golden/      # Expected output for each corresponding scenario
12 |`
13 |
14 | Each subdirectory under `input/` corresponds to a test case. The file with the
15 | same name under `golden/` contains the expected CLI output for that scenario.
16 |
17 | ## Updating Golden Files
18 |
19 | To regenerate golden files based on the current CLI output:
20 |
21 | `bash
22 | go test ./internal/cli -update
23 | `
24 |
25 | This will overwrite files in the `golden/` directory with new output.
26 |
27 | ## Regenerating Input Files
28 |
29 | To rebuild the test input directory structure from scratch:
30 |
31 | `bash
32 | scripts/mk-testdata.sh
33 | `
34 |
35 | This script creates consistent input files for all predefined scenarios under
36 | `input/`.
37 |

---

## /internal/cli/testdata/golden/.keep:

https://raw.githubusercontent.com/mnishiguchi/uit/bc7c3f35862f376763306203b940037009310a4d/internal/cli/testdata/golden/.keep

---

## /internal/cli/testdata/golden/binary:

1 | binary
2 | ├── a.txt
3 | ├── b.txt
4 | └── sub
5 | └── c.txt
6 |
7 |
8 | /testdata/input/binary/a.txt:
9 | --------------------------------------------------------------------------------
10 | [binary file omitted]
11 |
12 |
13 | --------------------------------------------------------------------------------
14 | /testdata/input/binary/b.txt:
15 | --------------------------------------------------------------------------------
16 | 1 | line 1
17 | 2 | line 2
18 | 3 | line 3
19 | 4 | line 4
20 | 5 | line 5
21 |
22 |
23 | --------------------------------------------------------------------------------
24 | /testdata/input/binary/sub/c.txt:
25 | --------------------------------------------------------------------------------
26 | 1 | line 1
27 | 2 | line 2
28 | 3 | line 3
29 | 4 | line 4
30 | 5 | line 5
31 |
32 |
33 | --------------------------------------------------------------------------------
34 |

---

## /internal/cli/testdata/golden/copy:

1 | copy
2 | ├── a.txt
3 | ├── b.txt
4 | └── sub
5 | └── c.txt
6 |
7 |
8 | /testdata/input/copy/a.txt:
9 | --------------------------------------------------------------------------------
10 | 1 | line 1
11 | 2 | line 2
12 | 3 | line 3
13 | 4 | line 4
14 | 5 | line 5
15 |
16 |
17 | --------------------------------------------------------------------------------
18 | /testdata/input/copy/b.txt:
19 | --------------------------------------------------------------------------------
20 | 1 | line 1
21 | 2 | line 2
22 | 3 | line 3
23 | 4 | line 4
24 | 5 | line 5
25 |
26 |
27 | --------------------------------------------------------------------------------
28 | /testdata/input/copy/sub/c.txt:
29 | --------------------------------------------------------------------------------
30 | 1 | line 1
31 | 2 | line 2
32 | 3 | line 3
33 | 4 | line 4
34 | 5 | line 5
35 |
36 |
37 | --------------------------------------------------------------------------------
38 |

---

## /internal/cli/testdata/golden/default:

1 | default
2 | ├── a.txt
3 | ├── b.txt
4 | └── sub
5 | └── c.txt
6 |
7 |
8 | /testdata/input/default/a.txt:
9 | --------------------------------------------------------------------------------
10 | 1 | line 1
11 | 2 | line 2
12 | 3 | line 3
13 | 4 | line 4
14 | 5 | line 5
15 |
16 |
17 | --------------------------------------------------------------------------------
18 | /testdata/input/default/b.txt:
19 | --------------------------------------------------------------------------------
20 | 1 | line 1
21 | 2 | line 2
22 | 3 | line 3
23 | 4 | line 4
24 | 5 | line 5
25 |
26 |
27 | --------------------------------------------------------------------------------
28 | /testdata/input/default/sub/c.txt:
29 | --------------------------------------------------------------------------------
30 | 1 | line 1
31 | 2 | line 2
32 | 3 | line 3
33 | 4 | line 4
34 | 5 | line 5
35 |
36 |
37 | --------------------------------------------------------------------------------
38 |

---

## /internal/cli/testdata/golden/filter:

1 | filter
2 | ├── a.txt
3 | ├── b.txt
4 | ├── keep-me.txt
5 | ├── skip-me.txt
6 | └── sub
7 | ├── c.txt
8 | └── keep-also.txt
9 |
10 |
11 | /testdata/input/filter/a.txt:
12 | --------------------------------------------------------------------------------
13 | 1 | line 1
14 | 2 | line 2
15 | 3 | line 3
16 | 4 | line 4
17 | 5 | line 5
18 |
19 |
20 | --------------------------------------------------------------------------------
21 |

---

## /internal/cli/testdata/golden/max-lines:

1 | max-lines
2 | ├── a.txt
3 | ├── b.txt
4 | └── sub
5 | └── c.txt
6 |
7 |
8 | /testdata/input/max-lines/a.txt:
9 | --------------------------------------------------------------------------------
10 | 1 | line 1
11 | 2 | line 2
12 | 3 | line 3
13 |
14 |
15 | --------------------------------------------------------------------------------
16 | /testdata/input/max-lines/b.txt:
17 | --------------------------------------------------------------------------------
18 | 1 | line 1
19 | 2 | line 2
20 | 3 | line 3
21 |
22 |
23 | --------------------------------------------------------------------------------
24 | /testdata/input/max-lines/sub/c.txt:
25 | --------------------------------------------------------------------------------
26 | 1 | line 1
27 | 2 | line 2
28 | 3 | line 3
29 |
30 |
31 | --------------------------------------------------------------------------------
32 |

---

## /internal/cli/testdata/golden/no-content:

1 | no-content
2 | ├── a.txt
3 | ├── b.txt
4 | └── sub
5 | └── c.txt
6 |
7 |
8 |

---

## /internal/cli/testdata/golden/no-tree:

1 | /testdata/input/no-tree/a.txt:
2 | --------------------------------------------------------------------------------
3 | 1 | line 1
4 | 2 | line 2
5 | 3 | line 3
6 | 4 | line 4
7 | 5 | line 5
8 |
9 |
10 | --------------------------------------------------------------------------------
11 | /testdata/input/no-tree/b.txt:
12 | --------------------------------------------------------------------------------
13 | 1 | line 1
14 | 2 | line 2
15 | 3 | line 3
16 | 4 | line 4
17 | 5 | line 5
18 |
19 |
20 | --------------------------------------------------------------------------------
21 | /testdata/input/no-tree/sub/c.txt:
22 | --------------------------------------------------------------------------------
23 | 1 | line 1
24 | 2 | line 2
25 | 3 | line 3
26 | 4 | line 4
27 | 5 | line 5
28 |
29 |
30 | --------------------------------------------------------------------------------
31 |

---

## /internal/cli/testdata/input/.keep:

https://raw.githubusercontent.com/mnishiguchi/uit/bc7c3f35862f376763306203b940037009310a4d/internal/cli/testdata/input/.keep

---

## /internal/cli/testdata/input/binary/a.txt:

1 | �This is binary data

---

## /internal/cli/testdata/input/binary/b.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/binary/sub/c.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/copy/a.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/copy/b.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/copy/sub/c.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/default/a.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/default/b.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/default/sub/c.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/filter/a.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/filter/b.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/filter/keep-me.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/filter/skip-me.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/filter/sub/c.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/filter/sub/keep-also.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/max-lines/a.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/max-lines/b.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/max-lines/sub/c.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/no-content/a.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/no-content/b.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/no-content/sub/c.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/no-tree/a.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/no-tree/b.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/cli/testdata/input/no-tree/sub/c.txt:

1 | line 1
2 | line 2
3 | line 3
4 | line 4
5 | line 5
6 |

---

## /internal/fileview/fileview.go:

1 | package fileview
2 |
3 | import (
4 | "bufio"
5 | "bytes"
6 | "fmt"
7 | "io"
8 | "os"
9 | "path/filepath"
10 | "strings"
11 |
12 | "github.com/mnishiguchi/uit/internal/gitutil"
13 | )
14 |
15 | // FileViewWithLines prints the content of a single file to the writer with line numbers.
16 | func FileViewWithLines(path string, w io.Writer, maxLines int) error {
17 | absPath, err := filepath.Abs(path)
18 | if err != nil {
19 | return fmt.Errorf("failed to resolve absolute path: %w", err)
20 | }
21 |
22 | info, err := os.Stat(absPath)
23 | if err != nil {
24 | return fmt.Errorf("failed to stat path: %w", err)
25 | }
26 | if info.IsDir() {
27 | return fmt.Errorf("cannot render directory as file: %s", absPath)
28 | }
29 |
30 | if isBin, err := isBinaryFile(absPath); err == nil && isBin {
31 | printFileHeader(w, path)
32 | fmt.Fprintln(w, "[binary file omitted]")
33 | printFileFooter(w)
34 | return nil
35 | }
36 |
37 | file, err := os.Open(absPath)
38 | if err != nil {
39 | return fmt.Errorf("failed to open file: %w", err)
40 | }
41 | defer file.Close()
42 |
43 | printFileHeader(w, path)
44 |
45 | if err := printFileBodyWithLines(file, w, maxLines); err != nil {
46 | return err
47 | }
48 |
49 | printFileFooter(w)
50 |
51 | return nil
52 | }
53 |
54 | func printFileHeader(w io.Writer, path string) {
55 | // Try to get path relative to Git root if available
56 | gitRoot, err := gitutil.GetGitRoot(path)
57 | if err != nil {
58 | relPath, relErr := filepath.Rel(".", path)
59 | if relErr != nil {
60 | relPath = path
61 | }
62 |
63 | fmt.Fprintf(w, "/%s:\n", filepath.ToSlash(relPath))
64 | } else {
65 | relToGitRoot, err := filepath.Rel(gitRoot, path)
66 | if err != nil {
67 | relToGitRoot = path
68 | }
69 |
70 | fmt.Fprintf(w, "\n\n/%s:\n", filepath.ToSlash(relToGitRoot))
71 | }
72 |
73 | fmt.Fprintln(w, strings.Repeat("-", 80))
74 | }
75 |
76 | func printFileBodyWithLines(file \*os.File, w io.Writer, maxLines int) error {
77 | scanner := bufio.NewScanner(file)
78 | lineNum := 1
79 |
80 | for scanner.Scan() {
81 | if maxLines > 0 && lineNum > maxLines {
82 | break
83 | }
84 | fmt.Fprintf(w, "%4d | %s\n", lineNum, scanner.Text())
85 | lineNum++
86 | }
87 |
88 | if err := scanner.Err(); err != nil {
89 | return fmt.Errorf("error reading file: %w", err)
90 | }
91 |
92 | return nil
93 | }
94 |
95 | func printFileFooter(w io.Writer) {
96 | fmt.Fprintln(w, "\n\n"+strings.Repeat("-", 80))
97 | }
98 |
99 | // isBinaryFile returns true if the file contains a null byte in the first 8000 bytes.
100 | func isBinaryFile(path string) (bool, error) {
101 | file, err := os.Open(path)
102 | if err != nil {
103 | return false, err
104 | }
105 | defer file.Close()
106 |
107 | const maxBytes = 8000
108 | buf := make([]byte, maxBytes)
109 |
110 | n, err := file.Read(buf)
111 | if err != nil && err != io.EOF {
112 | return false, err
113 | }
114 |
115 | return bytes.IndexByte(buf[:n], 0) >= 0, nil
116 | }
117 |

---

## /internal/fileview/fileview_test.go:

1 | package fileview*test
2 |
3 | import (
4 | "bytes"
5 | "os"
6 | "path/filepath"
7 | "testing"
8 |
9 | "github.com/stretchr/testify/assert"
10 |
11 | "github.com/mnishiguchi/uit/internal/fileview"
12 | "github.com/mnishiguchi/uit/internal/gitutil"
13 | "github.com/mnishiguchi/uit/internal/treeview"
14 | )
15 |
16 | func TestRenderGitTree(t *testing.T) {
17 | t.Run("prints tree including known file", func(t *testing.T) {
18 | var buf bytes.Buffer
19 |
20 | cwd, err := os.Getwd()
21 | assert.NoError(t, err)
22 |
23 | err = treeview.TreeViewFromGit(cwd, &buf)
24 | assert.NoError(t, err)
25 |
26 | output := buf.String()
27 |
28 | t.Run("includes root directory name", func(t *testing.T) {
29 | expectedRoot := filepath.Base(cwd)
30 | assert.Contains(t, output, expectedRoot)
31 | })
32 |
33 | t.Run("includes known file", func(t *testing.T) {
34 | assert.Contains(t, output, "fileview.go", "expected tree output to contain a known file")
35 | })
36 | })
37 | }
38 |
39 | func TestRenderFileContent(t *testing.T) {
40 | t.Run("renders file with line limit", func(t *testing.T) {
41 | tmpDir := t.TempDir()
42 | textFile := filepath.Join(tmpDir, "sample-head.txt")
43 | content := `line 1
 44 | line 2
 45 | line 3
 46 | line 4
 47 | line 5`
48 | err := os.WriteFile(textFile, []byte(content), 0644)
49 | assert.NoError(t, err)
50 |
51 | var buf bytes.Buffer
52 | err = fileview.FileViewWithLines(textFile, &buf, 3)
53 | assert.NoError(t, err)
54 |
55 | output := buf.String()
56 | assert.Contains(t, output, " 1 | line 1")
57 | assert.Contains(t, output, " 2 | line 2")
58 | assert.Contains(t, output, " 3 | line 3")
59 | assert.NotContains(t, output, "line 4")
60 | assert.NotContains(t, output, "line 5")
61 | })
62 |
63 | t.Run("renders full file when no limit", func(t *testing.T) {
64 | tmpDir := t.TempDir()
65 | textFile := filepath.Join(tmpDir, "sample-full.txt")
66 | content := `line A
 67 | line B
 68 | line C`
69 | err := os.WriteFile(textFile, []byte(content), 0644)
70 | assert.NoError(t, err)
71 |
72 | var buf bytes.Buffer
73 | err = fileview.FileViewWithLines(textFile, &buf, 0)
74 | assert.NoError(t, err)
75 |
76 | output := buf.String()
77 | assert.Contains(t, output, " 1 | line A")
78 | assert.Contains(t, output, " 2 | line B")
79 | assert.Contains(t, output, " 3 | line C")
80 | })
81 | }
82 |
83 | func TestFindGitRoot(t _testing.T) {
84 | t.Run("returns error for non-Git directory", func(t \*testing.T) {
85 | tmp := t.TempDir()
86 | _, err := gitutil.GetGitRoot(tmp)
87 | assert.Error(t, err)
88 | })
89 | }
90 |
91 | func TestListGitFilesUnder(t *testing.T) {
92 | t.Run("returns error for non-Git directory", func(t *testing.T) {
93 | tmp := t.TempDir()
94 | \_, err := gitutil.ListGitTrackedFiles(tmp)
95 | assert.Error(t, err)
96 | })
97 | }
98 |
99 | func TestRenderFileContent_RejectsDirectory(t *testing.T) {
100 | t.Run("returns error when given a directory", func(t *testing.T) {
101 | dir := t.TempDir()
102 | var buf bytes.Buffer
103 |
104 | err := fileview.FileViewWithLines(dir, &buf, 0)
105 | assert.ErrorContains(t, err, "cannot render directory as file")
106 | })
107 | }
108 |

---

## /internal/gitutil/gitutil.go:

1 | package gitutil
2 |
3 | import (
4 | "fmt"
5 | "os/exec"
6 | "path/filepath"
7 | "strings"
8 | )
9 |
10 | // GetGitRoot returns the absolute path of the Git repository root for the given path.
11 | func GetGitRoot(path string) (string, error) {
12 | cmd := exec.Command("git", "-C", path, "rev-parse", "--show-toplevel")
13 | output, err := cmd.Output()
14 | if err != nil {
15 | return "", fmt.Errorf("not a Git repository: %s", path)
16 | }
17 |
18 | return strings.TrimSpace(string(output)), nil
19 | }
20 |
21 | // ListGitTrackedFiles returns a list of Git-tracked files under a given directory.
22 | func ListGitTrackedFiles(dir string) ([]string, error) {
23 | cmd := exec.Command("git", "-C", dir, "ls-files")
24 | output, err := cmd.Output()
25 | if err != nil {
26 | return nil, err
27 | }
28 |
29 | lines := strings.Split(strings.TrimSpace(string(output)), "\n")
30 | files := make([]string, 0, len(lines))
31 | for \_, line := range lines {
32 | if strings.TrimSpace(line) == "" {
33 | continue // Skip blank lines to avoid treating directory as file
34 | }
35 | files = append(files, filepath.Join(dir, line))
36 | }
37 | return files, nil
38 | }
39 |

---

## /internal/treeview/treeview.go:

1 | package treeview
2 |
3 | import (
4 | "fmt"
5 | "io"
6 | "os/exec"
7 | "path/filepath"
8 | "sort"
9 | "strings"
10 |
11 | "github.com/mnishiguchi/uit/internal/gitutil"
12 | )
13 |
14 | // TreeNode represents a node in the directory tree.
15 | type TreeNode struct {
16 | Name string
17 | IsFile bool
18 | Children map[string]*TreeNode
19 | }
20 |
21 | // TreeViewFromGit builds and prints a Git-tracked file tree starting from the user-specified path.
22 | func TreeViewFromGit(inputPath string, w io.Writer) error {
23 | tree, err := buildTreeFromGit(inputPath)
24 | if err != nil {
25 | return err
26 | }
27 |
28 | printTreeRoot(tree, w)
29 |
30 | return nil
31 | }
32 |
33 | func buildTreeFromGit(inputPath string) (*TreeNode, error) {
34 | absInput, err := filepath.Abs(inputPath)
35 | if err != nil {
36 | return nil, fmt.Errorf("failed to resolve input path: %w", err)
37 | }
38 |
39 | gitRoot, err := gitutil.GetGitRoot(absInput)
40 | if err != nil {
41 | return nil, fmt.Errorf("failed to find git root: %w", err)
42 | }
43 |
44 | cmd := exec.Command("git", "-C", gitRoot, "ls-files")
45 | output, err := cmd.Output()
46 | if err != nil {
47 | return nil, fmt.Errorf("failed to run git ls-files: %w", err)
48 | }
49 | lines := strings.Split(strings.TrimSpace(string(output)), "\n")
50 |
51 | relInputPath, err := filepath.Rel(gitRoot, absInput)
52 | if err != nil {
53 | return nil, fmt.Errorf("failed to get relative input path: %w", err)
54 | }
55 |
56 | var relevantPaths [][]string
57 | for _, line := range lines {
58 | if relInputPath == "." || strings.HasPrefix(line, relInputPath+"/") || line == relInputPath {
59 | trimmed := strings.TrimPrefix(line, relInputPath+"/")
60 | relevantPaths = append(relevantPaths, strings.Split(trimmed, "/"))
61 | }
62 | }
63 |
64 | root := &TreeNode{
65 | Name: filepath.Base(absInput),
66 | IsFile: false,
67 | Children: make(map[string]\*TreeNode),
68 | }
69 |
70 | for _, parts := range relevantPaths {
71 | insertPathParts(root, parts)
72 | }
73 |
74 | return root, nil
75 | }
76 |
77 | // insertPathParts inserts a file path (split into parts) into the tree recursively.
78 | func insertPathParts(node *TreeNode, parts []string) {
79 | if len(parts) == 0 {
80 | return
81 | }
82 |
83 | name := parts[0]
84 | child, exists := node.Children[name]
85 | if !exists {
86 | child = &TreeNode{
87 | Name: name,
88 | IsFile: len(parts) == 1,
89 | Children: make(map[string]*TreeNode),
90 | }
91 | node.Children[name] = child
92 | }
93 |
94 | insertPathParts(child, parts[1:])
95 | }
96 |
97 | // printTreeRoot prints the tree starting from the root node.
98 | func printTreeRoot(node *TreeNode, w io.Writer) {
99 | fmt.Fprintf(w, "%s\n", node.Name)
100 | printTreeChildren(node, "", true, w)
101 | }
102 |
103 | // printTreeChildren prints child nodes of the given tree node recursively.
104 | func printTreeChildren(node *TreeNode, prefix string, isLast bool, w io.Writer) {
105 | \_ = isLast // Reserved for future enhancements
106 |
107 | var keys []string
108 | for k := range node.Children {
109 | keys = append(keys, k)
110 | }
111 | sort.Strings(keys)
112 |
113 | for i, key := range keys {
114 | child := node.Children[key]
115 |
116 | connector := "├──"
117 | nextPrefix := prefix + "│ "
118 | if i == len(keys)-1 {
119 | connector = "└──"
120 | nextPrefix = prefix + " "
121 | }
122 |
123 | fmt.Fprintf(w, "%s%s %s\n", prefix, connector, child.Name)
124 |
125 | if !child.IsFile {
126 | printTreeChildren(child, nextPrefix, i == len(keys)-1, w)
127 | }
128 | }
129 | }
130 |

---

## /scripts/build.sh:

1 | #!/usr/bin/env bash
2 | set -e
3 |
4 | # build.sh — Build binaries for multiple platforms
5 | #
6 | # Usage:
7 | # ./scripts/build.sh <app_name> <output_dir> [version]
8 | #
9 | # Example:
10 | # ./scripts/build.sh uit dist v2025.03.28
11 | #
12 | # Arguments:
13 | # <app_name> The name of the binary (e.g., "uit")
14 | # <output_dir> Output directory (e.g., "dist")
15 | # [version] Optional version string (default: "dev")
16 |
17 | APP_NAME="${1:-uit}"
18 | DIST="${2:-dist}"
19 | VERSION="${3:-dev}"
20 | 
21 | PLATFORMS=(
22 |   "linux amd64"
23 |   "linux arm64"
24 |   "darwin amd64"
25 |   "darwin arm64"
26 | )
27 | 
28 | mkdir -p "$DIST"
29 |
30 | for platform in "${PLATFORMS[@]}"; do
31 |   set -- $platform
32 |   GOOS=$1
33 |   GOARCH=$2
34 |   echo "  -> Building for $GOOS/$GOARCH"
35 | CGO_ENABLED=0 GOOS=$GOOS GOARCH=$GOARCH \
36 | go build -ldflags="-s -w -X main.version=$VERSION" \
37 |     -o "$DIST/$APP_NAME-$GOOS-$GOARCH" ./cmd/$APP_NAME
38 | done
39 |
40 |

---

## /scripts/empty-release-commit.sh:

1 | #!/usr/bin/env bash
2 | set -euo pipefail
3 |
4 | # empty-release-commit.sh — Create an empty commit for a release
5 | #
6 | # Usage:
7 | # ./scripts/empty-release-commit.sh vYYYY.MM.DD
8 | #
9 | # Creates a placeholder commit for marking a release point.
10 |
11 | VERSION="${1:-}"
12 | 
13 | if [[ -z "$VERSION" ]]; then
14 | echo "Usage: $0 vYYYY.MM.DD[-X]"
15 |   exit 1
16 | fi
17 | 
18 | git commit --allow-empty -m "$VERSION release"
19 |
20 |

---

## /scripts/mk-testdata.sh:

1 | #!/usr/bin/env bash
2 | set -euo pipefail
3 |
4 | # mk-testdata.sh — Generate test input directories for uit CLI tests
5 | #
6 | # Usage:
7 | # ./scripts/mk-testdata.sh
8 | #
9 | # This creates test input data under internal/cli/testdata/input/
10 |
11 | ROOT="internal/cli/testdata/input"
12 | CASES=(
13 | "default"
14 | "max-lines"
15 | "no-tree"
16 | "no-content"
17 | "filter"
18 | "copy"
19 | "binary"
20 | )
21 |
22 | write_text_file() {
23 | local file="$1"
24 |   cat <<EOF >"$file"
25 | line 1
26 | line 2
27 | line 3
28 | line 4
29 | line 5
30 | EOF
31 | }
32 |
33 | echo "🔧 Generating test input directories under $ROOT..."
34 | 
35 | for case in "${CASES[@]}"; do
36 | dir="$ROOT/$case"
37 | mkdir -p "$dir"
38 |   echo "📁 Created $dir"
39 | 
40 |   mkdir -p "$dir/sub"
41 |
42 | case "$case" in
43 |   "filter")
44 |     write_text_file "$dir/a.txt"
45 | write_text_file "$dir/b.txt"
46 |     write_text_file "$dir/sub/c.txt"
47 | ;;
48 | "binary")
49 | printf '\x00This is binary data' >"$dir/a.txt"
50 |     write_text_file "$dir/b.txt"
51 | write_text_file "$dir/sub/c.txt"
52 |     ;;
53 |   *)
54 |     write_text_file "$dir/a.txt"
55 | write_text_file "$dir/b.txt"
56 |     write_text_file "$dir/sub/c.txt"
57 | ;;
58 | esac
59 | done
60 |
61 | echo "✅ Done generating test inputs."
62 |

---

## /scripts/package.sh:

1 | #!/usr/bin/env bash
2 | set -euo pipefail
3 |
4 | # package.sh — Package prebuilt binaries into .tar.gz archives
5 | #
6 | # Usage:
7 | # ./scripts/package.sh <app_name> <output_dir>
8 | #
9 | # Example:
10 | # ./scripts/package.sh uit dist
11 | #
12 | # Each archive includes the binary and README.md
13 |
14 | APP_NAME=${1:-uit}
15 | DIST=${2:-dist}
16 | ARCHIVES=$DIST/archives
17 | PLATFORMS=("linux-amd64" "linux-arm64" "darwin-amd64" "darwin-arm64")
18 | 
19 | mkdir -p "$ARCHIVES"
20 |
21 | for platform in "${PLATFORMS[@]}"; do
22 |   binary="$DIST/$APP_NAME-$platform"
23 |
24 | if [[! -f "$binary"]]; then
25 | echo "!! Skipping $platform — binary not found"
26 |     continue
27 |   fi
28 | 
29 |   echo "-> Packaging $platform"
30 |   mkdir -p "$DIST/tmp/$platform"
31 |   cp "$binary" "$DIST/tmp/$platform/$APP_NAME"
32 |   cp README.md "$DIST/tmp/$platform/README.md"
33 |   tar -czf "$ARCHIVES/$APP_NAME-$platform.tar.gz" -C "$DIST/tmp/$platform" "$APP_NAME" README.md
34 |   rm -rf "$DIST/tmp/$platform"
35 | done
36 |
37 |

---

## /scripts/release.sh:

1 | #!/usr/bin/env bash
2 | set -euo pipefail
3 |
4 | # release.sh — Tag and push a version to trigger GitHub Release
5 | #
6 | # Usage:
7 | # ./scripts/release.sh vYYYY.MM.DD
8 | #
9 | # This script:
10 | # - Ensures you’re on main branch
11 | # - Ensures working tree is clean
12 | # - Shows changelog since last tag
13 | # - Tags and pushes the release tag
14 |
15 | VERSION="${1:-}"
16 | 
17 | if [[ -z "$VERSION" ]]; then
18 | echo "Usage: $0 vYYYY.MM.DD[-X]"
19 |   exit 1
20 | fi
21 | 
22 | branch=$(git rev-parse --abbrev-ref HEAD)
23 | if [["$branch" != "main"]]; then
24 | echo "❌ You must be on 'main' branch (currently on '$branch')"
25 |   exit 1
26 | fi
27 | 
28 | if [[ -n "$(git status --porcelain)" ]]; then
29 | echo "❌ Working directory is not clean. Commit or stash changes first."
30 | exit 1
31 | fi
32 |
33 | echo "📋 Changelog since last tag:"
34 | LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")
35 | if [[ -n "$LAST_TAG" ]]; then
36 | git log "$LAST_TAG"..HEAD --pretty=format:"- %s (%h)"
37 | else
38 |   git log --pretty=format:"- %s (%h)"
39 | fi
40 | 
41 | echo
42 | echo "🏷️ Tagging version: $VERSION"
43 | git tag "$VERSION"
44 | git push origin main --tags
45 |
46 | echo
47 | echo "🚀 Release triggered! GitHub Actions will publish binaries."
48 |
49 |

---
