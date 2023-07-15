#!/bin/sh

cargo build --release
mkdir -p $HOME/.local/bin
cp target/release/chainsaw $HOME/.local/bin
echo "Make sure $HOME/.local/bin is in path"
