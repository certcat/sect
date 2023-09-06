use clap::{Parser, Subcommand};
use tokio::main;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    server: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone, Debug)]
enum Command {
    AddChain {
        #[arg(short, long)]
        cert_path: String,
    },

    AddPreChain {
        #[arg(short, long)]
        precert_path: String,
    },

    GetSTH {},

    GetSTHConsistency {
        #[arg(short, long)]
        first: u64,
        #[arg(short, long)]
        second: u64,
    },

    GetProofByHash {
        #[arg(short = 'H', long)]
        hash: String,
        #[arg(short, long)]
        tree_size: u64,
    },

    GetEntries {
        #[arg(short, long)]
        start: u64,
        #[arg(short, long)]
        end: u64,
    },

    GetRoots {},

    GetEntryAndProof {
        #[arg(short, long)]
        leaf_index: u64,
        #[arg(short, long)]
        tree_size: u64,
    },
}

#[main]
async fn main() {
    let cli = Cli::parse();
    let server = &cli.server;

    let client = sect::client::CT::new(server).unwrap();

    let resp = match &cli.command {
        // TODO: need to load PEM from cert_path / precert_path
        Command::AddChain { cert_path } => client.add_chain(&[]).await,
        Command::AddPreChain { precert_path } => client.add_pre_chain(&[]).await,
        Command::GetSTH {} => client.get_sth().await,
        Command::GetSTHConsistency { first, second } => {
            client.get_sth_consistency(*first, *second).await
        }
        Command::GetProofByHash { hash, tree_size } => {
            client.get_proof_by_hash(hash, *tree_size).await
        }
        Command::GetEntries { start, end } => client.get_entries(*start, *end).await,
        Command::GetRoots {} => client.get_roots().await,
        Command::GetEntryAndProof {
            leaf_index,
            tree_size,
        } => client.get_entry_and_proof(*leaf_index, *tree_size).await,
    };

    match resp {
        Err(e) => println!("Error: {e}"),
        Ok(s) => println!("{s}"),
    }
}
