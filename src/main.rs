use clap::{command, Parser, Subcommand};
use std::path::PathBuf;
use zerotorrent::torrent::Torrent;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Displays information about a torrent")]
    Info {
        #[arg()]
        path: PathBuf,
    },
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Commands::Info { path } => {
            let file_data = std::fs::read(path).expect("Failed to read torrent file");
            let t: Torrent =
                serde_bencode::from_bytes(&file_data).expect("Failed to parse torrent file");
            println!("Tracker URL:\t{}", t.announce);
            println!("Length:\t\t{}", t.info.length);
            println!("Info Hash:\t{}", hex::encode(&t.info_hash()));
            println!("Piece Length:\t{}", t.info.piece_length);
            println!("--- Begin Piece Hashes ---");
            for hash in t.info.pieces.0 {
                println!("{}", hex::encode(&hash));
            }
            println!("---- End Piece Hashes ----")
        }
    }
}
