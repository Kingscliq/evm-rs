use evm_shared::EvmError;
use primitive_types::{U256, U512};

use tiny_keccak::{Hasher, Keccak};
use std::collections::HashSet;
use crate::stack::Stack;
use crate::memory::Memory;
use crate::storage::Storage;
use crate::context::ExecutionContext;
use crate::opcodes::*;

pub struct Evm {
    pub code: Vec<u8>,
    pub pc: usize,
    pub stack: Stack,
    pub memory: Memory,
    pub storage: Storage,
    pub gas_remaining: u64,
    pub context: ExecutionContext,
    pub jump_destinations: HashSet<usize>,
    pub trace: bool,
    pub halted: bool,
    pub logs: Vec<Log>,
}

#[derive(Debug, Clone)]
pub struct Log {
    pub address: U256,
    pub topics: Vec<U256>,
    pub data: Vec<u8>,
}

impl Evm {
    pub fn new(code: Vec<u8>, gas_limit: u64) -> Self {
        let jump_destinations = Self::analyze_jump_destinations(&code);
        Self {
            code,
            pc: 0,
            stack: Stack::new(),
            memory: Memory::new(),
            storage: Storage::new(),
            gas_remaining: gas_limit,
            context: ExecutionContext::default(),
            jump_destinations,
            trace: false,
            halted: false,
            logs: Vec::new(),
        }
    }

    /// Pre-scans bytecode to find valid JUMPDEST (0x5B) opcodes.
    /// It correctly skips data following PUSH instructions.
    fn analyze_jump_destinations(code: &[u8]) -> HashSet<usize> {
        let mut dests = HashSet::new();
        let mut i = 0;
        while i < code.len() {
            let op = code[i];
            if op == JUMPDEST {
                dests.insert(i);
            } else if (PUSH1..=PUSH32).contains(&op) {
                let n = (op - PUSH1 + 1) as usize;
                i += n;
            }
            i += 1;
        }
        dests
    }

