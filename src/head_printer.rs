use crate::common::HeadInfo;
use std::io::Write;
use tabwriter::TabWriter;

pub fn print_heads_detail(heads: Vec<&HeadInfo>) {
    let mut tw = TabWriter::new(vec![]);

    for head in heads {
        let phys_size_str;

        if head.physical_width.is_none() || head.physical_height.is_none() {
            phys_size_str = String::from("N/A");
        } else {
            phys_size_str = format!(
                "{}x{}",
                head.physical_width.unwrap(),
                head.physical_height.unwrap()
            )
        }

        let position_str;
        if head.position_x.is_none() || head.position_y.is_none() {
            position_str = String::from("N/A")
        } else {
            position_str = format!(
                "({},{})",
                head.position_x.unwrap(),
                head.position_y.unwrap()
            )
        }

        write!(
            &mut tw,
            "{}",
            format!(
                "
Name\tEnabled\tMake\tModel\tPhysical Size\tPosition
{}\t{}\t{}\t{}\t{}\t{}
    ",
                head.name.clone().unwrap(),
                head.enabled,
                head.make.clone().or(Some(String::from("N/A"))).unwrap(),
                head.model.clone().or(Some(String::from("N/A"))).unwrap(),
                phys_size_str,
                position_str
            )
        )
        .unwrap();

        tw.flush().unwrap();
    }

    let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
    println!("{}", written);
}
