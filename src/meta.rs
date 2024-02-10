use clap::CommandFactory;
use clap_complete::Shell;
use clap_mangen::Man;

use crate::cli::Cli;

pub fn gen_man_page() -> String {
    let man = Man::new(Cli::command());
    let mut buf: Vec<u8> = vec![];
    man.render(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

pub fn gen_completions(shell: Shell) -> String {
    let mut cmd = Cli::command();
    let mut buf: Vec<u8> = vec![];
    clap_complete::generate(shell, &mut cmd, "gh-grader-preview", &mut buf);
    String::from_utf8(buf).unwrap()
}
