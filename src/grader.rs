use anyhow::Result;
use serde::Deserialize;

use crate::runner::TestResult;

use super::runner;

const AUTO_GRADER_DEFAULT_PATH: &str = ".github/classroom/autograding.json";

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonType {
    Included,
    Excluded,
    Regex,
}

#[derive(Deserialize)]
pub struct TestCase {
    pub name: String,
    pub setup: String,
    pub run: String,
    pub input: String,
    pub output: String,
    pub comparison: ComparisonType,
    pub timeout: u32,
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
        match self.comparison {
            ComparisonType::Included => Ok(output.contains(&self.output)),
            ComparisonType::Excluded => Ok(!output.contains(&self.output)),
            ComparisonType::Regex => {
                let re = regex::Regex::new(&self.output)?;
                Ok(re.is_match(&output))
            }
        }
    }

    pub fn run(&self) -> Result<(bool, TestResult)> {
        runner::setup_phase(&self.setup)?;
        let res = runner::run_phase(&self.run, &self.input, self.timeout as u64)?;
        let matches = self.check_output(res.stdout.clone())?;
        Ok((matches, res))
    }
}
