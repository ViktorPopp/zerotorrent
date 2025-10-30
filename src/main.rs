use clap::{command, Parser, Subcommand};
use sha1::{Digest, Sha1};
use std::path::PathBuf;
use zerotorrent::{bencode, torrent::Torrent};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Decodes a Bencoded string")]
    Decode {
        #[arg()]
        value: String,
    },
    #[command(about = "Displays information about a torrent")]
    Info {
        #[arg()]
        path: PathBuf,
    },
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Commands::Decode { value } => {
            let (value, rest) = bencode::decode_value(value);
            println!("Decoded Value: {value}");
            if !rest.is_empty() {
                println!("Remainder:     \"{rest}\"");
            }
        }
        Commands::Info { path } => {
            let torrent_data = std::fs::read(path).expect("Failed to read torrent file");
            let t: Torrent =
                serde_bencode::from_bytes(&torrent_data).expect("Failed to parse torrent file");
            println!("Tracker URL:  {}", t.announce);
            println!("Length:       {}", t.info.length);

            // Find info hash
            let info_encoded =
                serde_bencode::to_bytes(&t.info).expect("Failed to re-encode info section");
            let mut hasher = Sha1::new();
            hasher.update(&info_encoded);
            let info_hash = hasher.finalize();
            println!("Info Hash:    {}", hex::encode(&info_hash));

            // Info about pieces
            println!("Piece Length: {}", t.info.piece_length);
            println!("Piece Hashes:");
            for hash in t.info.pieces.0 {
                println!("{}", hex::encode(hash));
            }
        }
    }
}
