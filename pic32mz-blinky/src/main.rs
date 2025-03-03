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

    unsafe{
        interrupt::enable_mv_irq();
        p.int.intconset().write(|w| {w.mvec().bit(true)});
        p.int.priss().write(|w| {w.ss0().bit(false)});
        interrupt::enable();
    }

    //init timer 
    p.tmr1.pr1().write(|w| unsafe {w.bits(0xffff)});
    p.tmr1.t1con().write(|w| unsafe {w.on().bit(true).tckps().bits(0b11)});

    //init oscillator?
    p.int.ifs0clr().write(|w| {w.t1if().bit(true)});
    p.int.iec0set().write(|w| {w.t1ie().bit(true)});
    p.int.ipc1().modify(|_, w| unsafe{w.t1ip().bits(1)});

    //set RB3 to digital output
    p.portb.anselb().write(|w| unsafe{ w.bits(0) });
    p.portb.trisb().write(|w| unsafe{ w.bits(0) });

    //toggle LED with delay
    loop {
        p.portb.latb().write(|w| unsafe{w.bits(0)});

        let mut i: i32= 0;
        while i < 200000 {
            i += 1;
            //unsafe{ asm!("nop") };
        }
    
        p.portb.latb().write(|w| unsafe{w.bits(1)});
    }
}
