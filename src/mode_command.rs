use crate::common::AppData;
use wayland_client::EventQueue;
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_configuration_v1::ZwlrOutputConfigurationV1;

pub fn mode_command(
    name: &str,
    mode: &String,
    mut state: AppData,
    configuration: &ZwlrOutputConfigurationV1,
    mut event_queue: EventQueue<AppData>,
) {
    println!("SETTING MODE {} for display {}", mode, name)
}

pub fn list_modes(name: &str, state: AppData) {
    for head in state.heads.values() {
        if head.name == Some(name.parse().unwrap()) {
            let mut modes: Vec<_> = head.modes.values().collect();
            modes.sort_by(|a, b| {
                b.height
                    .cmp(&a.height)
                    .then(b.width.cmp(&a.width))
                    .then(b.rate.cmp(&a.rate))
            });

            for (i, mode) in modes.iter().enumerate() {
                let mut string_result =
                    format!("{}x{}@{:.0}", mode.width, mode.height, mode.rate / 1000);

                if(mode.is_current || mode.is_preferred) {
                    string_result += "(";
                    if mode.is_preferred {
                        string_result += "preferred"
                    }
                    if mode.is_current {
                        if(mode.is_preferred) {
                            string_result += ","
                        }
                        string_result += "current"
                    }
                    string_result += ")"
                }

                if (i == modes.len() - 1) {
                    string_result += "\n"
                } else {
                    string_result += "\t"
                }
                print!("{}", string_result)
            }
        }
    }
}
