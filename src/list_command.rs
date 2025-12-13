use crate::common::AppData;
use std::collections::HashMap;
use wayland_client::Connection;

pub fn list_command() {
    let conn = Connection::connect_to_env().expect("failed to connect to a Wayland compositor");
    let display = conn.display();
    let mut event_queue = conn.new_event_queue::<AppData>();
    let qh = event_queue.handle();

    let _registry = display.get_registry(&qh, ());
    let mut state = AppData {
        initial_done: false,
        heads: HashMap::new(),
    };

    event_queue.roundtrip(&mut state).unwrap();
    while !state.initial_done {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }

    for (i, head) in state.heads.iter().enumerate() {
        if i == state.heads.iter().len() - 1 {
            print!("{}\n", head.1.name.clone().unwrap())
        } else {
            print!("{}\t", head.1.name.clone().unwrap())
        }
    }
}
