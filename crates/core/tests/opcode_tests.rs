use evm_core::evm::Evm;
use primitive_types::U256;

#[test]
fn test_signed_division() {
    // -10 / 2 = -5
    // -10 in Two's Complement: 0xFFF...F6 (U256::MAX - 9)
    // 2: 0x02
    let code = vec![
        0x60, 0x02, // PUSH1 2
        0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF6, // PUSH32 -10
        0x05, // SDIV
    ];
    let mut evm = Evm::new(code, 1000);
    evm.run().unwrap();
    
    // Result should be -5: 0xFFF...FB
    let res = evm.stack.pop().unwrap();
    assert!(res.bit(255)); // Sign bit is set
}

#[test]
fn test_exp_gas_deduction() {
    // PUSH1 2, PUSH1 10, EXP (10 is 0x0A)
    // 10 has 1 byte.
    // Static Gas (10) + Dynamic Gas (50 * 1 byte) = 60 gas used.
    // Plus 2 PUSH1 (3 gas each) = 66 gas used total.
    let code = vec![0x60, 0x02, 0x60, 0x0A, 0x0A];
    let mut evm = Evm::new(code, 100);
    evm.run().unwrap();
    
    assert_eq!(evm.gas_remaining, 100 - 66);
}

#[test]
fn test_lt_gt() {
    // 5 < 10 (True = 1)
    let code = vec![0x60, 0x0A, 0x60, 0x05, 0x10];
    let mut evm = Evm::new(code, 100);
    evm.run().unwrap();
    assert_eq!(evm.stack.pop().unwrap(), U256::one());

    // 10 < 5 (False = 0)
    let code = vec![0x60, 0x05, 0x60, 0x0A, 0x10];
    let mut evm = Evm::new(code, 100);
    evm.run().unwrap();
    assert_eq!(evm.stack.pop().unwrap(), U256::zero());
}

#[test]
fn test_stack_memory_storage_flow() {
    // 1. PUSH1 0x42 (Value)
    // 2. PUSH1 0x00 (Memory Offset)
    // 3. MSTORE
    // 4. PUSH1 0x00 (Memory Offset)
    // 5. MLOAD
    // 6. PUSH1 0x00 (Storage Key)
    // 7. SSTORE
    // 8. PUSH1 0x00 (Storage Key)
    // 9. SLOAD
    let code = vec![
        0x60, 0x42, // PUSH1 42
        0x60, 0x00, // PUSH1 0
        0x52,       // MSTORE
        0x60, 0x00, // PUSH1 0
        0x51,       // MLOAD
        0x60, 0x00, // PUSH1 0
        0x55,       // SSTORE
        0x60, 0x00, // PUSH1 0
        0x54,       // SLOAD
    ];
    let mut evm = Evm::new(code, 30000);
    evm.run().unwrap();
    
    assert_eq!(evm.stack.pop().unwrap(), U256::from(0x42));
}

#[test]
fn test_loop_countdown() {
    // 1. PUSH1 0x03 (Counter)
    // 2. JUMPDEST (Loop Start at PC 2)
    // 3. PUSH1 0x01
    // 4. SUB (Counter - 1)
    // 5. DUP1
    // 6. PUSH1 0x02 (Loop Start PC)
    // 7. JUMPI (Jump to 2 if Counter > 0)
    let code = vec![
        0x60, 0x03,       // PUSH1 3
        0x5B,             // JUMPDEST (PC: 2)
        0x60, 0x01,       // PUSH1 1
        0x03,             // SUB (pops 1, pops 3 -> 2)
        0x80,             // DUP1
        0x60, 0x02,       // PUSH1 2 (dest)
        0x57,             // JUMPI (pops 2, pops 2 -> jumps)
    ];
    let mut evm = Evm::new(code, 1000);
    evm.run().unwrap();
    
    assert_eq!(evm.stack.pop().unwrap(), U256::zero());
}

#[test]
fn test_environmental_opcodes() {
    let code = vec![
        0x30, // ADDRESS
        0x33, // CALLER
    ];
    let mut evm = Evm::new(code, 100);
    evm.run().unwrap();
    
    assert_eq!(evm.stack.pop().unwrap(), U256::from(0x2000)); // CALLER
    assert_eq!(evm.stack.pop().unwrap(), U256::from(0x1000)); // ADDRESS
}

#[test]
fn test_invalid_jump_destination() {
    // Try to jump to PC 1 (which is the data of a PUSH1)
    let code = vec![0x60, 0xFF, 0x56]; 
    let mut evm = Evm::new(code, 100);
    let res = evm.run();
    
    assert!(res.is_err());
}

#[test]
fn test_sha3_hashing() {
    // 1. PUSH1 0x42
    // 2. PUSH1 0x00
    // 3. MSTORE (Memory[0..32] = 0x42)
    // 4. PUSH1 0x20 (Size 32)
    // 5. PUSH1 0x00 (Offset 0)
    // 6. SHA3
    let code = vec![
        0x60, 0x42,
        0x60, 0x00,
        0x52,
        0x60, 0x20,
        0x60, 0x00,
        0x20,
    ];
    let mut evm = Evm::new(code, 1000);
    evm.run().unwrap();
    
    let result = evm.stack.pop().unwrap();
    // Keccak256 of 32-byte word 0x42
    let expected = U256::from_str_radix("38dfe4635b27babeca8be38d3b448cb5161a639b899a14825ba9c8d7892eb8c3", 16).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_log_events() {
    // PUSH1 0xAA (Data)
    // PUSH1 0x00 (Offset)
    // MSTORE
    // PUSH1 0xBB (Topic 1)
    // PUSH1 0x20 (Size)
    // PUSH1 0x00 (Offset)
    // LOG1
    let code = vec![
        0x60, 0xAA,
        0x60, 0x00,
        0x52,
        0x60, 0xBB,
        0x60, 0x20,
        0x60, 0x00,
        0xA1,
    ];
    let mut evm = Evm::new(code, 1000);
    evm.run().unwrap();
    
    assert_eq!(evm.logs.len(), 1);
    assert_eq!(evm.logs[0].topics[0], U256::from(0xBB));
    assert_eq!(evm.logs[0].data[31], 0xAA);
}

#[test]
fn test_gas_exhaustion() {
    // PUSH32 (huge value) - takes 3 gas
    // PUSH32 (huge value) - takes 3 gas
    // EXP (huge gas cost)
    let code = vec![
        0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0x0A,
    ];
    let mut evm = Evm::new(code, 10); // Very low gas limit
    let res = evm.run();
    
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), evm_shared::EvmError::OutOfGas);
}

#[test]
fn test_revert_opcode() {
    let code = vec![
        0x60, 0x00, // offset
        0x60, 0x00, // size
        0xFD,       // REVERT
    ];
    let mut evm = Evm::new(code, 100);
    let res = evm.run();
    
    assert!(res.is_err());
    // Should return Reverted error
    match res.unwrap_err() {
        evm_shared::EvmError::Reverted(_) => (),
        e => panic!("Expected Reverted error, got {:?}", e),
    }
}
