use std::process;
use wayland_client::EventQueue;
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_configuration_v1::ZwlrOutputConfigurationV1;
use crate::common::{AppData, ConfigResult};

pub fn apply(
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

pub fn handle_result(config_result: ConfigResult, success: &str, fail: &str) {
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
