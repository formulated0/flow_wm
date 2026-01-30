#!/bin/bash

# Enable permissions for the VM user to access input/video without root if configured, 
# or just run as root for the initial VM test phase for simplicity.
# If running as root (simplest for dev):
cargo build --release

if [ $? -eq 0 ]; then
    echo "Starting FlowWM..."
    # Using seatd-launch to handle TTY/Hardware permissions
    seatd-launch ./target/release/flow_wm
else
    echo "Compilation failed."
fi

# RESTORE TTY ON EXIT
echo "Restoring TTY..."
chvt 1