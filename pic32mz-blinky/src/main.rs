#![no_std]
#![no_main]

use mips_rt::entry;
use panic_halt as _;
use pic32mz2048efm144_pac::Peripherals as pac;

#[entry]
fn main() -> ! {
    let p = unsafe{pac::steal();};

    

    loop{};
}