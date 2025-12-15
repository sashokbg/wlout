use std::io;
use clap::{ArgMatches, Command};
use clap_complete::{generate, Shell};

pub fn completion_command(matches: &ArgMatches, cmd: &mut Command) {
    let shell = matches
        .get_one::<Shell>("shell")
        .copied()
        .expect("Shell argument required");

    eprintln!("Generating completion file for {shell}...");
    generate(shell, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
