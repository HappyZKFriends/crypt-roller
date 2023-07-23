use clap::Parser;
use clap::Subcommand;

#[derive(Subcommand)]
enum SequencerCommands {
    /// Create a new block and push it on chain
    Push,

    /// Pull new accounts from the contract
    Pull,
}

#[derive(Subcommand)]
enum WalletCommands {
    /// Create a new account in the rollup by communicating with the contract
    Enter,

    /// Exit the rollup and destroy the account
    Exit,

    /// Transfer funds to a different account in the rollup
    Transfer { amount: u128, to: String },
}

#[derive(Subcommand)]
enum Commands {
    /// Commands for managing state of the whole rollup
    Sequencer {
        #[command(subcommand)]
        command: SequencerCommands,
    },

    /// Commands for managing a user account
    Wallet {
        #[command(subcommand)]
        command: WalletCommands,
    },
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Sequencer { command } => {
            match command {
                SequencerCommands::Push => {
                    // TODO: Implement
                    println!("PUSH")
                }
                SequencerCommands::Pull => {
                    // TODO: Implement
                    println!("PULL")
                }
            }
        }
        Commands::Wallet { command } => {
            match command {
                WalletCommands::Enter => {
                    // TODO: Implement
                    println!("ENTER")
                }
                WalletCommands::Exit => {
                    // TODO: Implement
                    println!("EXIT")
                }
                WalletCommands::Transfer { amount, to } => {
                    // TODO: Implement
                    println!("Transferring {amount} to {to}.")
                }
            }
        }
    }
}
