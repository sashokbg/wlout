mod common;
mod list_command;
mod off_command;

use crate::common::AppData;
use crate::list_command::list_command;
use crate::off_command::off_command;
use clap::{Arg, Command};
use std::collections::HashMap;
use wayland_client::Connection;

fn main() {
    let conn = Connection::connect_to_env().expect("failed to connect to a Wayland compositor");
    let display = conn.display();
    let mut event_queue = conn.new_event_queue::<AppData>();
    let qh = event_queue.handle();

    let _registry = display.get_registry(&qh, ());
    let mut state = AppData {
        initial_done: false,
        heads: HashMap::new(),
        manager: None,
        config_result: None,
        config_serial: None,
    };

    event_queue.roundtrip(&mut state).unwrap();
    while !state.initial_done {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }

    let matches = Command::new("MyApp")
        .version("1.0")
        .about("Does awesome things")
        .subcommand(
            Command::new("off").about("Turn a display off").arg(
                Arg::new("name")
                    .required(true)
                    .help("The name of the display")
                    .short('n')
                    .long("name"), // .value_parser(PossibleValuesParser::new(state.heads.iter().map(|h| h.1.name.unwrap()))),
            ),
        )
        .subcommand(Command::new("list").about("List displays"))
        .get_matches();

    match matches.subcommand() {
        Some(("list", _sub_matches)) => list_command(state),
        Some(("off", _sub_matches)) => {
            let name = _sub_matches.get_one::<String>("name").unwrap();
            off_command(name, state, event_queue)
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
