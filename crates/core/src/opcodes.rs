/// Pre-defined standard gas costs based on the Ethereum Yellow Paper
pub const GAS_ZERO: u64 = 0;
pub const GAS_VERYLOW: u64 = 3;
pub const GAS_LOW: u64 = 5;

// Halting
pub const STOP: u8 = 0x00;

// Arithmetic
pub const ADD: u8 = 0x01;
pub const MUL: u8 = 0x02;

// Stack Manipulation (The PUSH family: 0x60 is PUSH1 ... 0x7f is PUSH32)
pub const PUSH1: u8 = 0x60;
pub const PUSH32: u8 = 0x7f;

// static gas cost for a given opcode independent of memory expansion.
pub fn static_gas_cost(opcode: u8) -> u64 {
    match opcode {
        STOP => GAS_ZERO,
        ADD => GAS_VERYLOW,
        MUL => GAS_LOW,
        op if (PUSH1..=PUSH32).contains(&op) => GAS_VERYLOW,
        _ => 0, // Fallback for unsupported opcodes in this phase
    }
}
