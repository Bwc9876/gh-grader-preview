use std::fmt::Display;

use anyhow::Result;
use serde::Deserialize;

use crate::runner::TestResult;

use super::runner;

const AUTO_GRADER_DEFAULT_PATH: &str = ".github/classroom/autograding.json";

#[derive(Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonType {
    #[default]
    Pass,
    Included,
    Excluded,
    Exact,
    Regex,
}

impl Display for ComparisonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ComparisonType::Included => "include this string",
            ComparisonType::Excluded => "not include this string",
            ComparisonType::Exact => "exactly match this string",
            ComparisonType::Regex => "match against this regular expression",
            ComparisonType::Pass => "can be anything",
        };
        f.write_str(s)
    }
}

#[derive(Deserialize)]
pub struct TestCase {
    pub name: String,
    pub setup: Option<String>,
    pub run: String,
    #[serde(default)]
    pub input: String,
    pub output: Option<String>,
    #[serde(default)]
    pub comparison: ComparisonType,
    pub timeout: u32,
    #[allow(unused)]
    pub points: Option<u32>,
}

#[derive(Deserialize)]
pub struct AutoGraderData {
    pub tests: Vec<TestCase>,
}

impl AutoGraderData {
    pub fn parse(data: &str) -> Result<Self> {
        let data: AutoGraderData = serde_json::from_str(data)?;
        Ok(data)
    }

    pub fn read_file(path: &str) -> Result<Self> {
        println!("Reading auto-grader JSON from `{path}`...");
        let data = std::fs::read_to_string(path)?;
        Self::parse(&data)
    }

    pub fn get(path: Option<String>) -> Result<Self> {
        let path = path.unwrap_or(AUTO_GRADER_DEFAULT_PATH.to_string());
        Self::read_file(&path)
    }
}

impl TestCase {
    pub fn check_output(&self, output: String) -> Result<bool> {
        if let Some(ref expected_output) = self.output {
            match self.comparison {
                ComparisonType::Included => Ok(output.contains(expected_output)),
                ComparisonType::Excluded => Ok(!output.contains(expected_output)),
                ComparisonType::Exact => Ok(output.trim() == expected_output.trim()),
                ComparisonType::Regex => {
                    let re = regex::Regex::new(expected_output)?;
                    Ok(re.is_match(&output))
                }
                ComparisonType::Pass => Ok(true),
            }
        } else {
            Ok(true)
        }
    }

    pub fn run(&self) -> Result<(bool, TestResult)> {
        if let Some(setup) = self.setup.as_ref() {
            runner::setup_phase(setup)?;
        }
        let res = runner::run_phase(&self.run, &self.input, self.timeout as u64)?;
        let matches = self.check_output(res.stdout.clone())?;
        Ok((matches, res))
    }
}
