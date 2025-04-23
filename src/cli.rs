use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum DoclessOutputMode {
    /// CSV Output
    CSV,

    /// Display as a table
    Tabular,

    /// Output simple JSON
    JSON,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {

    /// Do not attempt to read EDI into specific document types. Just show individual segments in input order.

    #[arg(long, short)]
    pub docless: bool,

    /// Optional name to operate on
    #[arg(long)]
    pub docless_output_mode: Option<DoclessOutputMode>,

    /// Specify EDI files to read. Read from STDIN by default.
    #[arg(value_name = "INPUT_FILE")]
    pub input_files: Option<Vec<PathBuf>>,

}
