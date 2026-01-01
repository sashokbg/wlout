use crate::backends::common::{apply, handle_result, prompt};
use crate::backends::wlroot::connect_trait::WaylandCommand;
use crate::commands::commands::Executable;
use crate::commands::commands::{
    ModeAutoCommand, ModeCurrentCommand, ModeListCommand, ModePreferredCommand, ModeSetCommand,
};
use crate::model::{AppData, ConfigResult, HeadMode};
use std::process::exit;

impl WaylandCommand for ModeCurrentCommand {}
impl WaylandCommand for ModeAutoCommand {}
impl WaylandCommand for ModePreferredCommand {}
impl WaylandCommand for ModeSetCommand {}
impl WaylandCommand for ModeListCommand {}

impl Executable for ModeCurrentCommand {
    fn execute(&self) {
        let (_, state) = self.connect();
        let target_head = state.get_head(self.name.as_str());

        let mode = target_head.get_current_mode().expect(&*format!(
            "No current mode not found on display {}. It is probably off",
            self.name
        ));

        let string_result = format!("{}x{}@{:.0}", mode.width, mode.height, mode.rate);
        println!("{}", string_result)
    }
}

impl Executable for ModeAutoCommand {
    fn execute(&self) {
        let (mut event_queue, mut state) = self.connect();
        let target_head = state.get_head(self.name.as_str());

        let mode = get_preferred_mode(self.name.as_str(), &state);

        let result = apply(&mut state, &mut event_queue, |config, qh| {
            let head_config = config.enable_head(&target_head.head, qh, ());
            head_config.set_mode(&mode.mode.clone().unwrap());
        });

        let success_message = &format!("Auto set mode {} for display {}", mode, self.name);
        let failure_message = &format!("Failed to set mode {} for display {}", mode, self.name);

        handle_result(result, success_message, failure_message);
    }
}

impl Executable for ModePreferredCommand {
    fn execute(&self) {
        let (_, state) = self.connect();
        let mode = get_preferred_mode(self.name.as_str(), &state);

        let string_result = format!("{}x{}@{:.0}", mode.width, mode.height, mode.rate);
        println!("{}", string_result)
    }
}

impl Executable for ModeSetCommand {
    fn execute(&self) {
        let (mut event_queue, mut state) = self.connect();

        let target_head = state.get_head(self.name.as_str());

        let target_mode = target_head.find_mode(self.mode.width, self.mode.height, self.mode.rate);

        let result: ConfigResult;

        if target_mode.is_none() {
            if !self.force {
                let prompt_msg = format!(
                    "The specified mode {} does not exist for display {}. Set it as custom mode for this display ?",
                    self.mode, self.name
                );
                let read = prompt(&prompt_msg);

                if read.to_lowercase() != "y" {
                    exit(1)
                }
            }
            result = apply(&mut state, &mut event_queue, |config, qh| {
                let head_config = config.enable_head(&target_head.head, qh, ());
                head_config.set_custom_mode(self.mode.width, self.mode.height, self.mode.rate);
            });
        } else {
            result = apply(&mut state, &mut event_queue, |config, qh| {
                let head_config = config.enable_head(&target_head.head, qh, ());
                head_config.set_mode(&target_mode.unwrap().mode.clone().unwrap());
            });
        }
        let success_message = &format!("Set mode {} for display {}", self.mode, self.name);
        let failure_message =
            &format!("Failed to set mode {} for display {}", self.mode, self.name);

        handle_result(result, success_message, failure_message);
    }
}

impl Executable for ModeListCommand {
    fn execute(&self) {
        let (_, state) = self.connect();
        for head in state.heads.values() {
            if head.name == Some(self.name.parse().unwrap()) {
                let mut modes: Vec<_> = head.modes.values().collect();
                modes.sort_by(|a, b| {
                    b.height
                        .cmp(&a.height)
                        .then(b.width.cmp(&a.width))
                        .then(b.rate.cmp(&a.rate))
                });

                for (i, mode) in modes.iter().enumerate() {
                    let mut string_result =
                        format!("{}x{}@{:.0}", mode.width, mode.height, mode.rate);

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
}

fn get_preferred_mode(name: &str, state: &AppData) -> HeadMode {
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
