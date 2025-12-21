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

mod cli;
mod commands;
mod head_printer;
mod model;

use crate::cli::{build_cli, NAME_ARG_ID};
use crate::commands::completion_command::completion_command;
use crate::commands::info_command::info_command;
use crate::commands::list_command::list_command;
use crate::commands::mirror_command::mirror_command;
use crate::commands::mode_command::{
    mode_auto_command, mode_current_command, mode_list_command, mode_preferred_command,
    mode_set_command,
};
use crate::commands::move_command::{
    move_command, move_relative_command, REL_POS_ABOVE, REL_POS_BELOW, REL_POS_LEFT_OF,
    REL_POS_RIGHT_OF,
};
use crate::commands::power_command::power_command;
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
            let verbose = sub_matches.get_one::<bool>("verbose").unwrap();
            list_command(state, verbose.clone());
        }
        Some(("info", sub_matches)) => {
            let name = sub_matches
                .get_one::<String>(cli::NAME_ARG_ID)
                .expect(format!("{} is required", cli::NAME_ARG_ID).as_str());

            info_command(name, state);
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

                    mirror_command(name, other_display, &mut state, event_queue);
                }
                _ => {}
            }
        }
        Some(("mode", sub_matches)) => {
            let name = sub_matches
                .get_one::<String>(NAME_ARG_ID)
                .expect(format!("{} is required", NAME_ARG_ID).as_str());

            match sub_matches.subcommand() {
                Some(("current", _)) => {
                    mode_current_command(name, state);
                }
                Some(("preferred", _)) => {
                    mode_preferred_command(name, &state);
                }
                Some(("auto", _)) => {
                    mode_auto_command(name, &mut state, &mut event_queue);
                }
                Some(("list", _)) => {
                    mode_list_command(name, state);
                }
                Some(("set", sub_sub_matches)) => {
                    match sub_sub_matches.get_one::<HeadModeInput>("mode") {
                        Some(mode) => mode_set_command(name, mode, state, event_queue),
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
    };

    event_queue.roundtrip(&mut state).unwrap();
    while !state.initial_done {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }
    (event_queue, state)
}

fn main() {
    run()
}
