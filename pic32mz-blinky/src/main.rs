#![no_std]
#![no_main]
#![feature(asm)]

use mips_mcu::interrupt;
use mips_rt::entry;
use panic_halt as _;
use pic32mz2048efm144_pac::Peripherals;

#[link_section = ".configsfrs"]
#[used]
pub static CONFIGSFRS: [u32; 48] = [
    0x0fffffff,
    0xfff9ffd9,
    0xff7fcfd9,
    0x7ffffffb,

    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffffffff,
    0xffff0000,     // Sequence number
];

const SEQ_NO: u16 = 1;

#[link_section = "seqsfrs"]
#[used]
pub static SEQSFRS: [u16; 2] = [SEQ_NO, !SEQ_NO];

#[entry]
fn main() -> ! {
    let p = unsafe { Peripherals::steal() };
    
    //Set PORTH GPIO as output
    p.porth.trish().write(|w| unsafe{w.bits(0)});
    p.porth.lath().write(|w| unsafe{w.bits(1)});

    //toggle LED with delay
    loop {
        ;
    }
}
