/// Pre-defined standard gas costs based on the Ethereum Yellow Paper
pub const GAS_ZERO: u64 = 0;
pub const GAS_JUMPDEST: u64 = 1;
pub const GAS_BASE: u64 = 2;
pub const GAS_VERYLOW: u64 = 3;
pub const GAS_LOW: u64 = 5;
pub const GAS_MID: u64 = 8;
pub const GAS_EXPONENTIAL: u64 = 10;
pub const GAS_SLOAD: u64 = 800;
pub const GAS_SSTORE: u64 = 20000;
pub const GAS_SHA3: u64 = 30;
pub const GAS_LOG: u64 = 375;

// Halting
pub const STOP: u8 = 0x00;

// Arithmetic
pub const ADD: u8 = 0x01;
pub const MUL: u8 = 0x02;
pub const SUB: u8 = 0x03;
pub const DIV: u8 = 0x04;
pub const SDIV: u8 = 0x05;
pub const MOD: u8 = 0x06;
pub const SMOD: u8 = 0x07;
pub const ADDMOD: u8 = 0x08;
pub const MULMOD: u8 = 0x09;
pub const EXP: u8 = 0x0A;
pub const SIGNEXTEND: u8 = 0x0B;

// Comparison
pub const LT: u8 = 0x10;
pub const GT: u8 = 0x11;
pub const SLT: u8 = 0x12;
pub const SGT: u8 = 0x13;
pub const EQ: u8 = 0x14;
pub const ISZERO: u8 = 0x15;

// Bitwise
pub const AND: u8 = 0x16;
pub const OR: u8 = 0x17;
pub const XOR: u8 = 0x18;
pub const NOT: u8 = 0x19;
pub const BYTE: u8 = 0x1A;
pub const SHL: u8 = 0x1B;
pub const SHR: u8 = 0x1C;
pub const SAR: u8 = 0x1D;

// Hashing
pub const SHA3: u8 = 0x20;

// Logging
pub const LOG0: u8 = 0xA0;
pub const LOG1: u8 = 0xA1;
pub const LOG2: u8 = 0xA2;
pub const LOG3: u8 = 0xA3;
pub const LOG4: u8 = 0xA4;

// Stack Manipulation (The PUSH family: 0x60 is PUSH1 ... 0x7f is PUSH32)
pub const POP: u8 = 0x50;
pub const PUSH1: u8 = 0x60;
pub const PUSH32: u8 = 0x7f;

pub const DUP1: u8 = 0x80;
pub const DUP16: u8 = 0x8F;

pub const SWAP1: u8 = 0x90;
pub const SWAP16: u8 = 0x9F;

// Memory
pub const MLOAD: u8 = 0x51;
pub const MSTORE: u8 = 0x52;
pub const MSTORE8: u8 = 0x53;
pub const MSIZE: u8 = 0x59;
pub const MCOPY: u8 = 0x5E;

// Storage
pub const SLOAD: u8 = 0x54;
pub const SSTORE: u8 = 0x55;

// Environment
pub const ADDRESS: u8 = 0x30;
pub const CALLER: u8 = 0x33;
pub const CALLVALUE: u8 = 0x34;

// Control Flow
pub const JUMP: u8 = 0x56;
pub const JUMPI: u8 = 0x57;
pub const JUMPDEST: u8 = 0x5B;
pub const GAS: u8 = 0x5A;

// Halting
pub const RETURN: u8 = 0xF3;
pub const REVERT: u8 = 0xFD;
pub const INVALID: u8 = 0xFE;
