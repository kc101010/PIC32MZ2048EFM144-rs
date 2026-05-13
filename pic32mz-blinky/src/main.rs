#![no_std]
#![no_main]
#![feature(asm)]

use mips_mcu::interrupt;
use mips_rt::entry;
use panic_halt as _;
use pic32mz2048efm144_pac::Peripherals;

#[link_section = ".configsfrs"]
#[used]
//See research-notes/MPLAB-CONFIGSRS.txt
pub static CONFIGSFRS: [u32; 7] = [
    0xFFFFFFFF, //DEFCFG3
    0xFFFFFFFF, //DEFCFG2
    0xFFFFFFFF, //DEFCFG1
    0xFFFFFFFF, //DEFCFG0
    0xFFFFFFFF, //DEFCP0
    0xFFFFFFFF, //SEQ3
    0x0000FFFF, //DEFCFG3
];

const SEQ_NO: u16 = 1;

#[link_section = "seqsfrs"]
#[used]
pub static SEQSFRS: [u16; 2] = [SEQ_NO, !SEQ_NO];

#[entry]
fn main() -> ! {
    let p = unsafe { Peripherals::steal() };

    //Set PORT H pins to digital (ansel h reg to 0)
    p.porth.anselhset().write(|w| unsafe{w.bits(0)});
 
    //Set PORTH GPIO as output
    p.porth.trish().write(|w| unsafe{w.bits(0)});
    
    //toggle LED with delay
    loop {
        //Turn LED ON
        p.porth.lath().write(|w| unsafe{w.bits(1)});
        
        let mut i: i32 = 0;
        while i < 200000 {
            i += 1;
        }
        //Turn LED OFF
        p.porth.lath().write(|w| unsafe{w.bits(0)});
    }
}
