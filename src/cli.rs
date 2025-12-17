use crate::commands::completion_command::completion_command;
use crate::commands::info_command::info_command;
use crate::commands::list_command::list_command;
use crate::commands::mode_command::{
    mode_current_command, mode_list_command, mode_preferred_command, mode_set_command,
};
use crate::commands::move_command::move_command;
use crate::commands::power_command::{off_command, on_command};
use crate::common::{AppData, HeadModeInput};
use crate::parsers::DisplayModeParser;
use clap::{value_parser, Arg, ArgAction, Command};
use clap_complete::aot::Shell;
use std::collections::HashMap;
use std::process::exit;
use wayland_client::{Connection, EventQueue, QueueHandle};

static NAME_ARG_ID: &str = "display";

fn build_cli() -> Command {
    let display_arg = Arg::new(NAME_ARG_ID)
        .required(true)
        .help("The name of the display");

    Command::new("wlout")
        .version("1.0")
        .about("Manage wayland compositor display outputs (heads).")
        .long_about("
Manage wayland compositor display outputs (heads).

This tool allows you to manage the display mode, refresh rate and position of your displays, also known as heads in Wayland protocol.
It is based on the still experimental wlr-output-management-unstable-v1 protocol and is subjected to breaking changes.

For more information please visit: https://wayland.app/protocols/wlr-output-management-unstable-v1
        ")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("completion")
                .about("Generate shell completion script")
                .arg(
                    Arg::new("shell")
                        .required(true)
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(Shell)),
                ),
        )
        .subcommand(
            Command::new("power")
                .about("Turn a display on or off")
                .arg_required_else_help(true)
                .arg(display_arg.clone())
                .arg(Arg::new("power_mode")
                         .required(true)
                         .action(ArgAction::Set)
                         .value_parser(clap::builder::PossibleValuesParser::new(
                             ["on", "off"]
                         ),),
                )
        )
        .subcommand(
            Command::new("move")
                .about("Set the position of the display in the global compositor space")
                .subcommand_required(true)
                .arg(display_arg.clone())
                .subcommand(
                    Command::new("position")
                        .about("Move the display to an absolute position defined by x and y coordinates on the global compositor space")
                        .arg(
                            Arg::new("x")
                                .required(true)
                                .help("x - coordinate")
                                .value_parser(
                                    value_parser!(i32)
                                )
                        )
                        .arg(
                            Arg::new("y")
                                .required(true)
                                .help("y - coordinate")
                                .value_parser(
                                    value_parser!(i32)
                                )
                        )
                )

        )
        .subcommand(
            Command::new("mode")
                .about("Manage the display mode resolution and refresh rate")
                .arg(display_arg.clone())
                .subcommand(Command::new("list")
                    .about("List the available modes for a display"))
                .subcommand(Command::new("current")
                    .about("Show the current mode for this display"))
                .subcommand(Command::new("preferred")
                    .about("Show the advertised preferred mode for this display"))
                .subcommand(Command::new("set")
                    .arg_required_else_help(true)
                    .about("Set the resolution and refresh rate for the display")
                    .arg(
                        Arg::new("mode")
                            .help("The mode format is <WIDTH>x<HEIGHT>@<RATE>")
                            .value_parser(DisplayModeParser {}),
                    )
                )
        )
        .subcommand(Command::new("list")
            .arg(Arg::new("verbose")
                     .help("Turn on verbose / detailed mode")
                     .long("verbose")
                     .short('v')
                     .action(clap::ArgAction::SetTrue),
            )
            .about("List displays"))
        .subcommand(Command::new("info")
            .about("Print additional detailed information for a display")
            .arg_required_else_help(true)
            .arg(display_arg.clone())
        )
}

fn connect_wayland_dm() -> (EventQueue<AppData>, QueueHandle<AppData>, AppData) {
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
    (event_queue, qh, state)
}

pub fn run() {
    let matches = build_cli().get_matches();

    if let Some(("completion", sub_matches)) = matches.subcommand() {
        let mut new_cli = build_cli();
        completion_command(sub_matches, &mut new_cli);
        return;
    }

    let (event_queue, qh, state) = connect_wayland_dm();

    match matches.subcommand() {
        Some(("power", sub_matches)) => {
            let name = sub_matches
                .get_one::<String>(NAME_ARG_ID)
                .expect(format!("{} is required", NAME_ARG_ID).as_str());

            let power_mode = sub_matches.get_one::<String>("power_mode").unwrap();

            let manager = state.manager.as_ref().expect("output manager not bound");
            let serial: u32 = state.config_serial.unwrap();
            let configuration = manager.create_configuration(serial, &qh, ());

            match power_mode.as_str() {
                "on" => on_command(name, state, configuration, event_queue),
                "off" => off_command(name, state, configuration, event_queue),
                &_ => {
                    eprintln!("Power mode should be on / off");
                    exit(1);
                }
            }
        }
        Some(("list", sub_matches)) => {
            let verbose = sub_matches.get_one::<bool>("verbose").unwrap();
            list_command(state, verbose.clone());
        }
        Some(("info", sub_matches)) => {
            let name = sub_matches
                .get_one::<String>(NAME_ARG_ID)
                .expect(format!("{} is required", NAME_ARG_ID).as_str());

            info_command(name, state);
        }
        Some(("move", sub_matches)) => {
            let name = sub_matches
                .get_one::<String>(NAME_ARG_ID)
                .expect(format!("{} is required", NAME_ARG_ID).as_str());

            let sub_sub_matches = sub_matches.subcommand();
            match sub_sub_matches {
                Some(("position", sub_sub_sub_matches)) => {
                    let x = sub_sub_sub_matches.get_one::<i32>("x").unwrap();
                    let y = sub_sub_sub_matches.get_one::<i32>("y").unwrap();

                    let manager = state.manager.as_ref().expect("output manager not bound");
                    let serial: u32 = state.config_serial.unwrap();
                    let configuration = manager.create_configuration(serial, &qh, ());

                    move_command(
                        name,
                        x.clone(),
                        y.clone(),
                        state,
                        configuration,
                        event_queue,
                    );
                }
                None => todo!(),
                Some((&_, _)) => todo!(),
            }
        }
        Some(("mode", sub_matches)) => {
            let name = sub_matches
                .get_one::<String>(NAME_ARG_ID)
                .expect(format!("{} is required", NAME_ARG_ID).as_str());

            let manager = state.manager.as_ref().expect("output manager not bound");
            let serial: u32 = state.config_serial.unwrap();
            let configuration = manager.create_configuration(serial, &qh, ());

            match sub_matches.subcommand() {
                Some(("current", _)) => {
                    mode_current_command(name, state);
                }
                Some(("preferred", _)) => {
                    mode_preferred_command(name, state);
                }
                Some(("list", _)) => {
                    mode_list_command(name, state);
                }
                Some(("set", sub_sub_matches)) => {
                    match sub_sub_matches.get_one::<HeadModeInput>("mode") {
                        Some(mode) => {
                            mode_set_command(name, mode, state, configuration, event_queue)
                        }
                        None => {}
                    }
                }
                None => {
                    mode_list_command(name, state);
                }
                Some((&_, _)) => todo!(),
            }
        }
        None => {
            let verbose = matches.get_one::<bool>("verbose").unwrap();
            list_command(state, verbose.clone())
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
