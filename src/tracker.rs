//! See: https://bittorrent.org/beps/bep_0003.html#trackers

use serde::{Deserialize, Serialize};

use crate::peers::Peers;

#[derive(Debug, Clone, Serialize)]
pub struct TrackerRequest {
    pub peer_id: String,
    pub port: u16,
    pub uploaded: usize,
    pub downloaded: usize,
    pub left: usize,
    pub compact: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrackerResponse {
    pub interval: usize,
    pub peers: Peers,
}
