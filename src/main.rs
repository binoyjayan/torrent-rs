use clap::Parser;
use serde_bencode;

mod cli;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    match encoded_value.chars().next().unwrap() {
        'i' => {
            // Numbers are encoded as i<number>e
            let number: i64 = serde_bencode::from_bytes(encoded_value.as_bytes()).unwrap();
            serde_json::Value::Number(serde_json::Number::from(number))
        }
        'l' => {
            // Lists are encoded as l<value1><value2>...e
            serde_json::Value::Null
        }
        'd' => {
            // Dictionaries are encoded as d<key1><value1><key2><value2>...e
            serde_json::Value::Null
        }
        digit => {
            // Strings are encoded as <length>:<string>
            if digit.is_ascii_digit() {
                let colon_index = encoded_value.find(':').unwrap();
                let number_string = &encoded_value[..colon_index];
                let number = number_string.parse::<i64>().unwrap();
                let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
                serde_json::Value::String(string.to_string())
            } else {
                panic!("Unhandled encoded value: {}", encoded_value)
            }
        }
    }
}

fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        Some(cli::Commands::Decode { encoded_value }) => {
            let decoded_value = decode_bencoded_value(&encoded_value);
            println!("{}", decoded_value);
        }
        None => {
            println!("unknown command");
        }
    }
}
