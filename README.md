# GitHub Grader Preview

Simple program that runs test cases specified in an autograding.json file and reports results. Useful for previewing how GitHub will run your autograder and project.

This will only work on Linux as it uses `bash`. Also most GitHub auto-graders runs on the Ubuntu runner image so the commands
are gonna be Linux specific anyway.

## Usage

```sh
gh-grader-preview
```

Will auto-find the `autograding.json` file located in `.github/classroom` relative to the current dir.

You can specify `-f` to choose a different file.

```sh
gh-grader-preview -f some/other/dir/autograding.json
```

For more information, run `gh-grader-preview -h`.

## Building

`cargo build --release`
