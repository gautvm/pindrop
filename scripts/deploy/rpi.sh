#!/bin/bash

# Raspberry Pi IP address and username
RASPBERRY_PI_IP_ADDRESS="koh-raspi.local"
RASPBERRY_PI_HOSTNAME="pi"

# Path to code source directory
SRC_DIRECTORY="./"

# Name of the binary
BINARY_NAME="pindrop"

# Path to the binary
RASPBERRY_PI_BINARY_PATH="/home/pi/$BINARY_NAME"

# Code build command
BUILD_SOURCE_COMMAND="cargo build --release"

# SCP local files to the pi 
SCP_COMMAND="scp $SRC_DIRECTORY/target/release/$BINARY_NAME $RASPBERRY_PI_HOSTNAME@$RASPBERRY_PI_IP_ADDRESS:$RASPBERRY_PI_BINARY_PATH"

# SSH command to execute binary
SSH_COMMAND="ssh $RASPBERRY_PI_USERNAME@$RASPBERRY_PI_IP_ADDRESS $RASPBERRY_PI_BINARY_PATH"

# Build source code
echo "Building Pindrop binary..."
cd $SRC_DIRECTORY
$BUILD_SOURCE_COMMAND

# Deploy and scp code
echo "Copying Pindrop..."
$SCP_COMMAND

# Run binary
echo "Executing Pindrop..."
$SSH_COMMAND