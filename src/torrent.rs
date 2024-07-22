use serde_bencode::value::Value as BencodeValue;

use crate::{
    decode_bencoded_bytes, decode_bencoded_int, decode_bencoded_str, decode_bencoded_value,
};

#[derive(Debug)]
pub(crate) struct Torrent {
    pub(crate) announce: String,
    pub(crate) info: TorrentInfo,
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct TorrentInfo {
    pub(crate) length: u64,
    pub(crate) name: String,
    pub(crate) piece_length: u64,
    pub(crate) pieces: Vec<u8>,
}

pub(crate) fn read_torrent_info(file_path: &str) -> anyhow::Result<Torrent> {
    let content = std::fs::read(file_path)?;
    let value: BencodeValue = serde_bencode::from_bytes(content.as_slice())?;

    if let BencodeValue::Dict(ref d) = value {
        let mut announce = None;
        let mut info = None;

        for (k, tval) in d {
            let key = String::from_utf8(k.to_vec())?;
            match key.as_str() {
                "announce" => {
                    let value = decode_bencoded_value(tval)?;
                    announce = value.as_str().map(|s| s.to_string());
                }
                "info" => {
                    if let BencodeValue::Dict(ref d) = tval {
                        let mut length = None;
                        let mut name = None;
                        let mut piece_length = None;
                        let mut pieces = None;

                        for (k, v) in d {
                            let key = String::from_utf8(k.to_vec())?;
                            match key.as_str() {
                                "length" => length = Some(decode_bencoded_int(v)?),
                                "name" => name = Some(decode_bencoded_str(v)?),
                                "piece length" => piece_length = Some(decode_bencoded_int(v)?),
                                "pieces" => pieces = Some(decode_bencoded_bytes(v)?),
                                _ => {}
                            }
                        }

                        if let (Some(length), Some(name), Some(piece_length), Some(pieces)) =
                            (length, name, piece_length, pieces)
                        {
                            info = Some(TorrentInfo {
                                length,
                                name,
                                piece_length,
                                pieces,
                            });
                        }
                    }
                }
                _ => {}
            }
        }

        if let (Some(announce), Some(info)) = (announce, info) {
            Ok(Torrent { announce, info })
        } else {
            anyhow::bail!("invalid torrent file: invalid announce or info")
        }
    } else {
        anyhow::bail!("invalid torrent file")
    }
}
