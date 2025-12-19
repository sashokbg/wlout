use crate::common::{AppData, ConfigResult};
use std::process;
use wayland_client::EventQueue;
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_configuration_v1::ZwlrOutputConfigurationV1;

pub const REL_POS_ABOVE: &str = "above";
pub const REL_POS_BELOW: &str = "below";
pub const REL_POS_LEFT_OF: &str = "left-of";
pub const REL_POS_RIGHT_OF: &str = "right-of";

pub fn move_relative_command(
    moved_display_name: &String,
    reference_display_name: &String,
    pos: &str,
    state: &mut AppData,
    mut event_queue: EventQueue<AppData>,
) {
    let (moved_display_mode, moved_display_info, reference_display_mode, reference_display_info) = {
        let moved_display_info = state
            .get_head(moved_display_name)
            .expect(&*format!("Display \"{}\" not found", moved_display_name));

        let reference_display_info = state.get_head(reference_display_name).expect(&*format!(
            "Display \"{}\" not found",
            reference_display_name
        ));

        let moved_display_mode = moved_display_info
            .get_current_mode()
            .expect("The display has no current mode set. Is it switched on ?");

        let reference_display_mode = reference_display_info
            .get_current_mode()
            .expect("The display has no current mode set. Is it switched on ?");
        (
            moved_display_mode.clone(),
            moved_display_info.clone(),
            reference_display_mode.clone(),
            reference_display_info.clone(),
        )
    };

    let qh = event_queue.handle();

    let result = apply(state, &mut event_queue, |config| {
        let moved_display_config = config.enable_head(&moved_display_info.head, &qh, ());

        match pos {
            REL_POS_ABOVE => {
                moved_display_config.set_position(
                    reference_display_info.position_x.unwrap(),
                    reference_display_info.position_y.unwrap() - moved_display_mode.height,
                );
            }
            REL_POS_BELOW => {
                moved_display_config.set_position(
                    reference_display_info.position_x.unwrap(),
                    reference_display_info.position_y.unwrap() + reference_display_mode.height,
                );
            }
            REL_POS_RIGHT_OF => {
                moved_display_config.set_position(
                    reference_display_info.position_x.unwrap() + reference_display_mode.width,
                    reference_display_info.position_y.unwrap(),
                );
            }
            REL_POS_LEFT_OF => {
                moved_display_config.set_position(
                    reference_display_info.position_x.unwrap() - moved_display_mode.width,
                    reference_display_info.position_y.unwrap(),
                );
            }
            &_ => todo!(),
        }
    });

    let success = format!("Moved display {moved_display_name} {pos} {reference_display_name}");
    let fail =
        format!("Unable to move display {moved_display_name} above {reference_display_name}");
    handle_result(result, &success, &fail)
}

pub fn move_command(
    name: &str,
    x: i32,
    y: i32,
    mut state: AppData,
    mut event_queue: EventQueue<AppData>,
) {
    let target_head = state
        .get_head(name)
        .expect(&*format!("Display \"{}\" not found", name))
        .head
        .clone();

    let qh = event_queue.handle();

    let config_result = apply(&mut state, &mut event_queue, |configuration| {
        let head_config = configuration.enable_head(&target_head, &qh, ());
        head_config.set_position(x, y);
    });

    match config_result {
        ConfigResult::Succeeded => {
            println!("Set position for display {} to x: {} y: {}", name, x, y)
        }
        ConfigResult::Failed => {
            eprintln!("Failed to set position for display {}", name);
            process::exit(1);
        }
        ConfigResult::Cancelled => {
            eprintln!(
                "Configuration cancelled before setting position for display {}",
                name
            );
            process::exit(1);
        }
    }
}

fn apply(
    state: &mut AppData,
    event_queue: &mut EventQueue<AppData>,
    configure: impl FnOnce(&ZwlrOutputConfigurationV1),
) -> ConfigResult {
    let qh = event_queue.handle();
    let manager = state.manager.as_ref().expect("output manager not bound");
    let serial: u32 = state.config_serial.unwrap();
    let configuration = manager.create_configuration(serial, &qh, ());

    state.config_result = None;
    configure(&configuration);
    configuration.apply();

    while state.config_result.is_none() {
        event_queue.blocking_dispatch(state).unwrap();
    }

    configuration.destroy();
    state.config_result.unwrap()
}

fn handle_result(config_result: ConfigResult, success: &str, fail: &str) {
    match config_result {
        ConfigResult::Succeeded => {
            println!("{}", success)
        }
        ConfigResult::Failed => {
            eprintln!("{}", fail);
            process::exit(1);
        }
        ConfigResult::Cancelled => {
            eprintln!("Configuration cancelled",);
            process::exit(1);
        }
    }
}
