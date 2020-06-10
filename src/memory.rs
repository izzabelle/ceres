/// all the memory as one big fat fucking slice
pub struct Memory {
    pub data: [u16; std::u16::MAX as usize],
}

impl Memory {
    pub fn init() -> Memory {
        Memory { data: [0x00; std::u16::MAX as usize] }
    }
}
