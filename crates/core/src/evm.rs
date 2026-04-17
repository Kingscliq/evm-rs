use evm_shared::EvmError;
use primitive_types::U256;

use crate::stack::Stack;
use crate::memory::Memory;
use crate::storage::Storage;
use crate::opcodes::*;

pub struct Evm {
    pub code: Vec<u8>,
    pub pc: usize,
    pub stack: Stack,
    pub memory: Memory,
    pub storage: Storage,
    pub gas_remaining: u64,
}

impl Evm {
    pub fn new(code: Vec<u8>, gas_limit: u64) -> Self {
        Self {
            code,
            pc: 0,
            stack: Stack::new(),
            memory: Memory::new(),
            storage: Storage::new(),
            gas_remaining: gas_limit,
        }
    }

    /// The Fetch-Decode-Execute loop
    pub fn run(&mut self) -> Result<(), EvmError> {
        while self.pc < self.code.len() {
            let opcode = self.code[self.pc];
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

                PUSH1 => {
                    if self.pc >= self.code.len() {
                        return Err(EvmError::InvalidBytecode);
                    }
                    let value = self.code[self.pc];
                    self.pc += 1;
                    self.stack.push(U256::from(value))?;
                }

                _ => return Err(EvmError::InvalidOpcode(opcode)),
            }
        }

        Ok(())
    }
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
