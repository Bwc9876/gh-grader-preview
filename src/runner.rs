use std::{
    io::{Read, Write},
    process::{Command, Stdio},
    time::Duration,
};

use anyhow::{anyhow, Result};
use wait_timeout::ChildExt;

pub fn setup_phase(cmd: &str) -> Result<()> {
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .spawn()
        .map_err(|e| anyhow!("Failed to spawn shell: {e:?}"))?;

    child
        .wait()
        .map_err(|e| anyhow!("Failed to execute \"{cmd}\": {e:?}"))?;
    Ok(())
}

pub struct TestResult {
    pub stdout: String,
    pub stderr: String,
    pub status: Result<i32>,
}

fn read_stream<T>(mut stream: T) -> Option<String>
where
    T: Read,
{
    let mut buf = String::new();
    stream
        .read_to_string(&mut buf)
        .map_err(|e| anyhow!("Failed to read stream: {e:?}"))
        .ok()?;
    Some(buf)
}

pub fn run_phase(cmd: &str, input: &str, timeout: u64) -> Result<TestResult> {
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow!("Failed to spawn shell: {e:?}"))?;

    let mut stdin = child
        .stdin
        .as_ref()
        .ok_or_else(|| anyhow!("Failed to get stdin"))?;

    write!(stdin, "{input}").map_err(|e| anyhow!("Failed to write to stdin: {e:?}"))?;

    let duration = Duration::from_secs(timeout * 60);

    let res = child.wait_timeout(duration);

    let nested_res = res
        .map_err(|e| anyhow!("Failed to execute: {e:?}"))
        .map(|opt| opt.ok_or_else(|| anyhow!("Program Timed Out")));

    let err = nested_res
        .and_then(|r| r)
        .map(|status| status.code().unwrap_or(0));

    Ok(TestResult {
        stdout: child.stdout.and_then(read_stream).unwrap_or_default(),
        stderr: child.stderr.and_then(read_stream).unwrap_or_default(),
        status: err,
    })
}
