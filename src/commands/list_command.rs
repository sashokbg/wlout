use crate::common::{AppData, HeadInfo};
use crate::head_printer::print_heads_detail;

pub fn list_command(state: AppData, verbose: bool) {
    if verbose {
        print_heads_detail(state.heads.into_values().collect::<Vec<HeadInfo>>())
    } else {
        for (i, head) in state.heads.iter().enumerate() {
            if i == state.heads.iter().len() - 1 {
                print!("{}\n", head.1.name.clone().unwrap())
            } else {
                print!("{}\t", head.1.name.clone().unwrap())
            }
        }
    }
}
