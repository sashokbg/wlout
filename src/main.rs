mod common;
mod list_command;
mod mode_command;
mod off_command;
mod on_command;

use crate::common::AppData;
use crate::list_command::list_command;
use crate::mode_command::{list_modes, mode_command};
use crate::off_command::off_command;
use crate::on_command::on_command;
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

    let name_arg = Arg::new("name")
        .required(true)
        .help("The name of the display")
        .short('n')
        .long("name");

    let matches = Command::new("MyApp")
        .version("1.0")
        .about("Does awesome things")
        .subcommand(
            Command::new("off")
                .about("Turn a display off")
                .arg(&name_arg),
        )
        .subcommand(Command::new("on").about("Turn a display on").arg(&name_arg))
        .subcommand(
            Command::new("mode")
                .about("List and set display mode")
                .arg(&name_arg)
                .arg(Arg::new("mode").required(false)),
        )
        .subcommand(Command::new("list").about("List displays"))
        .get_matches();

    match matches.subcommand() {
        Some(("list", _sub_matches)) => list_command(state),
        Some(("off", _sub_matches)) => {
            let name = _sub_matches.get_one::<String>("name").unwrap();
            let manager = state.manager.as_ref().expect("output manager not bound");
            let serial: u32 = state.config_serial.unwrap();
            let configuration = manager.create_configuration(serial, &qh, ());

            off_command(name, state, configuration, event_queue)
        }
        Some(("on", _sub_matches)) => {
            let name = _sub_matches.get_one::<String>("name").unwrap();
            let manager = state.manager.as_ref().expect("output manager not bound");
            let serial: u32 = state.config_serial.unwrap();
            let configuration = manager.create_configuration(serial, &qh, ());

            on_command(name, state, configuration, event_queue)
        }
        Some(("mode", _sub_matches)) => {
            let name = _sub_matches.get_one::<String>("name").unwrap();
            let manager = state.manager.as_ref().expect("output manager not bound");
            let serial: u32 = state.config_serial.unwrap();
            let configuration = manager.create_configuration(serial, &qh, ());

            match _sub_matches.get_one::<String>("mode") {
                Some(name) => mode_command(name, state, configuration, event_queue),
                None => {
                    list_modes(name, state);
                }
            }
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
