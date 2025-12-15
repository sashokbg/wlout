use std::process;
use crate::common::{AppData, ConfigResult, HeadModeInput};
use wayland_client::EventQueue;
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_configuration_head_v1::ZwlrOutputConfigurationHeadV1;
use wayland_protocols_wlr::output_management::v1::client::zwlr_output_configuration_v1::ZwlrOutputConfigurationV1;

pub fn mode_set_command(
    name: &str,
    mode: &HeadModeInput,
    mut state: AppData,
    configuration: ZwlrOutputConfigurationV1,
    mut event_queue: EventQueue<AppData>,
) {
    let target_head = state
        .heads
        .values()
        .find(|head| head.name.as_deref() == Some(name))
        .expect(&*format!("Display \"{}\" not found", name));

    let target_mode = target_head
        .modes
        .values()
        .find(|m| m.width == mode.width && m.height == mode.height && m.rate == mode.rate)
        .expect(&*format!("Mode {} not found on display {}", mode, name));

    let qh = event_queue.handle();
    let head_config: ZwlrOutputConfigurationHeadV1 =
        configuration.enable_head(&target_head.head, &qh, ());
    head_config.set_mode(&target_mode.mode);
    state.config_result = None;
    configuration.apply();

    while state.config_result.is_none() {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }

    match state.config_result.unwrap() {
        ConfigResult::Succeeded => {
            println!("Set mode {} for display {}", mode, name)
        }
        ConfigResult::Failed => {
            eprintln!("Failed to set mode {} for display {}", mode, name);
            process::exit(1);
        }
        ConfigResult::Cancelled => {
            eprintln!(
                "Configuration cancelled before setting mode {} for display {}",
                mode, name
            );
            process::exit(1);
        }
    }

    configuration.destroy();
}

pub fn mode_list_command(name: &str, state: AppData) {
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

                if mode.is_current || mode.is_preferred {
                    string_result += "(";
                    if mode.is_preferred {
                        string_result += "preferred"
                    }
                    if mode.is_current {
                        if mode.is_preferred {
                            string_result += ","
                        }
                        string_result += "current"
                    }
                    string_result += ")"
                }

                if i == modes.len() - 1 {
                    string_result += "\n"
                } else {
                    string_result += "\t"
                }
                print!("{}", string_result)
            }
        }
    }
}
