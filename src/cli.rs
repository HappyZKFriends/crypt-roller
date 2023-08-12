use clap::Parser;
use clap::Subcommand;

use crate::node::Node;
use crate::node::NodeError;
use crate::sequencer::build_transaction_batch;
use crate::transaction::Address;
use crate::transaction::Amount;
use crate::wallet::Wallet;
use crate::wallet::WalletError;

#[derive(Debug)]
pub enum CLIError {
    Wallet(WalletError),
    Node(NodeError),
}

#[derive(Subcommand)]
enum NodeCommands {
    /// Print all transactions from all batches, chronologically
    History,

    /// Print all transactions from the mempool, in no particular order
    Mempool,

    /// Print current state of the rollup, including balances of all accounts
    State,
}

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
    Enter { amount: Amount },

    /// Exit the rollup and destroy the account
    Exit,

    // TODO: Persist the wallet so that 'from' does not have to be given explicitly
    /// Transfer funds to a different account in the rollup
    Transfer {
        from: Address,
        to: Address,
        amount: Amount,
    },
}

#[derive(Subcommand)]
enum Commands {
    /// Commands for managing the rollup node
    Node {
        #[command(subcommand)]
        command: NodeCommands,
    },

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

pub fn run_cli() -> Result<(), CLIError> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Node { command } => match command {
            NodeCommands::History => {
                println!("{:#?}", Node::start().map_err(CLIError::Node)?.history)
            }
            NodeCommands::Mempool => {
                println!("{:#?}", Node::start().map_err(CLIError::Node)?.mempool)
            }
            NodeCommands::State => {
                println!("{:#?}", Node::start().map_err(CLIError::Node)?.state)
            }
        },
        Commands::Sequencer { command } => {
            match command {
                SequencerCommands::Push => {
                    let mut node = Node::start().map_err(CLIError::Node)?;
                    let batch = build_transaction_batch(&node);
                    if batch.is_empty() {
                        println!("No transactions to publish found.")
                    } else {
                        node.history.publish_batch(batch);
                    }
                    node.update_storage().map_err(CLIError::Node)?;
                }
                SequencerCommands::Pull => {
                    // TODO: Implement
                    println!("PULL");
                }
            }
        }
        Commands::Wallet { command } => {
            match command {
                WalletCommands::Enter { amount } => {
                    let mut node = Node::start().map_err(CLIError::Node)?;
                    let (_wallet, transaction) = Wallet::build_enter_transaction(*amount, &node)
                        .map_err(CLIError::Wallet)?;
                    node.mempool.publish_transaction(transaction);
                    node.update_storage().map_err(CLIError::Node)?;
                }
                WalletCommands::Exit => {
                    // TODO: Implement
                    println!("EXIT");
                }
                WalletCommands::Transfer { from, to, amount } => {
                    let mut node = Node::start().map_err(CLIError::Node)?;
                    let wallet = Wallet { account: *from };

                    let transaction = wallet
                        .build_transfer_transaction(*to, *amount, &node)
                        .map_err(CLIError::Wallet)?;
                    node.mempool.publish_transaction(transaction);
                    node.update_storage().map_err(CLIError::Node)?;
                }
            }
        }
    }

    Ok(())
}
