# Wlout - Wayland Outputs Command Line Tool

This CLI tool's job is to manage displays (heads) on Wayland compositors that use the Wayland Protocols.   
It uses the still experimental `wlr_output_management_unstable_v1` so things are subjected to changes at any time.

Compatible with sway, hyprland and should work with other compositors using wlroots:  
https://github.com/solarkraft/awesome-wlroots?tab=readme-ov-file#compositors

It is written in Rust and depends on the following packages:

- wayland-client
- wayland-protocols-wlr
- clap
- clap_complete

For more information visit the official wayland protocols repository.  
https://gitlab.freedesktop.org/wayland/wayland-protocols

## Philosophy & Features

- The tool's goal is to be a successor of the popular `xrandr` (tool for x11) and `wlr-randr` tools, but with a modern
  twist.
- Autocomplete is a first-class citizen
- Quality of life features like verifications when turning off last display etc

Unix philosophy

- Doing only one thing per command
- Standard parsable outputs
- Standard cli shape, no flags soup
- Self-explorable
- Self-documented

![video](./video.gif)

## Installation

- Arch based distributions

  Using your favorite AUR helper https://aur.archlinux.org/packages/wlout

  ```
  yay -Sy wlout
  ```

## Usage

- General use: `wlout <verb> <display> <option>`

## Examples

- List your displays: `wlout list`
- Turn off/on a display: `wlout power HDMI-A-1 <off|on>`
- List resolution and refresh rates for a display: `wlout mode HDMI-A-1 list`
- Set resolution and refresh rate for a display: `wlout mode HDMI-A-1 set 1920x1080@60`
- Move a display to an absolute position: `wlout move HDMI-A-1 position 0 0`

## Commands Reference

| Command Name      | Syntax                                           | Description                                                                |
|-------------------|--------------------------------------------------|----------------------------------------------------------------------------|
| Default           | `wlout`                                          | Prints help                                                                |
| List all displays | `wlout list`                                     | Prints all available displays                                              |
| Turn on           | `wlout power <display> on`                       | Turns on the display with last config                                      |
| Turn off          | `wlout power <display> off`                      | Turns off the display                                                      |
| Display Info      | `wlout info <display>`                           | Prints additional info about the display (serial, manufacturer, etc.)      |
| Mode list         | `wlout mode <display> list`                      | Lists available modes for this display                                     |
| Mode current      | `wlout mode <display> current`                   | Get the current display mode for this display                              |
| Mode preferred    | `wlout mode <display> preferred`                 | Get the preferred advertised display mode for this display                 |
| Mode auto         | `wlout mode <display> auto`                      | Automatically set the mode of the display to its advertised preferred mode |
| Mode set          | `wlout mode <display> set <mode>`                | Sets the current mode                                                      |
| Move left         | `wlout move <display> left <other display>`      | Moves the display to the left of another display                           |
| Move right        | `wlout move <display> right <other display>`     | Moves the display to the right of another display                          |
| Move above        | `wlout move <display> above <other display>`     | Moves the display to the above of another display                          |
| Move below        | `wlout move <display> below <other display>`     | Moves the display to the below of another display                          |
| Move position     | `wlout move <display> position <x> <y>`          | Moves the display to a defined position on the virtual desktop             |
| Mirror            | `wlout mirror <display> same-as <other display>` | Clones the display output of another display. Requires compositor support  |

## Shell Completion

Add `source <(wlout completion <shell_name>)` in your shell's rc profile file.

Where <shell_name> is one of:

- zsh
- bash
- fish (partially implemented)
- elvish (partially implemented)

## Building

Manually

```
cargo build
install target/debug/wlout ~/.local/bin
```

Or using Taskfile

`task install`

## Roadmap

- [x] Add dynamic shell completions for bash
- [ ] Add dynamic shell completions for fish
- [ ] Detect unsupported environments and show warning
- [x] Add GPLv3 license
- [ ] Add case-insensitive dynamic parameters support (ex edp <tab> yielding eDP-1)
- [ ] Add dbus KWin support for KDE
- [x] Prevent switching off last display
- [x] Use preferred screen mode when duplicating
- [ ] Allow to not use "best common resolution" when mirroring via a flag
- [x] Add mode <display> auto command that uses the screen preferred mode
- [ ] Optimize positions default screen by keeping a virtual map of all screens
- [x] Add to AUR
- [ ] Add adaptive sync options
- [ ] Add "set preferred mode"
- [ ] Add some unit tests

## AI Disclaimer

The project is 95% written by hand, but an LLM assistant has been used for the dynamic shell completion functions due to
their hard (for me) syntax.
