/// video memory
pub struct VideoMemory {
    data: [u16; crate::VIDEO_MEMORY_LEN],
}

impl VideoMemory {
    /// initialize the video memory
    pub fn init() -> VideoMemory {
        VideoMemory { data: [0x0000; crate::VIDEO_MEMORY_LEN] }
    }
}

impl std::ops::Index<u16> for VideoMemory {
    type Output = u16;

    fn index(&self, index: u16) -> &u16 {
        &self.data[index as usize]
    }
}

impl std::ops::IndexMut<u16> for VideoMemory {
    fn index_mut(&mut self, index: u16) -> &mut u16 {
        &mut self.data[index as usize]
    }
}
