# device.x
```
/* IRQ vector #0 */
PROVIDE(CORE_TIMER = _default_isr_fn);
PROVIDE(CORE_TIMER_CONTEXT = _isr_context);
_vector_0_fn = CORE_TIMER;
_vector_0_context = CORE_TIMER_CONTEXT;
```

Not sure about default_isr_fn or isr_context  as these seem to be used in every IRQ entry.
The vector numbers and names look to match the ICD.

# pic32mz-blinky/memory.x

Values used for ORIGIN vars seem sensible enough compared to ICD.

# pic32mz-blinky/src/main.rs

CONFIGSFRS declared at the top of the file  is linked to the configsfrs region in the memory linker script,, to set the device config register(s).

As noted in the embedded-training repo. MPLAB can generate these values for you for the hardware you have connected. I have output this to a txt file.

I re-worked the CONFIGSFRS list to more closely match what mplab creates for the C code. I used the txt file I created and placed the values into CONFIGSFRS.

```rust
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
```

Reprogramming the hex onto the starter kit with this change did not result in any visible changes.

I also wanted to see if placing the LATH writes into the loop might result in something:

```rust
#[entry]
fn main() -> ! {
    let p = unsafe { Peripherals::steal() };
    
    //Set PORTH GPIO as output
    p.porth.trish().write(|w| unsafe{w.bits(0)});
    
    //toggle LED with delay
    loop {
        //Turn LED ON
        p.porth.lath().write(|w| unsafe{w.bits(1)});

        //Turn LED OFF
        p.porth.lath().write(|w| unsafe{w.bits(0)});
    }
}

```

In a similar sense to how my C example works:
```c
#define timeout 1000
#define HIGH 1
#define LOW 0

#include <xc.h>


int main(void)
{
    TRISHbits.TRISH0 = 0; 
    TRISHbits.TRISH1 = 0; 
    TRISHbits.TRISH2 = 0; 

    while(1)
    {
        for(int i = 0; i < timeout; i++)
        {}
        
        LATHbits.LATH2 = LOW;
        LATHbits.LATH1 = LOW;
        LATHbits.LATH0 = LOW;
        
        
        for(int i = 0; i < timeout; i++)
        {}
        
        LATHbits.LATH2 = HIGH;
        LATHbits.LATH1 = HIGH;
        LATHbits.LATH0 = HIGH;

    }
    
    return EXIT_SUCCESS;
}
```

Though this change did not result in a different behaviour from the starter kit. No LED. :(


Also made further changes to set PORT H pins as digital, added in timer countdown and tried some interrupt enable code. Still no change.

```rust
#[entry]
fn main() -> ! {
    let p = unsafe { Peripherals::steal() };

    //Set PORT H pins to digital (ansel h reg to 0)
    p.porth.anselhset().write(|w| unsafe{w.bits(0)});

    unsafe{
        interrupt::enable_mv_irq();
        p.int.intconset().write(|w| {w.mvec().bit(true)});
        p.int.priss().write(|w| {w.ss0().bit(false)});
        interrupt::enable();
    }
    
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

```

# pic32mz-blinky/pic32mz-blinky.lst

This file is an objdump of the hex file contents. 