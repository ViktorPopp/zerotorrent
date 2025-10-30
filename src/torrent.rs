use crate::hashes::Hashes;
use serde::{Deserialize, Serialize};

/// See: https://bittorrent.org/beps/bep_0003.html#metainfo-files
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

/// See: https://bittorrent.org/beps/bep_0003.html#info-dictionary
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    #[serde(rename = "piece length")]
    pub piece_length: usize,
    pub pieces: Hashes,
    pub length: usize,
}
