/*
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {}

fn main() {
    let args = Args::parse();
}
*/

use std::fs::File;
use std::io::Read;
use std::path::Path;

use jevlor::generator::morphem::StatisticalMorphemGenerator;
use jevlor::language;
use jevlor::language::morphem::{MorphemConcatRule, MorphemicRules};
use jevlor::language::{Language, alphabet::Alphabet, statistics::LanguageStatistics};
use jevlor::morphem::MorphemType;
use serde::de::DeserializeOwned;

fn read_toml<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T, anyhow::Error> {
    let mut toml = String::new();
    File::options()
        .read(true)
        .open(path)?
        .read_to_string(&mut toml)?;

    Ok(toml::from_str(&toml)?)
}

fn main() {
    let alphabet: Alphabet = read_toml("lang/ru/alphabet.toml").unwrap();
    let mut lang: Language = read_toml("lang/ru/rules.toml").unwrap();

    lang.alphabet = alphabet;

    let stats = LanguageStatistics {
        rules: lang,
        vowels_per_morphem: [(MorphemType::Root, 3)].into_iter().collect(),
        consonants_per_morphem: [(MorphemType::Root, 5)].into_iter().collect(),
        lenghts_per_morphem: [(MorphemType::Root, 2u8..10u8)].into_iter().collect(),
    };

    let stat_gen =
        StatisticalMorphemGenerator::new(stats.morphem(&MorphemType::Root), MorphemType::Root);
    for morphem in stat_gen.take(10) {
        println!("{}", morphem);
    }
}
