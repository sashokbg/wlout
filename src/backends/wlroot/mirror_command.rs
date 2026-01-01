use crate::backends::common::{apply, handle_result};
use crate::backends::wlroot::connect_trait::WaylandCommand;
use crate::commands::commands::{Executable, MirrorCommand};
use crate::model::{get_best_display_modes, get_common_modes};

impl WaylandCommand for MirrorCommand {}

impl Executable for MirrorCommand {
    fn execute(&self) {
        let mirrored_display_name = &self.mirrored_display_name;
        let reference_display_name = &self.reference_display_name;

        let (mut event_queue, mut state) = self.connect();

        let (_, moved_display_info, _, reference_display_info) = {
            let moved_display_info = state.get_head(mirrored_display_name);

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

        let (modes_1, modes_2) = get_common_modes(&moved_display_info, &reference_display_info);
        let (best_mode_1, best_mode_2) = get_best_display_modes(modes_1, modes_2);

        let result = apply(&mut state, &mut event_queue, |config, qh| {
            let moved_display_config = config.enable_head(&moved_display_info.head, &qh, ());
            let ref_display_config = config.enable_head(&reference_display_info.head, &qh, ());

            moved_display_config.set_position(
                reference_display_info.position_x.unwrap(),
                reference_display_info.position_y.unwrap(),
            );
            moved_display_config.set_mode(&best_mode_1.clone().mode.unwrap());
            ref_display_config.set_mode(&best_mode_2.clone().mode.unwrap());
        });

        let success =
            format!("Mirrored display {mirrored_display_name}({best_mode_1}) same-as {reference_display_name}({best_mode_2}).

Using {best_mode_1} and {best_mode_2} as best common resolution.");
        let fail =
            format!("Unable to mirror display {mirrored_display_name} as {reference_display_name}");
        handle_result(result, &success, &fail)
    }
}
