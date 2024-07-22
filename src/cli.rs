use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(clap::Subcommand)]
pub(crate) enum Commands {
    /// Decode a bencoded value
    Decode {
        /// The bencoded value to decode
        encoded_value: String,
    },
    Info {
        /// The path to the torrent file
        file_path: String,
    },
}
