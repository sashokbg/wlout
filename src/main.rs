// wlout - Wayland (Wl Roots) Display Manager CLI with UNIX philosophy and modern goodies
//
// Copyright (C) 2025 Aleksandar KIRILOV
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod handles;
mod parsers;

mod backends;
mod cli;
mod commands;
mod head_printer;
mod model;

use crate::cli::{build_cli, NAME_ARG_ID};
use crate::commands::commands::{Executable, InfoCommand, ListCommand, MirrorCommand};
use crate::commands::completion_command::completion_command;
use crate::commands::mode_command::{
    ModeAutoCommand, ModeCurrentCommand, ModeListCommand, ModePreferredCommand, ModeSetCommand,
};
use crate::commands::move_command::{
    move_command, move_relative_command, REL_POS_ABOVE, REL_POS_BELOW, REL_POS_LEFT_OF,
    REL_POS_RIGHT_OF,
};
use crate::commands::power_command::power_command;
use crate::handles::OUTPUT_MANAGER_INTERFACE_NAME;
use crate::model::{AppData, HeadModeInput};
use std::collections::HashMap;
use std::process::exit;
use wayland_client::{Connection, EventQueue};

pub fn run() {
    let matches = build_cli().get_matches();
    if let Some(("completion", sub_matches)) = matches.subcommand() {
        let mut new_cli = build_cli();
        completion_command(sub_matches, &mut new_cli);
        return;
    }

    let (mut event_queue, mut state) = connect_wayland_dm();

    match matches.subcommand() {
        Some(("power", sub_matches)) => {
            let name = sub_matches
                .get_one::<String>(NAME_ARG_ID)
                .expect(format!("{} is required", NAME_ARG_ID).as_str());

            let power_mode = sub_matches.get_one::<String>("power_mode").unwrap();
            let force = sub_matches.get_one::<bool>("force").unwrap();

            match power_mode.as_str() {
                "on" => power_command(name, true, state, event_queue, force),
                "off" => power_command(name, false, state, event_queue, force),
                &_ => {
                    eprintln!("Power mode should be on / off");
                    exit(1);
                }
            }
        }
        Some(("list", sub_matches)) => {
            let verbose = sub_matches.get_one::<bool>("verbose").unwrap().clone();

            ListCommand { verbose }.execute()
        }
        Some(("info", sub_matches)) => {
            let name = sub_matches
                .get_one::<String>(cli::NAME_ARG_ID)
                .expect(format!("{} is required", cli::NAME_ARG_ID).as_str())
                .clone();

            InfoCommand { name }.execute();
        }
        Some(("move", sub_matches)) => {
            let name = sub_matches
                .get_one::<String>(cli::NAME_ARG_ID)
                .expect(format!("{} is required", cli::NAME_ARG_ID).as_str());

            let sub_sub_matches = sub_matches.subcommand();
            match sub_sub_matches {
                Some((REL_POS_ABOVE, sub_sub_sub_matches)) => {
                    let other_display = sub_sub_sub_matches
                        .get_one::<String>("other_display")
                        .unwrap();
                    if name == other_display {
                        eprintln!("The second display must be different !");
                        exit(1);
                    }

                    move_relative_command(
                        name,
                        other_display,
                        REL_POS_ABOVE,
                        &mut state,
                        event_queue,
                    );
                }
                Some((REL_POS_BELOW, sub_sub_sub_matches)) => {
                    let other_display = sub_sub_sub_matches
                        .get_one::<String>("other_display")
                        .unwrap();
                    if name == other_display {
                        eprintln!("The second display must be different !");
                        exit(1);
                    }

                    move_relative_command(
                        name,
                        other_display,
                        REL_POS_BELOW,
                        &mut state,
                        event_queue,
                    );
                }
                Some(("right-of", sub_sub_sub_matches)) => {
                    let other_display = sub_sub_sub_matches
                        .get_one::<String>("other_display")
                        .unwrap();
                    if name == other_display {
                        eprintln!("The second display must be different !");
                        exit(1);
                    }

                    move_relative_command(
                        name,
                        other_display,
                        REL_POS_RIGHT_OF,
                        &mut state,
                        event_queue,
                    );
                }
                Some(("left-of", sub_sub_sub_matches)) => {
                    let other_display = sub_sub_sub_matches
                        .get_one::<String>("other_display")
                        .unwrap();
                    if name == other_display {
                        eprintln!("The second display must be different !");
                        exit(1);
                    }

                    move_relative_command(
                        name,
                        other_display,
                        REL_POS_LEFT_OF,
                        &mut state,
                        event_queue,
                    );
                }
                Some(("position", sub_sub_sub_matches)) => {
                    let x = sub_sub_sub_matches.get_one::<i32>("x").unwrap();
                    let y = sub_sub_sub_matches.get_one::<i32>("y").unwrap();

                    move_command(name, x.clone(), y.clone(), state, event_queue);
                }
                None => todo!(),
                Some((&_, _)) => todo!(),
            }
        }
        Some(("mirror", sub_matches)) => {
            let name = sub_matches
                .get_one::<String>(NAME_ARG_ID)
                .expect(format!("{} is required", NAME_ARG_ID).as_str());

            match sub_matches.subcommand() {
                Some(("same-as", sub_sub_matches)) => {
                    let other_display = sub_sub_matches.get_one::<String>("other_display").unwrap();
                    if name == other_display {
                        eprintln!("The second display must be different !");
                        exit(1);
                    }

                    MirrorCommand {
                        mirrored_display_name: name.clone(),
                        reference_display_name: other_display.clone(),
                    }
                    .execute()
                }
                _ => {}
            }
        }
        Some(("mode", sub_matches)) => {
            let name = sub_matches
                .get_one::<String>(NAME_ARG_ID)
                .expect(format!("{} is required", NAME_ARG_ID).as_str())
                .clone();

            match sub_matches.subcommand() {
                Some(("current", _)) => {
                    ModeCurrentCommand { name: name.clone() }.execute();
                }
                Some(("preferred", _)) => {
                    ModePreferredCommand { name: name.clone() }.execute();
                }
                Some(("auto", _)) => {
                    ModeAutoCommand { name: name.clone() }.execute();
                }
                Some(("list", _)) => {
                    ModeListCommand { name: name.clone() }.execute();
                }
                Some(("set", sub_sub_matches)) => {
                    match sub_sub_matches.get_one::<HeadModeInput>("mode") {
                        Some(mode) => {
                            let force = *sub_sub_matches.get_one::<bool>("force").unwrap();
                            ModeSetCommand {
                                name: name.clone(),
                                mode: mode.clone(),
                                force,
                            }
                            .execute()
                        }
                        None => {}
                    }
                }
                None => {
                    ModeListCommand { name: name.clone() }.execute();
                }
                Some((&_, _)) => todo!(),
            }
        }
        None => {
            let verbose = matches.get_one::<bool>("verbose").unwrap().clone();
            ListCommand { verbose }.execute()
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

pub(crate) fn connect_wayland_dm() -> (EventQueue<AppData>, AppData) {
    let conn = Connection::connect_to_env().expect("failed to connect to a Wayland compositor");
    let display = conn.display();
    let mut event_queue = conn.new_event_queue::<AppData>();

    let _registry = display.get_registry(&event_queue.handle(), ());

    let mut state = AppData {
        initial_done: false,
        heads: HashMap::new(),
        manager: None,
        config_result: None,
        config_serial: None,
        output_manager_found: false,
    };

    event_queue.roundtrip(&mut state).unwrap();

    if !state.output_manager_found {
        eprintln!(
            "Your system does not support the {} interface. This tool only works on wlroots compositors.",
            OUTPUT_MANAGER_INTERFACE_NAME
        );
        exit(1)
    }

    while !state.initial_done {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }
    (event_queue, state)
}

fn main() {
    run()
}
