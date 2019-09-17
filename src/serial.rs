use stm32f30x::{
    usart1
};
use heapless::{consts::U128, Vec};
use core::str;
use core::str::from_utf8;

pub struct Serial {
    uart: &'static mut RegisterBlock,
}

impl Serial {
    pub fn new(uart: &'static mut RegisterBlock) -> Serial {
        Serial {
            uart,
        }
    }

    pub fn send(&self, value: &str) {
        for character in value.bytes() {
            while self.uart.isr.read().txe().bit_is_clear() {}
            self.uart.tdr.write(|w| w.tdr().bits(u16::from(character)));
        }
    }
    pub fn recv(&mut self, output_buffer: &mut Vec<u8, U128>) {
        output_buffer.clear();
        loop {
            while self.uart.isr.read().rxne().bit_is_clear() {}
            let byte = self.uart.rdr.read().rdr().bits() as u8;
            if output_buffer.push(byte).is_err() {
                self.send("Error: Max input length is 128 bytes.");
                break;
            }
            if byte == 10 {
                break;
            }
        }
    }
}

