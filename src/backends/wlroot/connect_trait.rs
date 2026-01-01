use crate::handles::OUTPUT_MANAGER_INTERFACE_NAME;
use crate::model::AppData;
use std::collections::HashMap;
use std::process::exit;
use wayland_client::{Connection, EventQueue};

pub trait WaylandCommand {
    fn connect(&self) -> (EventQueue<AppData>, AppData) {
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
}
