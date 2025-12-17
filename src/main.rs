mod common;
mod parsers;

mod cli;
mod commands;
mod head_printer;

use crate::cli::run;

fn main() {
    run()
}
