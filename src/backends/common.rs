use crate::model::{AppData, ConfigResult, HeadInfo};
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

    let first_result = state.config_result.unwrap();
    configuration.destroy();

    reposition_displays_to_origin(state, event_queue, &qh);

    first_result
}

fn reposition_displays_to_origin(
    state: &mut AppData,
    event_queue: &mut EventQueue<AppData>,
    qh: &QueueHandle<AppData>,
) {
    event_queue.roundtrip(state).unwrap();

    let enabled_heads: Vec<&HeadInfo> = state
        .heads
        .values()
        .filter(|head| head.enabled && head.position_x.is_some() && head.position_y.is_some())
        .collect();

    if let Some(closest_to_origin) = enabled_heads
        .iter()
        .copied()
        .min_by_key(|head| head.distance_to_origin())
    {
        if closest_to_origin.position_x.unwrap() != 0 || closest_to_origin.position_y.unwrap() != 0
        {
            let manager = state.manager.as_ref().expect("output manager not bound");
            let serial: u32 = state.config_serial.unwrap();
            let configuration = manager.create_configuration(serial, &qh, ());

            state.config_result = None;
            for head in enabled_heads {
                let head_config = configuration.enable_head(&head.head, &qh, ());
                let x = head.position_x.unwrap() - closest_to_origin.position_x.unwrap();
                let y = head.position_y.unwrap() - closest_to_origin.position_y.unwrap();
                head_config.set_position(x, y);
            }

            configuration.apply();
            while state.config_result.is_none() {
                event_queue.blocking_dispatch(state).unwrap();
            }
            configuration.destroy();
        }
    }
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
