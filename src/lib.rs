mod memory;
mod registers;

pub use memory::Memory;
pub use registers::Registers;

pub struct System {
    pub registers: Registers,
    pub memory: Memory,
}

impl System {
    pub fn init() -> System {
        System { registers: Registers::init(), memory: Memory::init() }
    }
}
