use clap::{Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// dictionary file, each line contains a single word if it's not empty
    #[arg(
        short = 'd',
        long = "dict",
        default_value = "/usr/share/dict/american-english",
        value_hint = ValueHint::FilePath,
    )]
    pub dict_file: PathBuf,
    #[arg(
        short = 'l',
        long = "length",
    )]
    pub word_len: u8,
}
