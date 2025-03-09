#!/bin/bash

killall waybar | echo "No waybar instances to kill"

# Change preset by preset<number>
waybar -c ~/.config/waybar/preset1/config.jsonc -s ~/.config/waybar/preset1/style.css
