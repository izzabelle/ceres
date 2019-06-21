/// 16 register cpu only capable of holding 16 bit unsigned integers
pub struct Cpu {
    // zero register
    z0: u16,

    // system registers
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
    t7: u16,
}

/// initializes cpu with 0x00 in all registers
impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            z0: 0,
            sp: 0,
            ra: 0,
            a0: 0,
            a1: 0,
            a2: 0,
            v0: 0,
            v1: 0,
            t0: 0,
            t1: 0,
            t2: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
            t7: 0,
        }
    }
}

/// indexes registers by register number and returns reference to the registers
/// contents
impl std::ops::Index<usize> for Cpu {
    type Output = u16;

    fn index(&self, idx: usize) -> &u16 {
        match idx {
            0 => &self.z0,
            1 => &self.sp,
            2 => &self.ra,
            3 => &self.a0,
            4 => &self.a1,
            5 => &self.a2,
            6 => &self.v0,
            7 => &self.v1,
            8 => &self.t0,
            9 => &self.t1,
            10 => &self.t2,
            11 => &self.t3,
            12 => &self.t4,
            13 => &self.t5,
            14 => &self.t6,
            15 => &self.t7,
            _ => panic!("Invalid register number"),
        }
    }
}

/// indexes registers by register number and returns mutable reference to the
/// registers contents
impl std::ops::IndexMut<usize> for Cpu {
    fn index_mut(&mut self, idx: usize) -> &mut u16 {
        match idx {
            0 => &mut self.z0,
            1 => &mut self.sp,
            2 => &mut self.ra,
            3 => &mut self.a0,
            4 => &mut self.a1,
            5 => &mut self.a2,
            6 => &mut self.v0,
            7 => &mut self.v1,
            8 => &mut self.t0,
            9 => &mut self.t1,
            10 => &mut self.t2,
            11 => &mut self.t3,
            12 => &mut self.t4,
            13 => &mut self.t5,
            14 => &mut self.t6,
            15 => &mut self.t7,
            _ => panic!("Invalid register number"),
        }
    }
}
