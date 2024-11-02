use clap::Parser;

#[derive(Debug, Clone, Copy, strum::EnumString)]
pub enum LangType {
    American,
    British,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// English variant to use. Allowed: American, British
    #[arg(short = 'L', long = "language", default_value = "American")]
    pub language: LangType,
    /// Word length.
    #[arg(short = 'l', long = "length")]
    pub word_len: u8,
}
