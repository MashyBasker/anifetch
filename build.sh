#/usr/bin/bash

mkdir -p ~/.config/anifetch
cp -r gallery/ ~/.config/anifetch
cargo build --release
cp target/release/anifetch ~/.local/bin/anifetch
