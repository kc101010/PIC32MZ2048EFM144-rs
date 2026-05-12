
BIN=pic32mz-blinky

cargo build
rust-objdump -sDt --demangle target/mipsel-unknown-none/release/$BIN > $BIN.lst
cargo objcopy $* -- -O ihex $BIN.hex 
