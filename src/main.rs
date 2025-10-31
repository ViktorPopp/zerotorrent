use clap::{Parser, Subcommand, command};
use std::{net::SocketAddrV4, path::PathBuf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use zerotorrent::{
    peers::Handshake,
    torrent::Torrent,
    tracker::{TrackerRequest, TrackerResponse},
    urlencode::urlencode,
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
    #[command(about = "Performs a handshake with a peer")]
    Handshake {
        #[arg()]
        path: PathBuf,
        peer: String,
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
        Commands::Handshake { path, peer } => {
            let file_data = std::fs::read(path).expect("Failed to read torrent file");
            let t: Torrent =
                serde_bencode::from_bytes(&file_data).expect("Failed to parse torrent file");

            let peer = peer
                .parse::<SocketAddrV4>()
                .expect("Failed to parse peer address");
            let mut peer = tokio::net::TcpStream::connect(peer)
                .await
                .expect("Failed to connect to peer");

            let mut handshake = Handshake::new(t.info_hash(), *b"00112233445566778899");
            {
                let handshake_bytes =
                    &mut handshake as *mut Handshake as *mut [u8; std::mem::size_of::<Handshake>()]; // "Treat the memory of this `Handshake` struct as if it were an array of bytes”
                // Safety: `Handshake` is using repr(C)
                let handshake_bytes: &mut [u8; std::mem::size_of::<Handshake>()] =
                    unsafe { &mut *handshake_bytes };

                peer.write_all(handshake_bytes)
                    .await
                    .expect("Failed the write handshake bytes");
                peer.read_exact(handshake_bytes)
                    .await
                    .expect("Failed to read handshake bytes");
            }
            assert_eq!(handshake.length, 19);
            assert_eq!(&handshake.bittorrent, b"BitTorrent protocol");
            println!("Peer ID: {}", hex::encode(&handshake.peer_id));
        }
    }
}
