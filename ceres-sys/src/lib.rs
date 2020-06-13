mod memory;
mod registers;
mod video;

pub use memory::Memory;
pub use registers::Registers;
pub use video::VideoMemory;

/// ceres screen width
pub const SCREEN_WIDTH: usize = 256;
/// ceres screen height
pub const SCREEN_HEIGHT: usize = 144;
/// ceres video memory buffer length
pub const VIDEO_MEMORY_LEN: usize = SCREEN_HEIGHT * SCREEN_WIDTH;

/// the core system structure of ceres
pub struct System {
    pub registers: Registers,
    pub memory: Memory,
    pub video_memory: VideoMemory,
}

impl System {
    /// initialize a new system and all of it's component parts
    pub fn init() -> System {
        System {
            registers: Registers::init(),
            memory: Memory::init(),
            video_memory: VideoMemory::init(),
        }
    }
}
