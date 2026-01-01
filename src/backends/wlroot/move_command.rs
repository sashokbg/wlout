use crate::backends::common::{apply, handle_result};
use crate::backends::wlroot::connect_trait::WaylandCommand;
use crate::commands::commands::Executable;
use crate::commands::move_command::{
    MoveCommand, MoveRelativeCommand, REL_POS_ABOVE, REL_POS_BELOW, REL_POS_LEFT_OF,
    REL_POS_RIGHT_OF,
};

impl WaylandCommand for MoveRelativeCommand {}
impl WaylandCommand for MoveCommand {}

impl Executable for MoveRelativeCommand {
    fn execute(&self) {
        let (mut event_queue, mut state) = self.connect();

        let moved_display_name = &self.moved_display_name;
        let reference_display_name = &self.reference_display_name;

        let (moved_display_mode, moved_display_info, reference_display_mode, reference_display_info) =
            {
                let moved_display_info = state.get_head(moved_display_name);

                let reference_display_info = state.get_head(reference_display_name);

                let moved_display_mode = moved_display_info
                    .get_current_mode()
                    .expect("The display has no current mode set. Is it switched on ?");

                let reference_display_mode = reference_display_info
                    .get_current_mode()
                    .expect("The display has no current mode set. Is it switched on ?");
                (
                    moved_display_mode.clone(),
                    moved_display_info.clone(),
                    reference_display_mode.clone(),
                    reference_display_info.clone(),
                )
            };

        let result = apply(&mut state, &mut event_queue, |config, qh| {
            let moved_display_config = config.enable_head(&moved_display_info.head, &qh, ());

            match self.pos.as_str() {
                REL_POS_ABOVE => {
                    moved_display_config.set_position(
                        reference_display_info.position_x.unwrap(),
                        reference_display_info.position_y.unwrap() - moved_display_mode.height,
                    );
                }
                REL_POS_BELOW => {
                    moved_display_config.set_position(
                        reference_display_info.position_x.unwrap(),
                        reference_display_info.position_y.unwrap() + reference_display_mode.height,
                    );
                }
                REL_POS_RIGHT_OF => {
                    moved_display_config.set_position(
                        reference_display_info.position_x.unwrap() + reference_display_mode.width,
                        reference_display_info.position_y.unwrap(),
                    );
                }
                REL_POS_LEFT_OF => {
                    moved_display_config.set_position(
                        reference_display_info.position_x.unwrap() - moved_display_mode.width,
                        reference_display_info.position_y.unwrap(),
                    );
                }
                &_ => todo!(),
            }
        });

        let success = format!(
            "Moved display {moved_display_name} {} {reference_display_name}",
            self.pos
        );
        let fail = format!(
            "Unable to move display {moved_display_name} {} {reference_display_name}",
            self.pos
        );
        handle_result(result, &success, &fail)
    }
}

impl Executable for MoveCommand {
    fn execute(&self) {
        let (mut event_queue, mut state) = self.connect();
        let target_head = state.get_head(self.name.as_str()).head.clone();

        let config_result = apply(&mut state, &mut event_queue, |configuration, qh| {
            let head_config = configuration.enable_head(&target_head, &qh, ());
            head_config.set_position(self.x, self.y);
        });

        let success_message = &format!(
            "Set position for display {} to x: {} y: {}",
            self.name, self.x, self.y
        );
        let error_message = &format!("Failed to set position for display {}", self.name);

        handle_result(config_result, success_message, error_message);
    }
}
