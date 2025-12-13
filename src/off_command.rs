/*
* This method subscribes to the Global events. The global events advertise the capabilities of the system.
 * Once we encounter event for interface of type "output_manager" we bind to it
*/
use crate::common::AppData;
use wayland_client::Connection;

pub fn off_command(name: &String) {
    let conn = Connection::connect_to_env().expect("failed to connect to a Wayland compositor");
    let display = conn.display();
    let mut event_queue = conn.new_event_queue::<AppData>();
    let qh = event_queue.handle();
    let _registry = display.get_registry(&qh, ());
}
