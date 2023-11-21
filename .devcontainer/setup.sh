#!/bin/bash
curl https://sh.rustup.rs -sSf | sh -s -- -y

# Update current shell environment variables after install to find rustup
source "$HOME/.cargo/env"
rustup install stablec