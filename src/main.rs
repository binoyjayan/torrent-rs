use clap::Parser;

mod cli;
mod decode;
mod torrent;

use decode::*;
use torrent::*;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        Some(cli::Commands::Decode { encoded_value }) => {
            let decoded_value = decode_bencoded_data(&encoded_value)?;
            println!("{}", decoded_value);
        }
        Some(cli::Commands::Info { file_path }) => {
            let decoded_value = read_torrent_info(&file_path)?;
            println!("Tracker URL: {}", decoded_value.announce);
            println!("Length: {}", decoded_value.info.length);
        }
        None => {
            anyhow::bail!("unknown command");
        }
    }
    Ok(())
}
