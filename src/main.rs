mod list_command;
mod off_command;
mod common;

use crate::list_command::list_command;
use crate::off_command::off_command;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("MyApp")
        .version("1.0")
        .about("Does awesome things")
        .subcommand(
            Command::new("off").about("Turn a display off").arg(
                Arg::new("name")
                    .help("The name of the display")
                    .short('n')
                    .long("name"),
            ),
        )
        .subcommand(Command::new("list").about("List displays"))
        .get_matches();

    match matches.subcommand() {
        Some(("list", _sub_matches)) => list_command(),
        Some(("off", _sub_matches)) => {
            let name = _sub_matches.get_one::<String>("name").unwrap();
            off_command(name)
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
