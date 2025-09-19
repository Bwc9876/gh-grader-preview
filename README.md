# GitHub Grader Preview

Simple program that runs test cases specified in an autograding.json file and
reports results. Useful for previewing how GitHub will run your autograder and
project.

> [!WARNING]
>
> While this works on Windows, the GitHub classroom runner uses Linux by
> default, and so some professors may write commands specific to Linux meaning
> they won't work for you.

## Usage

```sh
gh-grader-preview
```

Will auto-find the `autograding.json` file located in `.github/classroom`
relative to the current dir.

You can specify `-f` to choose a different file.

```sh
gh-grader-preview -f some/other/dir/autograding.json
```

To see output from the test cases, use the `-v` flag.

```sh
gh-grader-preview -v
```

For more information, run `gh-grader-preview -h`.

## Building

`cargo build --release`

### Installation

Copy the binary from `target/release/gh-grader-preview` to a directory in your
PATH.

Run `gh-grader-preview --man-page` to print the man page to stdout, and then
save it to a file.

Run `gh-grader-preview --completions=SHELL` (replace `SHELL` with `bash`,
`fish`, or `zsh`) to generate a shell completion script.
