use crate::parsers::DisplayModeParser;
use clap::{value_parser, Arg, ArgAction, Command};
use clap_complete::aot::Shell;

pub static NAME_ARG_ID: &str = "display";

pub fn build_cli() -> Command {
    let display_arg = Arg::new(NAME_ARG_ID)
        .required(true)
        .help("The name of the display");

    Command::new("wlout")
        .version("1.0")
        .about("Manage wayland compositor display outputs (heads).")
        .long_about("
Manage wayland compositor display outputs (heads).

This tool allows you to manage the display mode, refresh rate and position of your displays, also known as heads in Wayland protocol.
It is based on the still experimental wlr-output-management-unstable-v1 protocol and is subjected to breaking changes.

For more information please visit: https://wayland.app/protocols/wlr-output-management-unstable-v1
        ")
        .arg_required_else_help(true)
        .subcommand(Command::new("list")
            .arg(Arg::new("verbose")
                     .help("Turn on verbose / detailed mode")
                     .long("verbose")
                     .short('v')
                     .action(ArgAction::SetTrue),
            )
            .about("List displays"))
        .subcommand(Command::new("info")
            .about("Print additional detailed information for a display")
            .arg_required_else_help(true)
            .arg(display_arg.clone())
        )
        .subcommand(
            Command::new("power")
                .about("Turn a display on or off")
                .arg_required_else_help(true)
                .arg(display_arg.clone())
                .arg(Arg::new("power_mode")
                         .required(true)
                         .action(ArgAction::Set)
                         .value_parser(clap::builder::PossibleValuesParser::new(
                             ["on", "off"]
                         ),),
                )
                .arg(Arg::new("force")
                    .short('f')
                    .long("force")
                    .help("Do not prompt if the last display is turned off")
                    .action(ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("move")
                .about("Set the position of the display in the global compositor space")
                .subcommand_required(true)
                .arg(display_arg.clone())
                .subcommand(
                    Command::new("above")
                        .about("Move the display above another display")
                        .arg(
                            Arg::new("other_display")
                                .required(true)
                                .help("Other display")
                        )
                )
                .subcommand(
                    Command::new("below")
                        .about("Move the display below another display")
                        .arg(
                            Arg::new("other_display")
                                .required(true)
                                .help("Other display")
                        )
                )
                .subcommand(
                    Command::new("right-of")
                        .about("Move the display right-of another display")
                        .arg(
                            Arg::new("other_display")
                                .required(true)
                                .help("Other display")
                        )
                )
                .subcommand(
                    Command::new("left-of")
                        .about("Move the display left-of another display")
                        .arg(
                            Arg::new("other_display")
                                .required(true)
                                .help("Other display")
                        )
                )
                .subcommand(
                    Command::new("position")
                        .about("Move the display to an absolute position defined by x and y coordinates on the global compositor space.\nPlease mind that the origin (0, 0) is top left")
                        .arg(
                            Arg::new("x")
                                .required(true)
                                .help("x - coordinate")
                                .value_parser(
                                    value_parser!(i32)
                                )
                        )
                        .arg(
                            Arg::new("y")
                                .required(true)
                                .help("y - coordinate")
                                .value_parser(
                                    value_parser!(i32)
                                )
                        )
                )

        )
        .subcommand(
            Command::new("mode")
                .about("Manage the display mode resolution and refresh rate")
                .arg(display_arg.clone())
                .subcommand(Command::new("list")
                    .about("List the available modes for a display"))
                .subcommand(Command::new("current")
                    .about("Show the current mode for this display"))
                .subcommand(Command::new("preferred")
                    .about("Show the advertised preferred mode for this display"))
                .subcommand(Command::new("auto")
                    .about("Set the display mode to its preferred settings"))
                .subcommand(Command::new("set")
                    .arg_required_else_help(true)
                    .about("Set the resolution and refresh rate for the display")
                    .arg(
                        Arg::new("mode")
                            .help("The mode format is <WIDTH>x<HEIGHT>@<RATE>")
                            .value_parser(DisplayModeParser {}),
                    )
                )
        )
        .subcommand(
            Command::new("mirror")
                .about("Find the highest common resolution and align two display on top of each other in order to output the same picture")
                .subcommand_required(true)
                .arg(display_arg.clone())
                .subcommand(
                    Command::new("same-as")
                        .about("Other display")
                        .arg(
                            Arg::new("other_display")
                                .required(true)
                                .help("The display to mirror")
                        )
                )
        )
        .subcommand(
            Command::new("completion")
                .about("Generate shell completion script")
                .arg(
                    Arg::new("shell")
                        .required(true)
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(Shell)),
                ),
        )
}
