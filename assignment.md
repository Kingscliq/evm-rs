# Assessment — Build a Working EVM Binary

**Course:** Blockchain Engineering  
**Type:** Individual Assignment  
**Language:** Rust (required)  
**Deliverable:** CLI binary  
**Total Points:** 100  
**Due:** 7 days from release

---

## Background

The Ethereum Virtual Machine (EVM) is a stack-based, 256-bit virtual machine that executes smart contract bytecode deterministically across all Ethereum nodes. Every Solidity contract compiles down to a sequence of opcodes the EVM interprets one-by-one. This week you will build that interpreter from scratch — in Rust.

> [!IMPORTANT]
> **Rule:** You may **not** use an existing EVM library (e.g. `revm`, `evm`, `etk`) as a dependency. Your implementation must be built from the ground up.

---

## Tasks

### Task 01 — EVM Core: Stack Machine & Execution Loop [25 pts]

Implement the foundational EVM execution environment.

- **256-bit word stack** (max 1024 items) with push/pop/peek — use the `uint` or `primitive-types` crate for `U256`, or implement your own.
- **Volatile memory model** (byte-addressable, expandable with zero-padding).
- **Persistent storage model** (key-value: `U256` -> `U256`).
- **Program counter** (`PC`) and a main fetch-decode-execute loop.
- **Gas accounting:** every opcode must deduct the correct **static** gas cost per the Yellow Paper.

### Task 02 — Arithmetic & Comparison Opcodes [15 pts]

Implement all arithmetic and bitwise opcodes with correct 256-bit wrapping/overflow semantics.

| Group | Opcodes |
| :--- | :--- |
| **Arithmetic** | `ADD`, `MUL`, `SUB`, `DIV`, `SDIV`, `MOD`, `SMOD`, `ADDMOD`, `MULMOD`, `EXP`, `SIGNEXTEND` |
| **Comparison** | `LT`, `GT`, `SLT`, `SGT`, `EQ`, `ISZERO` |
| **Bitwise** | `AND`, `OR`, `XOR`, `NOT`, `BYTE`, `SHL`, `SHR`, `SAR` |

> [!TIP]
> Pay special attention to **signed** variants (`SDIV`, `SMOD`, `SLT`, `SGT`, `SAR`) and two's-complement edge cases.

### Task 03 — Stack, Memory & Storage Opcodes [20 pts]

Implement the full set of data manipulation opcodes, including all PUSH/DUP/SWAP variants.

| Group | Opcodes |
| :--- | :--- |
| **Stack** | `PUSH1` – `PUSH32`, `POP`, `DUP1` – `DUP16`, `SWAP1` – `SWAP16` |
| **Memory** | `MLOAD`, `MSTORE`, `MSTORE8`, `MSIZE`, `MCOPY` |
| **Storage** | `SLOAD`, `SSTORE` |
| **Calldata** | `CALLDATALOAD`, `CALLDATASIZE`, `CALLDATACOPY` |
| **Code** | `CODESIZE`, `CODECOPY`, `EXTCODESIZE`, `EXTCODECOPY`, `EXTCODEHASH` |
| **Return data** | `RETURNDATASIZE`, `RETURNDATACOPY` |

### Task 04 — Control Flow & Environment Opcodes [20 pts]

Implement branching, halting, and block/transaction context opcodes.

| Group | Opcodes |
| :--- | :--- |
| **Control flow** | `JUMP`, `JUMPI`, `JUMPDEST`, `PC`, `STOP`, `RETURN`, `REVERT`, `INVALID` |
| **Transaction context** | `ADDRESS`, `BALANCE`, `ORIGIN`, `CALLER`, `CALLVALUE`, `GASPRICE`, `GAS` |
| **Block context** | `BLOCKHASH`, `COINBASE`, `TIMESTAMP`, `NUMBER`, `PREVRANDAO`, `GASLIMIT`, `CHAINID`, `SELFBALANCE`, `BASEFEE` |

> [!NOTE]
> For `JUMPDEST` validation: pre-scan the bytecode at load time and build a `HashSet<usize>` of valid jump destinations — do not validate at runtime.

### Task 05 — Hashing, Logging & System Opcodes [10 pts]

Implement the remaining opcodes including cryptographic hashing, event logging, and contract lifecycle calls.

| Group | Opcodes |
| :--- | :--- |
| **Hashing** | `SHA3` / `KECCAK256` |
| **Logging** | `LOG0`, `LOG1`, `LOG2`, `LOG3`, `LOG4` |
| **Calls** | `CALL`, `CALLCODE`, `DELEGATECALL`, `STATICCALL` |
| **Contract** | `CREATE`, `CREATE2`, `SELFDESTRUCT` |

> [!NOTE]
> For `CALL` and friends, a stub implementation that correctly handles the stack inputs/outputs and gas deduction is acceptable if you do not implement a full nested execution context.

### Task 06 — CLI Interface & Test Suite [10 pts]

Build a usable binary and prove correctness with tests.

#### CLI requirements:

```bash
# Run bytecode from a hex string
evm run --code 0x6001600201

# Run bytecode from a file
evm run --file contract.bin

# Enable per-opcode execution trace
evm run --code 0x6001600201 --trace
```

#### Expected output on exit:

```text
Stack:    [0x3]
Return:   0x
Gas used: 6
Status:   STOP
```

#### Test suite requirements:

- Minimum **10 test cases** using Rust's built-in `#[test]` framework.
- Must cover: stack overflow, integer overflow/wrapping, gas exhaustion, invalid jump destination, `REVERT` with return data, and at least one multi-opcode program.
- Run with `cargo test`.

---

## Grading Rubric

| Criteria | Points |
| :--- | :--- |
| Core stack machine correctness | 25 |
| Arithmetic & bitwise opcode accuracy (256-bit edge cases) | 15 |
| Stack, memory & storage opcodes | 20 |
| Control flow & environment opcodes | 20 |
| Hashing, logging & system opcodes | 10 |
| CLI usability & test coverage | 10 |
| **Total** | **100** |

---

## Resources

- [Ethereum Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf) — Appendix H has the full opcode table with gas costs
- [ethereum/execution-specs](https://github.com/ethereum/execution-specs) — authoritative reference for opcode behavior and edge cases
- [evm.codes](https://www.evm.codes/) — interactive opcode reference with examples
- `primitive-types` crate — for `U256` if you don't roll your own
- `tiny-keccak` crate — for `KECCAK256`

---

## Submission Requirements

Submit a link to a **public GitHub repository** containing:

- [ ] Full Rust source (`cargo build --release` must succeed cleanly)
- [ ] `README.md` with build instructions and a short description of your architecture
- [ ] Passing test suite (`cargo test` output included in the README)
- [ ] At least one example `.bin` file demonstrating your EVM running real bytecode

### Repo structure suggestion:

```text
evm/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs      # CLI entry point
    ├── evm.rs       # Core execution loop
    ├── opcodes.rs   # Opcode definitions & dispatch
    ├── stack.rs     # Stack implementation
    ├── memory.rs    # Memory implementation
    └── storage.rs   # Storage implementation
└── tests/
    └── integration.rs
```
