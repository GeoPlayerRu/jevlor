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
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start process of word generation.
    Generate {
        /// Amount of words to generate
        #[arg(default_value_t = 10)]
        amount: usize,

        /// Path to language directory. Either relative or absolute.
        ///
        /// If left empty, it will search in directories in order:
        ///
        /// $JEVLOR_DICT
        ///
        /// $XDG_CONFIG_HOME/jevlor/languages/default
        ///
        /// $XDG_CONFIG_HOME/jevlor/lang/default
        ///
        /// $HOME/config/jevlor/languages/default
        ///
        /// $HOME/config/jevlor/lang/default
        ///
        /// ./jevlor/languages/default
        ///
        /// ./jevlor/lang/default
        path: Option<PathBuf>,

        /// Prints morphems of word right after word
        ///
        /// Format: Prefixes:[]?;Roots:[];Suffixes:[]?;Ending:?;Postfix:?
        ///
        /// Question mark '?' means that entire entry will not appear if morphem does not exist
        #[arg(short, long)]
        morphems: bool,
    },
}
