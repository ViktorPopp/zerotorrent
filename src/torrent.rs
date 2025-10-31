use crate::hashes::Hashes;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

/// See: https://bittorrent.org/beps/bep_0003.html#metainfo-files
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

impl Torrent {
    pub fn info_hash(&self) -> [u8; 20] {
        let info_encoded =
            serde_bencode::to_bytes(&self.info).expect("Failed to re-encode info section");
        let mut hasher = Sha1::new();
        hasher.update(&info_encoded);
        hasher
            .finalize()
            .try_into()
            .expect("Failed to convert info section to hash")
    }
}

/// See: https://bittorrent.org/beps/bep_0003.html#info-dictionary
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Info {
    name: String,
    #[serde(rename = "piece length")]
    pub piece_length: usize,
    pub pieces: Hashes,
    pub length: usize,
}
