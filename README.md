Usage

| Command Name      | Syntax                                       | Description                                                               | Impl |
|-------------------|----------------------------------------------|---------------------------------------------------------------------------|------|
| Default           | `wlout`                                      | Prints help                                                               | Y    |
| List all displays | `wlout list`                                 | Prints all available displays                                             | Y    |
| Turn on           | `wlout power <display> on`                   | Turns on the display with last config                                     | Y    |
| Turn off          | `wlout power <display> off`                  | Turns off the display                                                     | Y    |
| Display Info      | `wlout info <display>`                       | Prints additional info about the display (serial, manufacturer, etc.)     |      |
| Mode list         | `wlout mode <display> list`                  | Lists available modes for this display                                    | Y    |
| Mode list         | `wlout mode <display> current`               | Get the current display mode for this display                             | Y    |
| Mode set          | `wlout mode <display> set <mode>`            | Sets the current mode                                                     | Y    |
| Move left         | `wlout move <display> left <other display>`  | Moves the display to the left of another display                          |      |
| Move right        | `wlout move <display> right <other display>` | Moves the display to the right of another display                         |      |
| Move above        | `wlout move <display> above <other display>` | Moves the display to the above of another display                         |      |
| Move below        | `wlout move <display> below <other display>` | Moves the display to the below of another display                         |      |
| Move position     | `wlout move <display> absolute <x> <y>`      | Moves the display to a defined position                                   |      |
| Mirror            | `wlout clone <display> <other display>`      | Clones the display output of another display. Requires compositor support |      |
| Mirror            | `wlout align <display> <other display>`      | Align the display output with the (0, 0) origin of another display        |      |

