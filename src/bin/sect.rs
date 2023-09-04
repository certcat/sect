use clap::{Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();
    let server = &cli.server;

    match &cli.command {
        Command::AddChain { cert_path } => {
            println!("POST https://{server}/ct/v1/add-chain with data from {cert_path}")
        }
        Command::AddPreChain { precert_path } => {
            println!("POST https://{server}/ct/v1/add-pre-chain with data from {precert_path}")
        }
        Command::GetSTH {} => {
            println!("GET https://{server}/ct/v1/get-sth")
        }
        Command::GetSTHConsistency { first, second } => {
            println!("GET https://{server}/ct/v1/get-sth-consistency?first={first}&second={second}")
        }
        Command::GetProofByHash { hash, tree_size } => {
            println!(
                "GET https://{server}/ct/v1/get-proof-by-hash?hash={hash}&tree_size={tree_size}"
            )
        }
        Command::GetEntries { start, end } => {
            println!("GET https://{server}/ct/v1/get-entries?start={start}&end={end}")
        }
        Command::GetRoots {} => {
            println!("GET https://{server}/ct/v1/get-roots")
        }
        Command::GetEntryAndProof {
            leaf_index,
            tree_size,
        } => {
            println!("GET https://{server}/ct/v1/get-entry-and-proof?leaf_index={leaf_index}&tree_size={tree_size}")
        }
    }
}
