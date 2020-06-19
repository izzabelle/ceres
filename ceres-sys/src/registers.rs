use std::fmt;

/// 16 register cpu only capable of holding 16 bit unsigned integers
pub struct Registers {
    // zero register
    z0: u16,

    // system registers
    pc: u16,
    sp: u16,
    ra: u16,

    // argument registers
    a0: u16,
    a1: u16,
    a2: u16,

    // return registers
    v0: u16,
    v1: u16,

    // temp registers
    t0: u16,
    t1: u16,
    t2: u16,
    t3: u16,
    t4: u16,
    t5: u16,
    t6: u16,
}

impl Registers {
    /// initializes cpu with 0x0000 in all registers
    pub fn init() -> Registers {
        Registers {
            z0: 0x0000,
            pc: 0x0000,
            sp: 0x0000,
            ra: 0x0000,
            a0: 0x0000,
            a1: 0x0000,
            a2: 0x0000,
            v0: 0x0000,
            v1: 0x0000,
            t0: 0x0000,
            t1: 0x0000,
            t2: 0x0000,
            t3: 0x0000,
            t4: 0x0000,
            t5: 0x0000,
            t6: 0x0000,
        }
    }
}

/// indexes registers by register number and returns reference to the registers
/// contents
impl std::ops::Index<usize> for Registers {
    type Output = u16;

    fn index(&self, idx: usize) -> &u16 {
        match idx {
            0 => &self.z0,
            1 => &self.pc,
            2 => &self.sp,
            3 => &self.ra,
            4 => &self.a0,
            5 => &self.a1,
            6 => &self.a2,
            7 => &self.v0,
            8 => &self.v1,
            9 => &self.t0,
            10 => &self.t1,
            11 => &self.t2,
            12 => &self.t3,
            13 => &self.t4,
            14 => &self.t5,
            15 => &self.t6,
            _ => panic!("Invalid register number"),
        }
    }
}

/// indexes registers by register number and returns mutable reference to the
/// registers contents
impl std::ops::IndexMut<usize> for Registers {
    fn index_mut(&mut self, idx: usize) -> &mut u16 {
        match idx {
            0 => &mut self.z0,
            1 => &mut self.pc,
            2 => &mut self.sp,
            3 => &mut self.ra,
            4 => &mut self.a0,
            5 => &mut self.a1,
            6 => &mut self.a2,
            7 => &mut self.v0,
            8 => &mut self.v1,
            9 => &mut self.t0,
            10 => &mut self.t1,
            11 => &mut self.t2,
            12 => &mut self.t3,
            13 => &mut self.t4,
            14 => &mut self.t5,
            15 => &mut self.t6,
            _ => panic!("Invalid register number"),
        }
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "-- registers ----------------------------------------\n\
| z0: 0x{:04X} | pc: 0x{:04X} | sp: 0x{:04X} | ra: 0x{:04X} |\n\
| a0: 0x{:04X} | a1: 0x{:04X} | a2: 0x{:04X} | v0: 0x{:04X} |\n\
| v1: 0x{:04X} | t0: 0x{:04X} | t1: 0x{:04X} | t2: 0x{:04X} |\n\
| t3: 0x{:04X} | t4: 0x{:04X} | t5: 0x{:04X} | t6: 0x{:04X} |\n\
-----------------------------------------------------",
            self[0],
            self[1],
            self[2],
            self[3],
            self[4],
            self[5],
            self[6],
            self[7],
            self[8],
            self[9],
            self[10],
            self[11],
            self[12],
            self[13],
            self[14],
            self[15]
        )
    }
}
