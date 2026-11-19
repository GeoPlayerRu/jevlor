use std::path::PathBuf;

use clap::{Parser, Subcommand};

fn about_commands() -> String {
    String::from(
        "Word generator based on different approaches to create words. Currently features statistical homogenous word generator.",
    )
}

#[derive(Parser)]
#[command(about = about_commands(), version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to language directory. Either relative or absolute.
    ///
    /// If left empty, it will search for directory 'default' in directories in order listed below.
    /// If implicitly set, it will search for value in reverse order of a listed below.
    ///
    /// Order of search:
    /// - $JEVLOR_DICT/
    /// - $XDG_CONFIG_HOME/jevlor/languages/
    /// - $XDG_CONFIG_HOME/jevlor/lang/
    /// - $HOME/config/jevlor/languages/
    /// - $HOME/config/jevlor/lang/
    /// - ./languages/
    /// - ./lang/
    ///
    pub path: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start process of word generation.
    Words {
        /// Amount of words to generate
        #[arg(default_value_t = 10)]
        amount: usize,

        /// Prints morphems of word right after word
        ///
        /// Format: Prefixes:[]?;Roots:[];Suffixes:[]?;Ending:?;Postfix:?
        ///
        /// Question mark '?' means that entire entry will not appear if morphem does not exist
        #[arg(short, long)]
        split_moprhems: bool,
    },
}
