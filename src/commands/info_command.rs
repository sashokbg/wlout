use crate::common::AppData;
use crate::head_printer::print_heads_detail;

pub fn info_command(name: &str, state: AppData) {
    let target_head = state
        .heads
        .values()
        .find(|v| v.name.as_deref() == Some(name))
        .expect(&*format!("Display \"{}\" not found", name));

    print_heads_detail(vec![target_head])
}
