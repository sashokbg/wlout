use crate::common::{AppData, ConfigResult};
use std::process;
use wayland_client::EventQueue;

pub fn off_command(name: &str, mut state: AppData, mut event_queue: EventQueue<AppData>) {
    let target_head = state
        .heads
        .values()
        .find(|head| head.name.as_deref() == Some(name))
        .expect("requested head not found");

    let manager = state.manager.as_ref().expect("output manager not bound");
    let serial: u32 = state.config_serial.unwrap();
    let qh = event_queue.handle();
    let configuration = manager.create_configuration(serial, &qh, ());
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
