**wlout**:
```
Wayland (Wl Roots) Display Manager CLI with UNIX philosophy and modern goodies
```
        **list**: 
```
List displays
```
        **info**: 
```
Print additional detailed information for a display
```
        **power**: 
```
Turn a display on or off
```
        **move**: 
```
Set the position of the display in the global compositor space.

Applies position optimisation after moving to ensure that one of your displays is always at the (0, 0) origin.
```
                **above**: 
```
Move the display above another display
```
                **below**: 
```
Move the display below another display
```
                **right-of**: 
```
Move the display right-of another display
```
                **left-of**: 
```
Move the display left-of another display
```
                **position**: 
```
Move the display to an absolute position defined by x and y coordinates on the global compositor space.
Please mind that the origin (0, 0) is top left
```
        **mode**: 
```
Manage the display mode resolution and refresh rate
```
                **list**: 
```
List the available modes for a display
```
                **current**: 
```
Show the current mode for this display
```
                **preferred**: 
```
Show the advertised preferred mode for this display
```
                **auto**: 
```
Set the display mode to its preferred settings
```
                **set**: 
```
Set the resolution and refresh rate for the display
```
        **mirror**: 
```
Find the highest common resolution and align two display on top of each other in order to output the same picture
```
                **same-as**: 
```
Other display
```
        **completion**: 
```
Generate shell completion script. Dynamic completion for display names and modes is currently available for Zsh and Bash
```

