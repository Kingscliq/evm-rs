pub use evm_shared::types::*;

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
