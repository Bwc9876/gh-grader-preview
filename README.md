# GitHub Grader Preview

Simple program that runs test cases specified in an autograding.json file and reports results. Useful for previewing how GitHub will run your autograder and project.

> [!WARNING]
> 
> While this works on Windows, the GitHub classroom runner uses Linux by default, and so some professors may write commands
> specific to Linux meaning they won't work for you.

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
