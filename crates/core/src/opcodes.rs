pub use evm_shared::types::*;

// static gas cost for a given opcode independent of memory expansion.
pub fn static_gas_cost(opcode: u8) -> u64 {
    match opcode {
        STOP => GAS_ZERO,
        
        ADD | SUB | LT | GT | SLT | SGT | EQ | ISZERO | AND | OR | XOR | NOT | BYTE | SHL | SHR | SAR 
        | MLOAD | MSTORE | MSTORE8 | MSIZE | MCOPY => {
            GAS_VERYLOW
        }

        POP => {
            GAS_BASE
        }

        ADDRESS | CALLER | CALLVALUE | GAS => {
            GAS_BASE
        }
        
        MUL | DIV | SDIV | MOD | SMOD | SIGNEXTEND => {
            GAS_LOW
        }
        
        ADDMOD | MULMOD | JUMP => {
            GAS_MID
        }
        
        EXP | JUMPI => {
            GAS_EXPONENTIAL
        }

        JUMPDEST => {
            GAS_JUMPDEST
        }

        RETURN | REVERT | INVALID => {
            GAS_ZERO
        }

        SLOAD => {
            GAS_SLOAD
        }

        SSTORE => {
            GAS_SSTORE
        }

        SHA3 => {
            GAS_SHA3
        }

        op if (LOG0..=LOG4).contains(&op) => {
            GAS_LOG
        }

        op if (PUSH1..=PUSH32).contains(&op) => GAS_VERYLOW,
        op if (DUP1..=DUP16).contains(&op) => GAS_VERYLOW,
        op if (SWAP1..=SWAP16).contains(&op) => GAS_VERYLOW,
        _ => 0, // Fallback for unsupported opcodes in this phase
    }
}
