use clap::Command;
use sailfish::TemplateSimple;
use std::fmt::Write;
use wlout::cli::build_cli;

#[derive(TemplateSimple)]
#[template(path = "readme.stpl")]
struct CommandTemplate {
    command_tree: String,
}

fn visit(command: &Command, indent: usize, result: &mut String) {
    let indent_str = "\t".repeat(indent);

    writeln!(
        result,
        "{}{}",
        indent_str,
        command
            .clone()
            .render_usage()
            .to_string()
            .replace(" [OPTIONS]", "")
            .replace("Usage: ", "")
    )
    .unwrap();

    command.get_subcommands().for_each(|sub_com| {
        visit(sub_com, indent + 1, result);
    });
}

fn main() {
    use std::io::{stdout, Write};
    let command = build_cli();
    let mut result = String::new();
    visit(&command, 0, &mut result);

    let ctx = CommandTemplate {
        command_tree: result,
    };

    stdout()
        .write_all(ctx.render_once().unwrap().as_bytes())
        .expect("Unable to write README.md");
}
