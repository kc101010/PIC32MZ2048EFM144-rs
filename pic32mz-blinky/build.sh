
BIN=pic32mz-blinky

cargo build --release
rust-objdump -sDt --demangle target/mipsel-unknown-none/release/$BIN > $BIN.lst
cargo objcopy --release $* -- -O ihex $BIN.hex 
