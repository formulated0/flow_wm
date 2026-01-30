#!/bin/bash

cargo build --release

if [ $? -eq 0 ]; then
    echo "starting flow_wm..."
    # Using seatd-launch to handle TTY/Hardware permissions
    seatd-launch ./target/release/flow_wm
else
    echo "compilation failed"
fi

# RESTORE TTY ON EXIT
echo "restoring tty..."
chvt 1