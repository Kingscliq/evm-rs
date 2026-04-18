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
        Commands::Run { code, file, trace, .. } => {
            let bytecode = if let Some(hex_code) = code {
                // Remove 0x prefix if present
                let clean_hex = hex_code.strip_prefix("0x").unwrap_or(&hex_code);
                
                // Decode hex string into bytes
                match hex::decode(clean_hex) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        eprintln!("\x1b[31mError decoding hex string:\x1b[0m {}", e);
                        return;
                    }
                }
            } else if let Some(file_path) = file {
                // Read from file
                let content = match std::fs::read_to_string(&file_path) {
                    Ok(s) => s.trim().to_string(),
                    Err(e) => {
                        eprintln!("\x1b[31mError reading file '{}':\x1b[0m {}", file_path, e);
                        return;
                    }
                };
                
                let clean_hex = content.strip_prefix("0x").unwrap_or(&content);
                match hex::decode(clean_hex) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        eprintln!("\x1b[31mError decoding hex in file:\x1b[0m {}", e);
                        return;
                    }
                }
            } else {
                eprintln!("\x1b[33mNo bytecode provided. Use --code <HEX> or --file <PATH>\x1b[0m");
                return;
            };

            println!("\x1b[36mRunning EVM bytecode...\x1b[0m");
                
                let mut evm = Evm::new(bytecode, 100000); // 100k gas limit
                evm.trace = trace;
                
                let initial_gas = evm.gas_remaining;
                let result = evm.run();

                println!("---------------------------------");
                let stack_hex = evm.stack.to_hex_strings();
                println!("Stack:    {:?}", stack_hex);
                println!("Return:   0x"); 
                println!("Gas used: {}", initial_gas - evm.gas_remaining);
                
                match result {
                    Ok(_) => {
                        println!("Status:   STOP");
                    }
                    Err(e) => {
                        println!("Status:   FAILED ({})", e);
                    }
                }
                println!("---------------------------------");
        }
    }
}
