#![no_std]
#![feature(asm)]

extern crate tock;

use tock::{led, timer};

fn main() {
    let led_count = led::count();
    loop {
        for i in 0..led_count {
            led::toggle(i as u32);
            for j in 0..1000000 {
                unsafe { asm!("nop") }
            }
            //timer::delay_ms(500);
        }
    }
}
