#!/bin/bash

# n3bu1a shell controller
# i made this so i dont have to memorize long commands
# use: n3bu1a-ctl <action>

ACTION=$1
SAVE_DIR="$HOME/Screenshots"
TIMESTAMP=$(date +'%Y%m%d_%H%M%S')

case $ACTION in
    "screenshot-area")
        # drag a box and it saves + copies. ez.
        grim -g "$(slurp)" "$SAVE_DIR/n3bu1a_$TIMESTAMP.png"
        wl-copy < "$SAVE_DIR/n3bu1a_$TIMESTAMP.png"
        notify-send "snagged that area for u"
        ;;
    "screenshot-full")
        # whole screen flex
        grim "$SAVE_DIR/n3bu1a_$TIMESTAMP.png"
        notify-send "saved the full screen" # tbh this is really bad source code ik that but the project is something bright
        ;;
    "reload")
        # if the bar acting weird just run this
        pkill waybar
        waybar &
        ;;
    *)
        # u typed it wrong bro
        echo "n3bu1a: $ACTION is not a thing"
        exit 1
        ;;
esac
