//
//    jevlor - extensible random word generator
//
//    Copyright (C) 2026  Fedor Veselov
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.
//

use std::path::{Component, Path, PathBuf};

use jevlor::generator::morphem::StatisticalMorphemGenerator;
use jevlor::generator::word::HomogenousWordGenerator;
use jevlor::serde::load_language;

use clap::Parser;
use commands::Cli;

use crate::error::*;

pub mod commands;
pub mod error;
pub mod format;

// Separate function for easier migration
fn print_notice() {
    eprintln!(
        "
    jevlor  Copyright (C) 2026  Fedor Veselov
    This program comes with ABSOLUTELY NO WARRANTY.
    This is free software, and you are welcome to redistribute it
    under certain conditions.
    "
    );
}

fn expand(path: &Path) -> PathBuf {
    let mut comps = path.components();
    let Some(Component::Normal(root)) = comps.next() else {
        return path.to_path_buf();
    };

    let Some(resolved_var) = std::env::var_os(root) else {
        return path.to_path_buf();
    };

    let mut buf = PathBuf::from(resolved_var);
    for c in comps {
        buf.push(c);
    }
    buf
}

fn resolve_paths<P: AsRef<Path>>(lookup: &[P]) -> Result<PathBuf, JevlorCliError> {
    for lookup_path in lookup.into_iter() {
        let path = expand(lookup_path.as_ref());
        if path.exists() {
            return Ok(path);
        }
    }
    Err(JevlorCliError::Generate(
        JevlorGenerateError::UnresolvedDefaultPath,
    ))
}

fn main() -> Result<(), JevlorCliError> {
    print_notice();
    let cli = Cli::parse();

    match cli.command {
        commands::Commands::Generate {
            path: working_directory,
            amount,
            morphems,
        } => {
            let lang_directory = working_directory
                .ok_or(JevlorCliError::Generate(
                    JevlorGenerateError::UnresolvedDefaultPath,
                ))
                .or(resolve_paths(&[
                    "JEVLOR_DICT",
                    "XDG_CONFIG_HOME/jevlor/languages/default",
                    "XDG_CONFIG_HOME/jevlor/lang/default",
                    "HOME/.config/jevlor/languages/default",
                    "HOME/.config/jevlor/lang/default",
                    "./languages/default",
                    "./lang/default",
                ]))?;

            let lang = load_language(lang_directory).unwrap();

            let word_gen: HomogenousWordGenerator<'_, StatisticalMorphemGenerator<'_, _>, _> =
                HomogenousWordGenerator::new(&lang);
            for word in word_gen.take(amount) {
                println!("{}", word,);
                if morphems {
                    println!("{}", format::format_word_morphems(&word));
                }
            }
        }
    }

    Ok(())
}
