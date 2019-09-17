use stm32f30x;

pub struct FS {
    flash: &'static mut stm32f30x::FLASH
}

impl FS {
    pub fn new(flash: &'static mut stm32f30x::FLASH) -> FS {
        FS {
            flash
        }
    }
    pub fn read(&mut self, address: u16) {
    }
}