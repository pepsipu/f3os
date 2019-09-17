#![no_main]
#![no_std]
/*
f3os - Sammy Hajhamid
an operating system for stm32f3 boards, just for kicks
enjoy

I tried my best to comment code so newbies can get into embedded, I hope I did a good job!
*/

use cortex_m_rt::entry;
use cortex_m::iprintln;
use heapless::{consts::U128, Vec};
use core::str::from_utf8;
use stm32f30x;

mod serial;
mod flash;

extern crate panic_halt;

#[entry]
fn main() -> ! {
    let mut recv_buffer : Vec<u8, U128> = Vec::new();
    let mut uart = serial::Serial::new(stm32f30x::usart1::RegisterBlock);
    loop {
        uart.send("root@stm32f3:~$ \n");
        uart.recv(&mut recv_buffer);
        uart.send(from_utf8(recv_buffer.as_ref()).unwrap());
    }
}