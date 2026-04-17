use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum EvmError {
    #[error("Stack underflow: tried to pop an empty stack")]
    StackUnderflow,
    #[error("Stack overflow: tried to push more than 1024 items")]
    StackOverflow,
    #[error("Invalid Opcode encountered: {0:#x}")]
    InvalidOpcode(u8),
    #[error("Out of gas")]
    OutOfGas,
    #[error("Invalid bytecode: expected data missing")]
    InvalidBytecode,
}
