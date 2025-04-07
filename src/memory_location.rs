pub enum MemoryValue {
    U16(u16),
    U8(u8),
    Bool(bool),
}

pub struct MemoryLocation {
    pub addr: u8,
    pub size: u8,
}

pub const ACCELERATION: MemoryLocation = MemoryLocation {
    addr: 0x29,
    size: 1,
};
pub const CURRENT_POSITION: MemoryLocation = MemoryLocation {
    addr: 0x38,
    size: 2,
};
pub const ID: MemoryLocation = MemoryLocation {
    addr: 0x05,
    size: 1,
};

pub const TARGET_POSITION: MemoryLocation = MemoryLocation {
    addr: 0x2A,
    size: 2,
};

pub const TEMPERATURE: MemoryLocation = MemoryLocation {
    addr: 0x3F,
    size: 1,
};

pub const TORQUE_SWITCH: MemoryLocation = MemoryLocation {
    addr: 0x28,
    size: 1,
};
