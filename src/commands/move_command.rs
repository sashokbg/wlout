use crate::common::{AppData, ConfigResult};
use std::process;
use wayland_client::EventQueue;
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_configuration_head_v1::ZwlrOutputConfigurationHeadV1;
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_configuration_v1::ZwlrOutputConfigurationV1;

pub fn move_command(
    name: &str,
    x: i32,
    y: i32,
    mut state: AppData,
    configuration: ZwlrOutputConfigurationV1,
    mut event_queue: EventQueue<AppData>,
) {
    let target_head = state
        .heads
        .values()
        .find(|head| head.name.as_deref() == Some(name))
        .expect(&*format!("Display \"{}\" not found", name));

    let qh = event_queue.handle();
    let head_config: ZwlrOutputConfigurationHeadV1 =
        configuration.enable_head(&target_head.head, &qh, ());

    head_config.set_position(x as i32, y as i32);

    state.config_result = None;
    configuration.apply();

    while state.config_result.is_none() {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }

    match state.config_result.unwrap() {
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

    configuration.destroy();
}
