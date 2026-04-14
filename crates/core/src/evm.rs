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
    
    // Gas accounting mapped out!
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

    /// The main Fetch-Decode-Execute loop!
    pub fn run(&mut self) -> Result<(), EvmError> {
        todo!("Task 01: Implement Fetch-Decode-Execute loop with gas metering")
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
