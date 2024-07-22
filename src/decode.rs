use serde_bencode::value::Value as BencodeValue;
use serde_json::Value as JsonValue;

pub(crate) fn decode_bencoded_int(value: &BencodeValue) -> anyhow::Result<u64> {
    match value {
        BencodeValue::Int(i) => Ok(*i as u64),
        _ => anyhow::bail!("expected integer"),
    }
}

pub(crate) fn decode_bencoded_str(value: &BencodeValue) -> anyhow::Result<String> {
    match value {
        BencodeValue::Bytes(b) => Ok(String::from_utf8(b.to_vec())?),
        _ => anyhow::bail!("expected integer"),
    }
}

pub(crate) fn decode_bencoded_bytes(value: &BencodeValue) -> anyhow::Result<Vec<u8>> {
    match value {
        BencodeValue::Bytes(b) => Ok(b.to_vec()),
        _ => anyhow::bail!("expected bytes"),
    }
}

pub(crate) fn decode_bencoded_value(value: &BencodeValue) -> anyhow::Result<JsonValue> {
    Ok(match value {
        BencodeValue::Bytes(b) => {
            let string = String::from_utf8(b.to_vec())?;
            JsonValue::String(string)
        }
        BencodeValue::Int(i) => JsonValue::Number((*i).into()),
        BencodeValue::List(l) => {
            let array = l
                .iter()
                .map(decode_bencoded_value)
                .collect::<anyhow::Result<Vec<serde_json::Value>>>()?;
            JsonValue::Array(array)
        }
        BencodeValue::Dict(d) => {
            let object = d
                .iter()
                .map(|(k, v)| {
                    let key = String::from_utf8(k.to_vec())?;
                    let value = decode_bencoded_value(v)?;
                    Ok((key, value))
                })
                .collect::<anyhow::Result<serde_json::Map<String, serde_json::Value>>>()?;
            JsonValue::Object(object)
        }
    })
}

pub(crate) fn decode_bencoded_data(encoded_value: &str) -> anyhow::Result<serde_json::Value> {
    let value: BencodeValue = serde_bencode::from_str(encoded_value)?;

    decode_bencoded_value(&value)
}
