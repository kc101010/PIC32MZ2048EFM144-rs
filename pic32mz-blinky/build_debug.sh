
BIN=pic32mz-blinky

cargo build
rust-objdump -sDt --demangle target/mipsel-unknown-none/debug/$BIN > $BIN.lst
cargo objcopy $* -- -O ihex $BIN.hex 
