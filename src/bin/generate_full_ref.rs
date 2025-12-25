use clap_markdown::help_markdown_command;
use std::io::{stdout, Write};
use wlout::cli::build_cli;

fn main() {
    let command = build_cli();
    let full_ref = help_markdown_command(&command);

    stdout()
        .write_all(full_ref.as_bytes())
        .expect("Unable to write README.md");
}
