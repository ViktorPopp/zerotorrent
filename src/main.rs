use clap::{Parser, Subcommand, command};
use std::path::PathBuf;
use zerotorrent::{
    torrent::Torrent,
    tracker::{TrackerRequest, TrackerResponse},
};

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
    #[command(about = "Lists all peers returned by the tracker")]
    Peers {
        #[arg()]
        path: PathBuf,
    },
}

#[tokio::main]
async fn main() {
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
        Commands::Peers { path } => {
            let file_data = std::fs::read(path).expect("Failed to read torrent file");
            let t: Torrent =
                serde_bencode::from_bytes(&file_data).expect("Failed to parse torrent file");

            let request = TrackerRequest {
                peer_id: String::from("00112233445566778899"),
                port: 6881,
                uploaded: 0,
                downloaded: 0,
                left: t.info.length,
                compact: 1,
            };

            let url_parameters = serde_urlencoded::to_string(&request)
                .expect("Failed URL-encode tracker parameters");
            let tracker_url = format!(
                "{}?{}&info_hash={}",
                t.announce,
                url_parameters,
                &urlencode(&t.info_hash())
            );

            let response = reqwest::get(tracker_url)
                .await
                .expect("Failed to query tracker response");
            let response = response
                .bytes()
                .await
                .expect("Failed to fetch tracker response");
            let response: TrackerResponse =
                serde_bencode::from_bytes(&response).expect("Failed to parse tracker response");
            for peer in &response.peers.0 {
                println!("{}:{}", peer.ip(), peer.port());
            }
        }
    }
}

fn urlencode(t: &[u8; 20]) -> String {
    let mut encoded = String::with_capacity(3 * t.len());
    for &byte in t {
        encoded.push('%');
        encoded.push_str(&hex::encode_upper(&[byte]));
    }
    encoded
}
