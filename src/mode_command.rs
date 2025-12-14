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
            let mut modes: Vec<_> = head.modes.values().collect();
            modes.sort_by(|a, b| {
                b.height.cmp(&a.height)
                    .then(b.width.cmp(&a.width))
                    .then(b.rate.cmp(&a.rate))
            });

            for (i, mode) in modes.iter().enumerate() {
                if (i == modes.len() - 1) {
                    print!("{}x{}@{:.0}\n", mode.width, mode.height, mode.rate / 1000);
                } else {
                    print!("{}x{}@{:.0}\t", mode.width, mode.height, mode.rate / 1000);
                }
            }

            break;
        }
    }
}
