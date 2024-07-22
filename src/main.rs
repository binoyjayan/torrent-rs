use clap::Parser;
use serde_json;

// use serde_bencode

mod cli;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    // If encoded_value starts with a digit, it's a number
    if encoded_value.chars().next().unwrap().is_digit(10) {
        // Example: "5:hello" -> "hello"
        let colon_index = encoded_value.find(':').unwrap();
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<i64>().unwrap();
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
        return serde_json::Value::String(string.to_string());
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
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
