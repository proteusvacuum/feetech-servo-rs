pub enum Instruction {
    Ping,
    Read,
    Write,
    // RegWrite,
    // Action,
    // SyncWrite,
    // SyncRead,
}

impl From<Instruction> for u8 {
    fn from(value: Instruction) -> Self {
        match value {
            // This instruction is used to see if a device exists.
            Instruction::Ping => 1,
            Instruction::Read => 2,
            // Write Instruction is executed immediately when an Instruction Packet is received.
            Instruction::Write => 3,
            // Reg Write Instruction registers the Instruction Packet to a standby status, and sets Control table Registered Instruction to ‘1’.
            // Instruction::RegWrite => 4,
            // This instruction is to execute the registered Reg Write instruction
            // Instruction::Action => 5,
            // Instruction::SyncWrite => 0x83,
            // Instruction::SyncRead => 0x82,
        }
    }
}
