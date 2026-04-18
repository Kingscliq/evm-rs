# EVM Interpreter in Rust

A modular, high-performance Ethereum Virtual Machine (EVM) interpreter built from the ground up in Rust. This project implements the core logic of the Ethereum execution layer, following the specifications laid out in the Ethereum Yellow Paper.

## 🚀 Features

### Core Execution
- **Full Instruction Set**: Support for arithmetic, comparison, bitwise, stack, memory, and storage operations.
- **Control Flow**: Robust implementation of `JUMP`, `JUMPI`, and `JUMPDEST` with mandatory bytecode pre-scanning to prevent invalid jumps into data.
- **Gas Metering**: Precise static and dynamic gas calculation for all opcodes.
- **Security**: Panic-proof 256-bit arithmetic using `wrapping` and `overflowing` operations.

### Advanced Modules
- **Hashing**: `SHA3` (Keccak256) support for memory ranges using the `tiny-keccak` library.
- **Environmental Context**: Access to transaction and block data via `ADDRESS`, `CALLER`, `CALLVALUE`, and `GAS`.
- **Logging**: Implementation of `LOG0` through `LOG4` for event emission and off-chain indexing.

## 🛠 Project Structure

- **`bin/cli`**: A command-line interface for running EVM bytecode with support for hex strings, file inputs, and real-time execution tracing.
- **`crates/core`**: The heartbeat of the interpreter, containing the `Evm` engine, `Stack`, `Memory`, and `Storage` implementations.
- **`crates/shared`**: Shared types, opcode constants, and error definitions used across the workspace.

## 🚦 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Installation
```bash
git clone <repository-url>
cd evm-rs
cargo build
```

### Usage
Run bytecode directly from the terminal:
```bash
cargo run -p evm-cli -- run --code 0x6001600201
```

Run bytecode from a `.hex` file:
```bash
cargo run -p evm-cli -- run --file path/to/program.hex
```

Enable execution tracing to see the stack state per opcode:
```bash
cargo run -p evm-cli -- run --code 0x6001600201 --trace
```

## 🧪 Testing
The project includes an extensive suite of integration tests covering math, data flow, loops, and security boundaries.

```bash
cargo test --workspace
```

## 📝 Compliance
- **Arithmetic**: 256-bit wide words (via `primitive-types`).
- **Memory**: Linear, byte-addressable volatile memory.
- **Storage**: Key-value mapping for persistent contract state.
- **Environment**: Support for caller identity and execution context.
