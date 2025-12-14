mod common;
mod list_command;
mod mode_list_command;
mod off_command;
mod on_command;

use crate::common::AppData;
use crate::list_command::list_command;
use crate::mode_list_command::{mode_list_command, mode_set_command};
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
        .help("The name of the display")
        .short('n')
        .long("name");

    let matches = Command::new("MyApp")
        .version("1.0")
        .about("Does awesome things")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("off")
                .about("Turn a display off")
                .arg(&name_arg),
        )
        .subcommand(Command::new("on").about("Turn a display on").arg(&name_arg))
        .subcommand(
            Command::new("mode")
                .about("Manage the display mode resolution and refresh rate")
                .arg_required_else_help(true)
                .arg(&name_arg.clone().global(true))
                .subcommand(Command::new("list").about("List the available mods for a display"))
                .subcommand(
                    Command::new("set").about("Set the mode for a display").arg(
                        Arg::new("mode")
                            .help("The mode format is <WIDTH>x<HEIGHT>@<RATE>")
                            .required(true)
                            .long("mode"),
                    ),
                ),
        )
        .subcommand(Command::new("list").about("List displays"))
        .get_matches();

    match matches.subcommand() {
        Some(("list", _sub_matches)) => list_command(state),
        Some(("off", _sub_matches)) => {
            let name = _sub_matches
                .get_one::<String>("name")
                .expect("--name is required`");

            let manager = state.manager.as_ref().expect("output manager not bound");
            let serial: u32 = state.config_serial.unwrap();
            let configuration = manager.create_configuration(serial, &qh, ());

            off_command(name, state, configuration, event_queue)
        }
        Some(("on", _sub_matches)) => {
            let name = _sub_matches
                .get_one::<String>("name")
                .expect("--name is required`");
            let manager = state.manager.as_ref().expect("output manager not bound");
            let serial: u32 = state.config_serial.unwrap();
            let configuration = manager.create_configuration(serial, &qh, ());

            on_command(name, state, configuration, event_queue)
        }
        Some(("mode", _sub_matches)) => {
            let manager = state.manager.as_ref().expect("output manager not bound");
            let serial: u32 = state.config_serial.unwrap();
            let configuration = manager.create_configuration(serial, &qh, ());

            match _sub_matches.subcommand() {
                Some(("list", _sub_sub_command)) => {
                    let name = _sub_matches
                        .get_one::<String>("name")
                        .expect("--name is required");
                    mode_list_command(name, state);
                }
                Some(("set", _sub_sub_command)) => {
                    let name = _sub_matches
                        .get_one::<String>("name")
                        .expect("--name is required");
                    match _sub_sub_command.get_one::<String>("mode") {
                        Some(mode) => {
                            mode_set_command(name, mode, state, &configuration, event_queue)
                        }
                        None => {}
                    }
                }
                None => {}
                _ => {}
            }
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
