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

use evm_core::Evm;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { code, .. } => {
            if let Some(hex_code) = code {
                // Remove 0x prefix if present
                let clean_hex = hex_code.strip_prefix("0x").unwrap_or(&hex_code);
                
                // Decode hex string into bytes
                let bytecode = match hex::decode(clean_hex) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        eprintln!("\x1b[31mError decoding bytecode:\x1b[0m {}", e);
                        return;
                    }
                };

                println!("\x1b[36mRunning EVM bytecode...\x1b[0m");
                
                let mut evm = Evm::new(bytecode, 100000); // 100k gas limit
                
                match evm.run() {
                    Ok(_) => {
                        println!("\x1b[32mExecution successful!\x1b[0m");
                        println!("---------------------------------");
                        println!("Final Stack: {:?}", evm.stack);
                        println!("Gas Remaining: {} units", evm.gas_remaining);
                        println!("---------------------------------");
                    }
                    Err(e) => {
                        eprintln!("\x1b[31mExecution failed:\x1b[0m {}", e);
                    }
                }
            } else {
                eprintln!("\x1b[33mNo bytecode provided. Use --code <HEX>\x1b[0m");
            }
        }
    }
}
