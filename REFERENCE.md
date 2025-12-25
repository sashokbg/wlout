# Command-Line Help for `wlout`

This document contains the help content for the `wlout` command-line program.

**Command Overview:**

* [`wlout`↴](#wlout)
* [`wlout list`↴](#wlout-list)
* [`wlout info`↴](#wlout-info)
* [`wlout power`↴](#wlout-power)
* [`wlout move`↴](#wlout-move)
* [`wlout move above`↴](#wlout-move-above)
* [`wlout move below`↴](#wlout-move-below)
* [`wlout move right-of`↴](#wlout-move-right-of)
* [`wlout move left-of`↴](#wlout-move-left-of)
* [`wlout move position`↴](#wlout-move-position)
* [`wlout mode`↴](#wlout-mode)
* [`wlout mode list`↴](#wlout-mode-list)
* [`wlout mode current`↴](#wlout-mode-current)
* [`wlout mode preferred`↴](#wlout-mode-preferred)
* [`wlout mode auto`↴](#wlout-mode-auto)
* [`wlout mode set`↴](#wlout-mode-set)
* [`wlout mirror`↴](#wlout-mirror)
* [`wlout mirror same-as`↴](#wlout-mirror-same-as)
* [`wlout completion`↴](#wlout-completion)

## `wlout`


Manage wayland compositor display outputs (heads).

This tool allows you to manage the display mode, refresh rate and position of your displays, also known as heads in Wayland protocol.
It is based on the still experimental wlr-output-management-unstable-v1 protocol and is subjected to breaking changes.

For more information please visit: https://wayland.app/protocols/wlr-output-management-unstable-v1
        

**Usage:** `wlout [COMMAND]`

###### **Subcommands:**

* `list` — List displays
* `info` — Print additional detailed information for a display
* `power` — Turn a display on or off
* `move` — Set the position of the display in the global compositor space.

Applies position optimisation after moving to ensure that one of your displays is always at the (0, 0) origin.
* `mode` — Manage the display mode resolution and refresh rate
* `mirror` — Find the highest common resolution and align two display on top of each other in order to output the same picture
* `completion` — Generate shell completion script. Dynamic completion for display names and modes is currently available for Zsh and Bash



## `wlout list`

List displays

**Usage:** `wlout list [OPTIONS]`

###### **Options:**

* `-v`, `--verbose` — Turn on verbose / detailed mode



## `wlout info`

Print additional detailed information for a display

**Usage:** `wlout info <display>`

###### **Arguments:**

* `<DISPLAY>` — The name of the display



## `wlout power`

Turn a display on or off

**Usage:** `wlout power [OPTIONS] <display> <power_mode>`

###### **Arguments:**

* `<DISPLAY>` — The name of the display
* `<POWER_MODE>`

  Possible values: `on`, `off`


###### **Options:**

* `-f`, `--force` — Do not prompt if the last display is turned off



## `wlout move`

Set the position of the display in the global compositor space.

Applies position optimisation after moving to ensure that one of your displays is always at the (0, 0) origin.

**Usage:** `wlout move <display> <COMMAND>`

###### **Subcommands:**

* `above` — Move the display above another display
* `below` — Move the display below another display
* `right-of` — Move the display right-of another display
* `left-of` — Move the display left-of another display
* `position` — Move the display to an absolute position defined by x and y coordinates on the global compositor space.
Please mind that the origin (0, 0) is top left

###### **Arguments:**

* `<DISPLAY>` — The name of the display



## `wlout move above`

Move the display above another display

**Usage:** `wlout move above <other_display>`

###### **Arguments:**

* `<OTHER_DISPLAY>` — Other display



## `wlout move below`

Move the display below another display

**Usage:** `wlout move below <other_display>`

###### **Arguments:**

* `<OTHER_DISPLAY>` — Other display



## `wlout move right-of`

Move the display right-of another display

**Usage:** `wlout move right-of <other_display>`

###### **Arguments:**

* `<OTHER_DISPLAY>` — Other display



## `wlout move left-of`

Move the display left-of another display

**Usage:** `wlout move left-of <other_display>`

###### **Arguments:**

* `<OTHER_DISPLAY>` — Other display



## `wlout move position`

Move the display to an absolute position defined by x and y coordinates on the global compositor space.
Please mind that the origin (0, 0) is top left

**Usage:** `wlout move position <x> <y>`

###### **Arguments:**

* `<X>` — x - coordinate
* `<Y>` — y - coordinate



## `wlout mode`

Manage the display mode resolution and refresh rate

**Usage:** `wlout mode <display> [COMMAND]`

###### **Subcommands:**

* `list` — List the available modes for a display
* `current` — Show the current mode for this display
* `preferred` — Show the advertised preferred mode for this display
* `auto` — Set the display mode to its preferred settings
* `set` — Set the resolution and refresh rate for the display

###### **Arguments:**

* `<DISPLAY>` — The name of the display



## `wlout mode list`

List the available modes for a display

**Usage:** `wlout mode list`

**Command Alias:** `print`



## `wlout mode current`

Show the current mode for this display

**Usage:** `wlout mode current`



## `wlout mode preferred`

Show the advertised preferred mode for this display

**Usage:** `wlout mode preferred`



## `wlout mode auto`

Set the display mode to its preferred settings

**Usage:** `wlout mode auto`



## `wlout mode set`

Set the resolution and refresh rate for the display

**Usage:** `wlout mode set [mode]`

###### **Arguments:**

* `<MODE>` — The mode format is <WIDTH>x<HEIGHT>@<RATE>



## `wlout mirror`

Find the highest common resolution and align two display on top of each other in order to output the same picture

**Usage:** `wlout mirror <display> <COMMAND>`

###### **Subcommands:**

* `same-as` — Other display

###### **Arguments:**

* `<DISPLAY>` — The name of the display



## `wlout mirror same-as`

Other display

**Usage:** `wlout mirror same-as <other_display>`

###### **Arguments:**

* `<OTHER_DISPLAY>` — The display to mirror



## `wlout completion`

Generate shell completion script. Dynamic completion for display names and modes is currently available for Zsh and Bash

**Usage:** `wlout completion <shell>`

###### **Arguments:**

* `<SHELL>`

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`




<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
