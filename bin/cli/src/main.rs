use clap::{Parser, Subcommand};
// use evm_core::Evm;
// use hex;

#[derive(Parser)]
#[command(name = "evm")]
#[command(about = "A custom Ethereum Virtual Machine interpreter", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run EVM bytecode
    Run {
        /// Bytecode as a hex string (e.g., 0x6001600201)
        #[arg(long)]
        code: Option<String>,

        /// Path to a file containing raw bytecode
        #[arg(long)]
        file: Option<String>,

        /// Enable per-opcode execution trace
        #[arg(long)]
        trace: bool,
    },
}

fn main() {
    todo!("Task 06: Parse CLI args, pass the hex to Evm::new(), and print the final Stack/Gas status.")
}
