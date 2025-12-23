use crate::head_printer::print_heads_detail;
use crate::model::AppData;

pub fn info_command(name: &str, state: AppData) {
    let target_head = state.get_head(name);

    print_heads_detail(vec![target_head])
}
