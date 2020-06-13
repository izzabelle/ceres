mod memory;
mod registers;
mod video;

pub use memory::Memory;
pub use registers::Registers;
pub use video::VideoMemory;

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 144;
pub const VIDEO_MEMORY_LEN: usize = SCREEN_HEIGHT * SCREEN_WIDTH;

pub struct System {
    pub registers: Registers,
    pub memory: Memory,
    pub video_memory: VideoMemory,
}

impl System {
    pub fn init() -> System {
        System {
            registers: Registers::init(),
            memory: Memory::init(),
            video_memory: VideoMemory::init(),
        }
    }
}