    /// The Fetch-Decode-Execute loop
    pub fn run(&mut self) -> Result<(), EvmError> {
        while self.pc < self.code.len() && !self.halted {
            let opcode = self.code[self.pc];
            
            if self.trace {
                println!("PC: {:04X} | Op: {:02X} | Stack: {:?}", self.pc, opcode, self.stack.to_hex_strings());
            }

            self.pc += 1;

            // 1. Metering: Charge static gas
            let gas_cost = static_gas_cost(opcode);
            if self.gas_remaining < gas_cost {
                return Err(EvmError::OutOfGas);
            }
            self.gas_remaining -= gas_cost;

            // 2. Decode & Execute
            match opcode {
                STOP => break, // Exit loop successfully

                ADD => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a.overflowing_add(b).0)?;
                }

                MUL => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a.overflowing_mul(b).0)?;
                }

                SUB => {
                    let a = self.stack.pop()?; // Subtrahend
                    let b = self.stack.pop()?; // Minuend
                    self.stack.push(b.overflowing_sub(a).0)?;
                }

                DIV => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    if b.is_zero() {
                        self.stack.push(U256::zero())?;
                    } else {
                        self.stack.push(a / b)?;
                    }
                }

                SDIV => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    if b.is_zero() {
                        self.stack.push(U256::zero())?;
                    } else {
                        let (a_sign, a_abs) = get_signed(a);
                        let (b_sign, b_abs) = get_signed(b);
                        let res_abs = a_abs / b_abs;
                        let res = if a_sign == b_sign { res_abs } else { set_signed(res_abs) };
                        self.stack.push(res)?;
                    }
                }

                MOD => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    if b.is_zero() {
                        self.stack.push(U256::zero())?;
                    } else {
                        self.stack.push(a % b)?;
                    }
                }

                SMOD => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    if b.is_zero() {
                        self.stack.push(U256::zero())?;
                    } else {
                        let (a_sign, a_abs) = get_signed(a);
                        let (_, b_abs) = get_signed(b);
                        let res_abs = a_abs % b_abs;
                        let res = if a_sign { set_signed(res_abs) } else { res_abs };
                        self.stack.push(res)?;
                    }
                }

                ADDMOD => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    let n = self.stack.pop()?;
                    if n.is_zero() {
                        self.stack.push(U256::zero())?;
                    } else {
                        let a_512 = U512::from(a);
                        let b_512 = U512::from(b);
                        let n_512 = U512::from(n);
                        let res = (a_512 + b_512) % n_512;
                        
                        // We take the lower 256 bits. Mathematically, it must fit.
                        let mut bytes = [0u8; 64];
                        res.to_big_endian(&mut bytes);
                        self.stack.push(U256::from_big_endian(&bytes[32..64]))?;
                    }
                }

                MULMOD => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    let n = self.stack.pop()?;
                    if n.is_zero() {
                        self.stack.push(U256::zero())?;
                    } else {
                        let res = a.full_mul(b) % U512::from(n);
                        
                        // We take the lower 256 bits. Mathematically, it must fit.
                        let mut bytes = [0u8; 64];
                        res.to_big_endian(&mut bytes);
                        self.stack.push(U256::from_big_endian(&bytes[32..64]))?;
                    }
                }

                SHA3 => {
                    let offset = self.stack.pop()?.as_usize();
                    let size = self.stack.pop()?.as_usize();
                    
                    let mut hasher = Keccak::v256();
                    let mut output = [0u8; 32];
                    
                    self.memory.expand_if_needed(offset, size);
                    // Extract data from memory
                    let mut data = vec![0u8; size];
                    for i in 0..size {
                        data[i] = self.memory.read_byte(offset + i);
                    }
                    
                    hasher.update(&data);
                    hasher.finalize(&mut output);
                    
                    self.stack.push(U256::from_big_endian(&output))?;
                }

                EXP => {
                    let base = self.stack.pop()?;
                    let exponent = self.stack.pop()?;
                    
                    // Dynamic gas: 50 * exponent_byte_size
                    let exp_byte_size = (exponent.bits() + 7) / 8;
                    let dynamic_gas = 50 * (exp_byte_size as u64);
                    if self.gas_remaining < dynamic_gas {
                        return Err(EvmError::OutOfGas);
                    }
                    self.gas_remaining -= dynamic_gas;
                    
                    self.stack.push(base.pow(exponent))?;
                }

                SIGNEXTEND => {
                    let b = self.stack.pop()?;
                    let x = self.stack.pop()?;
                    if b < U256::from(32) {
                        let bit_index = (b.as_u32() * 8) + 7;
                        let mask = (U256::one() << bit_index) - U256::one();
                        let is_negative = x.bit(bit_index as usize);
                        let res = if is_negative {
                            x | !mask
                        } else {
                            x & mask
                        };
                        self.stack.push(res)?;
                    } else {
                        self.stack.push(x)?;
                    }
                }

                LT => {
                    let a = self.stack.pop()?; // Top
                    let b = self.stack.pop()?; // Second
                    self.stack.push(if a < b { U256::one() } else { U256::zero() })?;
                }

                GT => {
                    let a = self.stack.pop()?; // Top
                    let b = self.stack.pop()?; // Second
                    self.stack.push(if a > b { U256::one() } else { U256::zero() })?;
                }

                SLT => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    let (a_sign, _) = get_signed(a);
                    let (b_sign, _) = get_signed(b);
                    let res = if a_sign != b_sign {
                        if a_sign { U256::one() } else { U256::zero() }
                    } else {
                        if a < b { U256::one() } else { U256::zero() }
                    };
                    self.stack.push(res)?;
                }

                SGT => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    let (a_sign, _) = get_signed(a);
                    let (b_sign, _) = get_signed(b);
                    let res = if a_sign != b_sign {
                        if b_sign { U256::one() } else { U256::zero() }
                    } else {
                        if a > b { U256::one() } else { U256::zero() }
                    };
                    self.stack.push(res)?;
                }

                EQ => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(if a == b { U256::one() } else { U256::zero() })?;
                }

                ISZERO => {
                    let a = self.stack.pop()?;
                    self.stack.push(if a.is_zero() { U256::one() } else { U256::zero() })?;
                }

                AND => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a & b)?;
                }

                OR => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a | b)?;
                }

                XOR => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a ^ b)?;
                }

                NOT => {
                    let a = self.stack.pop()?;
                    self.stack.push(!a)?;
                }

                BYTE => {
                    let i = self.stack.pop()?;
                    let x = self.stack.pop()?;
                    if i < U256::from(32) {
                        let shift = (31 - i.as_u32()) * 8;
                        let res = (x >> shift) & U256::from(0xFF);
                        self.stack.push(res)?;
                    } else {
                        self.stack.push(U256::zero())?;
                    }
                }

                SHL => {
                    let shift = self.stack.pop()?;
                    let value = self.stack.pop()?;
                    if shift < U256::from(256) {
                        self.stack.push(value << shift.as_u32())?;
                    } else {
                        self.stack.push(U256::zero())?;
                    }
                }

                SHR => {
                    let shift = self.stack.pop()?;
                    let value = self.stack.pop()?;
                    if shift < U256::from(256) {
                        self.stack.push(value >> shift.as_u32())?;
                    } else {
                        self.stack.push(U256::zero())?;
                    }
                }

                SAR => {
                    let shift = self.stack.pop()?;
                    let value = self.stack.pop()?;
                    if shift < U256::from(256) {
                        let shift_u32 = shift.as_u32();
                        let (is_neg, _) = get_signed(value);
                        let mut res = value >> shift_u32;
                        if is_neg {
                            // Manual sign extension of the shifted result
                            let mask = !((U256::one() << (256 - shift_u32)) - U256::one());
                            res |= mask;
                        }
                        self.stack.push(res)?;
                    } else {
                        let (is_neg, _) = get_signed(value);
                        self.stack.push(if is_neg { !U256::zero() } else { U256::zero() })?;
                    }
                }
                POP => {
                    self.stack.pop()?;
                }

                MLOAD => {
                    let offset = self.stack.pop()?;
                    let word = self.memory.read_word(offset.as_usize());
                    self.stack.push(word)?;
                }

                MSTORE => {
                    let offset = self.stack.pop()?;
                    let value = self.stack.pop()?;
                    self.memory.store_word(offset.as_usize(), value);
                }

                MSTORE8 => {
                    let offset = self.stack.pop()?;
                    let value = self.stack.pop()?;
                    self.memory.store_byte(offset.as_usize(), (value.low_u32() & 0xFF) as u8);
                }

                MSIZE => {
                    self.stack.push(U256::from(self.memory.len()))?;
                }

                MCOPY => {
                    let dest = self.stack.pop()?;
                    let src = self.stack.pop()?;
                    let size = self.stack.pop()?;
                    self.memory.copy(src.as_usize(), dest.as_usize(), size.as_usize());
                }

                SLOAD => {
                    let key = self.stack.pop()?;
                    let val = self.storage.read(key);
                    self.stack.push(val)?;
                }

                SSTORE => {
                    let key = self.stack.pop()?;
                    let val = self.stack.pop()?;
                    self.storage.write(key, val);
                }

                // Environment
                ADDRESS => {
                    self.stack.push(self.context.address)?;
                }

                CALLER => {
                    self.stack.push(self.context.caller)?;
                }

                CALLVALUE => {
                    self.stack.push(self.context.value)?;
                }

                GAS => {
                    self.stack.push(U256::from(self.gas_remaining))?;
                }

                // Control Flow
                JUMP => {
                    let dest = self.stack.pop()?.as_usize();
                    if !self.jump_destinations.contains(&dest) {
                        return Err(EvmError::InvalidJump(dest));
                    }
                    self.pc = dest;
                }

                JUMPI => {
                    let dest = self.stack.pop()?.as_usize();
                    let cond = self.stack.pop()?;
                    if !cond.is_zero() {
                        if !self.jump_destinations.contains(&dest) {
                            return Err(EvmError::InvalidJump(dest));
                        }
                        self.pc = dest;
                    }
                }

                JUMPDEST => {} // No-op, gas already charged

                // Halting
                RETURN => {
                    self.halted = true;
                }

                REVERT => {
                    self.halted = true;
                    return Err(EvmError::Reverted(vec![]));
                }

                INVALID => {
                    self.halted = true;
                    return Err(EvmError::InvalidOpcode(opcode));
                }

                op if (LOG0..=LOG4).contains(&op) => {
                    let num_topics = (op - LOG0) as usize;
                    let offset = self.stack.pop()?.as_usize();
                    let size = self.stack.pop()?.as_usize();
                    
                    let mut topics = Vec::with_capacity(num_topics);
                    for _ in 0..num_topics {
                        topics.push(self.stack.pop()?);
                    }
                    
                    let mut data = vec![0u8; size];
                    for i in 0..size {
                        data[i] = self.memory.read_byte(offset + i);
                    }
                    
                    let log = Log {
                        address: self.context.address,
                        topics,
                        data,
                    };
                    
                    if self.trace {
                        println!("LOG: {:?}", log);
                    }
                    self.logs.push(log);
                }

                op if (PUSH1..=PUSH32).contains(&op) => {
                    let n = (op - PUSH1 + 1) as usize;
                    if self.pc + n > self.code.len() {
                        return Err(EvmError::InvalidBytecode);
                    }
                    let value = U256::from_big_endian(&self.code[self.pc..self.pc + n]);
                    self.pc += n;
                    self.stack.push(value)?;
                }

                op if (DUP1..=DUP16).contains(&op) => {
                    let depth = (op - DUP1) as usize;
                    let val = self.stack.peek(depth)?;
                    self.stack.push(val)?;
                }

                op if (SWAP1..=SWAP16).contains(&op) => {
                    let depth = (op - SWAP1 + 1) as usize;
                    self.stack.swap(depth)?;
                }

                _ => return Err(EvmError::InvalidOpcode(opcode)),

            }
        }

        Ok(())
    }
}

/// Helper to decompose a U256 into its sign bit and absolute value (two's complement)
fn get_signed(v: U256) -> (bool, U256) {
    if v.bit(255) {
        (true, (!v).overflowing_add(U256::one()).0)
    } else {
        (false, v)
    }
}

/// Helper to convert an absolute value into a negative U256 (two's complement)
fn set_signed(v: U256) -> U256 {
    (!v).overflowing_add(U256::one()).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        // Bytecode: PUSH1 0x02, PUSH1 0x03, ADD
        let code = vec![0x60, 0x02, 0x60, 0x03, 0x01];
        let mut evm = Evm::new(code, 1000); // 1000 gas limit
        
        evm.run().expect("Execution failed");
        
        // Result should be 5 on the top of the stack
        assert_eq!(evm.stack.len(), 1);
        assert_eq!(evm.stack.pop().unwrap(), U256::from(5));
        
        // Prove gas was metered: (PUSH1 = 3, PUSH1 = 3, ADD = 3) -> 9 gas total used.
        // Gas remaining should be 991.
        assert_eq!(evm.gas_remaining, 991);
    }
}
