use crate::commands::common::{apply, handle_result};
use crate::model::AppData;
use wayland_client::EventQueue;

pub fn mirror_command(
    mirrored_display_name: &String,
    reference_display_name: &String,
    state: &mut AppData,
    mut event_queue: EventQueue<AppData>,
) {
    let (_, moved_display_info, _, reference_display_info) = {
        let moved_display_info = state
            .get_head(mirrored_display_name);

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

    let result = apply(state, &mut event_queue, |config, qh| {
        let moved_display_config = config.enable_head(&moved_display_info.head, &qh, ());
        moved_display_config.set_position(
            reference_display_info.position_x.unwrap(),
            reference_display_info.position_y.unwrap(),
        );
    });

    let success =
        format!("Mirrored display {mirrored_display_name} same-as {reference_display_name}");
    let fail =
        format!("Unable to mirror display {mirrored_display_name} as {reference_display_name}");
    handle_result(result, &success, &fail)
}
