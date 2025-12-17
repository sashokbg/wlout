# Wayland Outputs Command Line Tool - wlout

This CLI tool's job is to manage displays (heads) on Wayland compositors that use the Wayland Protocols.   
It uses the still unstable "wlr_output_management_unstable_v1" so things are subjected to changes at any time.

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

- The tool's goal is to be a successor of the popular `xrandr` (tool for x11) and `wlr-randr` tools, but with some modernization.
- Autocomplete is a first-class citizen
- Quality of life features like verifications when turning off last display etc

Unix philosophy
- Doing only one thing per command
- Parsable outputs
- Standard cli shape, no flags soup
- Self-explorable
- Self-documented

![video](./video.gif)

## Usage

- General use: `wlout <verb> <display> <option>`

## Examples

- List your displays: `wlout list`
- Turn off/on a display: `wlout power HDMI-A-1 <off|on>`
- List resolution and refresh rates for a display: `wlout mode HDMI-A-1 list`
- Set resolution and refresh rate for a display: `wlout mode HDMI-A-1 set 1920x1080@60`
- Move a display to an absolute position: `wlout move HDMI-A-1 absolute 0 0`

## Commands Reference

| Command Name      | Syntax                                       | Description                                                               | Implemented |
|-------------------|----------------------------------------------|---------------------------------------------------------------------------|-------------|
| Default           | `wlout`                                      | Prints help                                                               | Y           |
| List all displays | `wlout list`                                 | Prints all available displays                                             | Y           |
| Turn on           | `wlout power <display> on`                   | Turns on the display with last config                                     | Y           |
| Turn off          | `wlout power <display> off`                  | Turns off the display                                                     | Y           |
| Display Info      | `wlout info <display>`                       | Prints additional info about the display (serial, manufacturer, etc.)     | Y           |
| Mode list         | `wlout mode <display> list`                  | Lists available modes for this display                                    | Y           |
| Mode list         | `wlout mode <display> current`               | Get the current display mode for this display                             | Y           |
| Mode list         | `wlout mode <display> preferred`             | Get the preferred advertised display mode for this display                | Y           |
| Mode set          | `wlout mode <display> set <mode>`            | Sets the current mode                                                     | Y           |
| Move left         | `wlout move <display> left <other display>`  | Moves the display to the left of another display                          |             |
| Move right        | `wlout move <display> right <other display>` | Moves the display to the right of another display                         |             |
| Move above        | `wlout move <display> above <other display>` | Moves the display to the above of another display                         |             |
| Move below        | `wlout move <display> below <other display>` | Moves the display to the below of another display                         |             |
| Move position     | `wlout move <display> absolute <x> <y>`      | Moves the display to a defined position on the virtual desktop            | Y           |
| Mirror            | `wlout clone <display> <other display>`      | Clones the display output of another display. Requires compositor support |             |
| Mirror            | `wlout align <display> <other display>`      | Align the display output with the (0, 0) origin of another display        |             |

