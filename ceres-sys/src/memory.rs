/// all the memory as one big fat fucking slice
pub struct Memory {
    data: [u16; std::u16::MAX as usize],
}

impl Memory {
    pub fn init() -> Memory {
        Memory { data: [0x00; std::u16::MAX as usize] }
    }
}

impl std::ops::Index<u16> for Memory {
    type Output = u16;

    fn index(&self, index: u16) -> &u16 {
        &self.data[index as usize]
    }
}

impl std::ops::IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut u16 {
        &mut self.data[index as usize]
    }
}
