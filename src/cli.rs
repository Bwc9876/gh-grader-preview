use clap::{command, Parser};

#[derive(Parser)]
#[command(name="gh-grader-preview", author, version, about, long_about = None)]
pub struct Cli {
    #[arg(
        short = 'f',
        long = "file",
        help = "Override the autograder.json file to use, by default we look in `.github/classroom/autograder.json`"
    )]
    pub file: Option<String>,
    #[arg(
        short = 'v',
        long = "verbose",
        help = "Show stdout and stderr of tests"
    )]
    pub verbose: bool,
    #[arg(
        short = 't',
        long = "test",
        help = "Run only the test specified (must match `name` case-insensitively)"
    )]
    pub test: Option<String>,
    #[arg(
        short = 'x',
        long = "skip",
        help = "Skip the first X tests, useful if you have tests that are purely informational"
    )]
    pub skip: Option<usize>,
}
