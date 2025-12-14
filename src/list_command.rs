use crate::common::AppData;

pub fn list_command(state: AppData) {
    for (i, head) in state.heads.iter().enumerate() {
        if i == state.heads.iter().len() - 1 {
            print!("{}\n", head.1.name.clone().unwrap())
        } else {
            print!("{}\t", head.1.name.clone().unwrap())
        }
    }
}
