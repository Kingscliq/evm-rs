/// Pre-defined standard gas costs based on the Ethereum Yellow Paper
pub const GAS_ZERO: u64 = 0;
pub const GAS_VERYLOW: u64 = 3;
pub const GAS_LOW: u64 = 5;

pub const STOP: u8 = 0x00;
pub const ADD: u8 = 0x01;
pub const MUL: u8 = 0x02;

// Stack Manipulation (The PUSH family: 0x60 is PUSH1 ... 0x7f is PUSH32)
pub const PUSH1: u8 = 0x60;
pub const PUSH32: u8 = 0x7f;
