use crate::commands::common::{apply, handle_result};
use crate::model::{AppData, HeadMode, HeadModeInput};
use wayland_client::EventQueue;

pub fn mode_current_command(name: &str, state: AppData) {
    let target_head = state
        .get_head(name);

    let mode = target_head
        .get_current_mode()
        .expect(&*format!(
            "No current mode not found on display {}. It is probably off",
            name
        ));

    let string_result = format!("{}x{}@{:.0}", mode.width, mode.height, mode.rate);
    println!("{}", string_result)
}

pub fn mode_auto_command(name: &str, state: &mut AppData, event_queue: &mut EventQueue<AppData>) {
    let target_head = state.get_head(name);

    let mode = _get_preferred_mode(name, state);

    let result = apply(state, event_queue, |config, qh| {
        let head_config = config.enable_head(&target_head.head, qh, ());
        head_config.set_mode(&mode.mode.clone().unwrap());
    });

    let success_message = &format!("Auto set mode {} for display {}", mode, name);
    let failure_message = &format!("Failed to set mode {} for display {}", mode, name);

    handle_result(result, success_message, failure_message);
}

pub fn mode_preferred_command(name: &str, state: &AppData) {
    let mode = _get_preferred_mode(name, state);

    let string_result = format!("{}x{}@{:.0}", mode.width, mode.height, mode.rate);
    println!("{}", string_result)
}

fn _get_preferred_mode(name: &str, state: &AppData) -> HeadMode {
    let target_head = state.get_head(name);

    let mode = target_head
        .modes
        .values()
        .find(|m| m.is_preferred)
        .expect(&*format!(
            "No preferred mode not found on display {}.",
            name
        ));
    mode.clone()
}

pub fn mode_set_command(
    name: &str,
    mode: &HeadModeInput,
    mut state: AppData,
    mut event_queue: EventQueue<AppData>,
) {
    let target_head = state.get_head(name);

    let target_mode = target_head
        .find_mode(mode.width, mode.height, mode.rate)
        .expect(&format!("Mode {} not found on display {}", mode, name));

    let result = apply(&mut state, &mut event_queue, |config, qh| {
        let head_config = config.enable_head(&target_head.head, qh, ());
        head_config.set_mode(&target_mode.mode.clone().unwrap());
    });
    let success_message = &format!("Set mode {} for display {}", mode, name);
    let failure_message = &format!("Failed to set mode {} for display {}", mode, name);

    handle_result(result, success_message, failure_message);
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
                let mut string_result = format!("{}x{}@{:.0}", mode.width, mode.height, mode.rate);

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
