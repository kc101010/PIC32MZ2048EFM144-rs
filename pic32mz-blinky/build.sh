cargo build --release
cargo objcopy --release $* -- -O ihex blinky.hex 