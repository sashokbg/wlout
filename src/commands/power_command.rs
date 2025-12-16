use crate::common::{AppData, ConfigResult};
use std::process;
use wayland_client::EventQueue;
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_configuration_v1::ZwlrOutputConfigurationV1;

pub fn off_command(
    name: &String,
    mut state: AppData,
    configuration: ZwlrOutputConfigurationV1,
    mut event_queue: EventQueue<AppData>,
) {
    let name_str = name.as_str();
    let error_message = format!("Requested display {} not found", name_str);

    let target_head = state
        .heads
        .values()
        .find(|head| head.name.as_deref() == Some(name_str))
        .expect(error_message.as_str());

    configuration.disable_head(&target_head.head);
    configuration.apply();

    while state.config_result.is_none() {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }

    match state.config_result.unwrap() {
        ConfigResult::Succeeded => {
            println!("Disabled {}", name)
        }
        ConfigResult::Failed => {
            eprintln!("Failed to disable {}", name);
            process::exit(1);
        }
        ConfigResult::Cancelled => {
            eprintln!("Configuration cancelled before disabling {}", name);
            process::exit(1);
        }
    }

    configuration.destroy();
}

pub fn on_command(
    name: &String,
    mut state: AppData,
    configuration: ZwlrOutputConfigurationV1,
    mut event_queue: EventQueue<AppData>,
) {

    let name_str = name.as_str();
    let error_message = format!("Requested display {} not found", name_str);

    let target_head = state
        .heads
        .values()
        .find(|head| head.name.as_deref() == Some(name.as_str()))
        .expect(error_message.as_str());

    let qh = event_queue.handle();

    configuration.enable_head(&target_head.head, &qh, ());
    configuration.apply();

    while state.config_result.is_none() {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }

    match state.config_result.unwrap() {
        ConfigResult::Succeeded => {
            println!("Enabled {}", name)
        }
        ConfigResult::Failed => {
            eprintln!("Failed to disable {}", name);
            process::exit(1);
        }
        ConfigResult::Cancelled => {
            eprintln!("Configuration cancelled before disabling {}", name);
            process::exit(1);
        }
    }

    configuration.destroy();
}
