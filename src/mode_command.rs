use crate::common::AppData;
use wayland_client::EventQueue;
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_configuration_v1::ZwlrOutputConfigurationV1;

pub fn mode_command(
    name: &str,
    mut state: AppData,
    configuration: ZwlrOutputConfigurationV1,
    mut event_queue: EventQueue<AppData>,
) {
}

pub fn list_modes(name: &str, state: AppData) {
    for head in state.heads.values() {
        if head.name == Some(name.parse().unwrap()) {
            let mut modes_iter = head.modes.values();
            if let Some(first) = modes_iter.next() {
                print!("{}x{}@{:.0}", first.width, first.height, first.rate / 1000);
                for mode in modes_iter {
                    print!("\t{}x{}@{:.0}", mode.width, mode.height, mode.rate / 1000);
                }
            }

            break;
        }
    }
    println!()
}
