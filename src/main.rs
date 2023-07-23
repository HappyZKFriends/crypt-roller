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
enum UserCommands {
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
    User {
        #[command(subcommand)]
        command: UserCommands,
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
        Commands::User { command } => {
            match command {
                UserCommands::Enter => {
                    // TODO: Implement
                    println!("ENTER")
                }
                UserCommands::Exit => {
                    // TODO: Implement
                    println!("EXIT")
                }
                UserCommands::Transfer { amount, to } => {
                    // TODO: Implement
                    println!("Transferring {amount} to {to}.")
                }
            }
        }
    }
}
