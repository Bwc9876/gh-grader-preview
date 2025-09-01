mod cli;
mod grader;
mod meta;
mod runner;

use std::time::Duration;

use anyhow::Result;
use clap::Parser;

use cli::Cli;
use grader::AutoGraderData;
use indicatif::ProgressBar;

use crate::{grader::ComparisonType, runner::TestResult};

struct CmdOutput(String, String, Option<String>);

impl CmdOutput {
    fn format_output(self, name: &str, comp: &ComparisonType) -> String {
        let CmdOutput(stdout, stderr, expected) = self;

        let heading = format!("┏━┻ Stdout of {name}");
        let stdout = stdout.trim().replace("\n", "\n┃ ");
        let mid = format!("┣━━ Stderr of {name}");
        let stderr = stderr.trim().replace("\n", "\n┃ ");
        let (mid2, expected) = if let Some(expected) = expected {
            (
                format!("┣━━ Stdout of {name} must {comp}"),
                expected.trim().replace("\n", "\n┃ "),
            )
        } else {
            (format!("┣━━ {name} has no output check"), String::new())
        };
        let foot = "┗━━ End of output".to_string();

        format!("{heading}\n┃ {stdout}\n{mid}\n┃ {stderr}\n{mid2}\n┃ {expected}\n{foot}")
    }
}

#[derive(Debug)]
enum TestOutput {
    Passed,
    LogicError,
    NonZeroExit(Option<i32>),
    RunnerError(anyhow::Error),
    Skipped,
}

impl From<Result<(bool, TestResult)>> for TestOutput {
    fn from(res: Result<(bool, TestResult)>) -> Self {
        match res {
            Ok((matched, TestResult { status: Ok(0), .. })) => {
                if matched {
                    Self::Passed
                } else {
                    Self::LogicError
                }
            }
            Ok((_, TestResult { status, .. })) => Self::NonZeroExit(status.ok()),
            Err(why) => Self::RunnerError(why),
        }
    }
}

impl TestOutput {
    fn output(&self) -> (&str, String) {
        match self {
            TestOutput::Passed => ("✅", "Passed".to_string()),
            TestOutput::LogicError => ("❌", "Did not match expected output".to_string()),
            TestOutput::NonZeroExit(code) => (
                "💥",
                format!("Exited with code {}", code.unwrap_or(i32::MAX)),
            ),
            TestOutput::RunnerError(error) => ("🙈", format!("Wasn't able to run: {error:?}")),
            TestOutput::Skipped => ("〰️", "Was skipped".to_string()),
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.man_gen {
        println!("{}", meta::gen_man_page());
        return Ok(());
    } else if let Some(shell) = cli.completions {
        println!("{}", meta::gen_completions(shell.parse().unwrap()));
        return Ok(());
    }

    let target_test = cli.test.as_ref().map(|s| s.to_lowercase());

    let grader_data = AutoGraderData::get(cli.file)?;

    let outputs = grader_data
        .tests
        .into_iter()
        .enumerate()
        .map(|(i, test)| {
            // In verbose mode make the output a bit easier to follow
            if cli.verbose {
                println!();
            }
            let bar = ProgressBar::new_spinner();
            bar.set_message(format!("Running {}", test.name));
            bar.enable_steady_tick(Duration::from_millis(100));

            let skip_number = cli.skip.map(|skip| i < skip).unwrap_or(false);

            let skip_non_target = target_test
                .as_ref()
                .is_some_and(|s| *s != test.name.to_lowercase());

            let (output, verbose_output) = if skip_number || skip_non_target {
                (TestOutput::Skipped, None)
            } else {
                let result = test.run();
                let verbose_output =
                    result
                        .as_ref()
                        .ok()
                        .map(|(_, TestResult { stdout, stderr, .. })| {
                            CmdOutput(stdout.clone(), stderr.clone(), test.output.clone())
                        });
                (result.into(), verbose_output)
            };

            let (emoji, msg) = output.output();

            bar.finish_with_message(format!("{emoji} {} - {msg}", test.name));

            if let (Some(verbose_output), true) = (verbose_output, cli.verbose) {
                println!(
                    "{}",
                    verbose_output.format_output(&test.name, &test.comparison)
                );
            }

            output
        })
        .collect::<Vec<_>>();

    let total_tests = outputs.len();
    let total_not_skipped = outputs
        .iter()
        .filter(|o| !matches!(o, TestOutput::Skipped))
        .count();
    let total_passed = outputs
        .iter()
        .filter(|o| matches!(o, TestOutput::Passed))
        .count();

    let percent_passed = total_passed as f32 / total_not_skipped as f32 * 100_f32;

    println!("\n== TEST SUMMARY ==");
    println!("{total_passed} / {total_not_skipped} Tests Passed ({percent_passed:.2}%)");
    if total_tests != total_not_skipped {
        println!("({} Tests Were Skipped)", total_tests - total_not_skipped);
    }

    Ok(())
}
