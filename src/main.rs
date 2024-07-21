use clap::Parser;
use serde_bencode::value::Value as BencodeValue;
use serde_json::Value as JsonValue;

mod cli;

fn decode_bencoded_value(value: BencodeValue) -> anyhow::Result<JsonValue> {
    Ok(match value {
        BencodeValue::Bytes(b) => {
            let string = String::from_utf8(b)?;
            JsonValue::String(string)
        }
        BencodeValue::Int(i) => JsonValue::Number(i.into()),
        BencodeValue::List(l) => {
            let array = l
                .into_iter()
                .map(decode_bencoded_value)
                .collect::<anyhow::Result<Vec<serde_json::Value>>>()?;
            JsonValue::Array(array)
        }
        _ => anyhow::bail!("Unhandled encoded value: {:?}", value),
    })
}

fn decode_bencoded_str(encoded_value: &str) -> anyhow::Result<serde_json::Value> {
    let value: BencodeValue = serde_bencode::from_str(encoded_value)?;

    decode_bencoded_value(value)
}

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        Some(cli::Commands::Decode { encoded_value }) => {
            let decoded_value = decode_bencoded_str(&encoded_value)?;
            println!("{}", decoded_value);
        }
        None => {
            anyhow::bail!("unknown command");
        }
    }
    Ok(())
}
