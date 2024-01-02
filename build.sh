#/usr/bin/bash

cp -r anifetch-images/ /tmp/
cargo build --release
cp target/release/anifetch ~/.local/bin/anifetch
