use crate::hashes::Hashes;
use serde::Deserialize;

pub mod bencode;
pub mod hashes;

/// See: https://bittorrent.org/beps/bep_0003.html#metainfo-files
#[derive(Debug, Clone, Deserialize)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

/// See: https://bittorrent.org/beps/bep_0003.html#info-dictionary
#[derive(Debug, Clone, Deserialize)]
pub struct Info {
    pub name: String,
    #[serde(rename = "piece length")]
    pub piece_length: usize,
    pub pieces: Hashes,
    pub length: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct File {
    pub length: usize,
    /// Subdirectory names for this file, the last of which is the actual file name
    pub path: Vec<String>,
}
