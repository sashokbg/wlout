use crate::model::{AppData, ConfigResult};
use std::io::Write;
use std::process;
use wayland_client::{EventQueue, QueueHandle};
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_configuration_v1::ZwlrOutputConfigurationV1;

pub fn apply(
    state: &mut AppData,
    event_queue: &mut EventQueue<AppData>,
    configure: impl FnOnce(&ZwlrOutputConfigurationV1, &QueueHandle<AppData>),
) -> ConfigResult {
    let qh = event_queue.handle();
    let manager = state.manager.as_ref().expect("output manager not bound");
    let serial: u32 = state.config_serial.unwrap();
    let configuration = manager.create_configuration(serial, &qh, ());

    state.config_result = None;
    configure(&configuration, &qh);
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

pub fn prompt(text: &str) -> String {
    print!("{} ", text);
    std::io::stdout().flush().expect("Oups, stdout error");

    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to get input");

    response.trim_end().to_string()
}
