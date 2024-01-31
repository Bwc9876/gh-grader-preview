mod cli;
mod grader;
mod runner;

use std::time::Duration;

use anyhow::Result;
use clap::Parser;

use cli::Cli;
use grader::AutoGraderData;
use indicatif::ProgressBar;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let grader_data = AutoGraderData::get(cli.file)?;

    let test_len = grader_data.tests.len();

    let amount_passed = grader_data
        .tests
        .into_iter()
        .enumerate()
        .filter(|(i, test)| {
            let bar = ProgressBar::new_spinner();
            bar.set_message(format!("Running {}", test.name));
            bar.enable_steady_tick(Duration::from_millis(100));
            if cli.skip.map(|skip| i < &skip).unwrap_or(false)
                || cli
                    .test
                    .as_ref()
                    .map(|target| target.to_lowercase() != test.name.to_lowercase())
                    .unwrap_or(false)
            {
                bar.finish_with_message(format!("〰️ Skipped {}", test.name));
                return false;
            }
            match test.run() {
                Ok((matched, result)) => {
                    if cli.verbose {
                        println!(
                            "Stdout of {}:\n{}\n\nStderr of {}:\n{}",
                            test.name, result.stdout, test.name, result.stderr
                        );
                    }
                    match result.status {
                        Ok(code) => {
                            if code != 0 {
                                bar.finish_with_message(format!(
                                    "⚠️ {} Exited with code {code}",
                                    test.name
                                ))
                            }
                            if matched {
                                bar.finish_with_message(format!("✅ {} passed!", test.name));
                                true
                            } else {
                                bar.finish_with_message(format!(
                                    "❌ {} did not give the correct output",
                                    test.name
                                ));
                                false
                            }
                        }
                        Err(why) => {
                            bar.finish_with_message(format!(
                                "❌ Failed to run {}: {why:?}",
                                test.name
                            ));
                            false
                        }
                    }
                }
                Err(why) => {
                    bar.finish_with_message(format!("❌ Failed to run {}: {why:?}", test.name));
                    false
                }
            }
        })
        .count();

    println!("Passed {amount_passed}/{test_len} tests");
    if cli.skip.is_some() || cli.test.is_some() {
        println!("(Some were skipped)");
    }

    Ok(())
}
