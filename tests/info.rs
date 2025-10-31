use zerotorrent::torrent::Torrent;

#[test]
fn info() {
    let file_data = std::fs::read("sample.torrent").expect("Failed to read torrent file");
    let t: Torrent = serde_bencode::from_bytes(&file_data).expect("Failed to parse torrent file");

    assert_eq!(
        t.announce,
        "http://bittorrent-test-tracker.codecrafters.io/announce"
    );
    assert_eq!(t.info.length, 92063);
    assert_eq!(
        hex::encode(&t.info_hash()),
        "d69f91e6b2ae4c542468d1073a71d4ea13879a7f"
    );
    assert_eq!(t.info.piece_length, 32768);

    let expected_hashes = vec![
        "e876f67a2a8886e8f36b136726c30fa29703022d",
        "6e2275e604a0766656736e81ff10b55204ad8d35",
        "f00d937a0213df1982bc8d097227ad9e909acc17",
    ];
    let actual_hashes: Vec<String> = t.info.pieces.0.iter().map(|h| hex::encode(h)).collect();
    assert_eq!(actual_hashes, expected_hashes);
}
