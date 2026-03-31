#!/bin/bash

set -e

echo "Installing rusk ...."
curl -L https://github.com/flippedpants/rusk/releases/latest/download/rusk -o rusk

chmod +x rusk
sudo mv rusk /usr/local/bin

echo "rusk installed succesfully! Run the command: rusk"