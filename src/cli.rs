use crate::common::{AppData, HeadModeInput};
use crate::parsers::RefreshRateParser;
use clap::{Arg, Command};
use std::collections::HashMap;
use wayland_client::{Connection, EventQueue, QueueHandle};
use crate::commands::list_command::list_command;
use crate::commands::mode_command::{mode_list_command, mode_set_command};
use crate::commands::off_command::off_command;
use crate::commands::on_command::on_command;

fn build_cli() -> (Command, EventQueue<AppData>, QueueHandle<AppData>) {
    let conn = Connection::connect_to_env().expect("failed to connect to a Wayland compositor");
    let display = conn.display();
    let event_queue = conn.new_event_queue::<AppData>();
    let qh = event_queue.handle();

    let _registry = display.get_registry(&qh, ());

    let name_arg = Arg::new("name")
        .help("The name of the display")
        .short('n')
        .long("name");

    return (Command::new("MyApp")
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
                            .long("mode")
                            .short('m')
                            .value_parser(RefreshRateParser {}),
                    ),
                ),
        )
        .subcommand(Command::new("list").about("List displays")), event_queue, qh);
}

pub fn run() {
    let (cli, mut event_queue, qh) = build_cli();
    let matches = cli.get_matches();

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
                    match _sub_sub_command.get_one::<HeadModeInput>("mode") {
                        Some(mode) => {
                            mode_set_command(name, mode, state, configuration, event_queue)
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
