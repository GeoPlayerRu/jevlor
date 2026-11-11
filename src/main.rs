/*
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {}

fn main() {
    let args = Args::parse();
}
*/
use jevlor::generator::morphem::StatisticalMorphemGenerator;
use jevlor::language::{Language, alphabet::Alphabet, statistics::LanguageStatistics};
use jevlor::morphem::MorphemType;
use jevlor::serde::loader::FromTomlFile;

use std::path::Path;


fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let lookup_dir = String::from("lang/ru/");

    let working_directory = Path::new(args.get(1).unwrap_or(&lookup_dir));

    let alphabet: Alphabet = Alphabet::from_toml_file(working_directory.join("alphabet.toml")).unwrap();
    let mut lang: Language = Language::from_toml_file(working_directory.join("rules.toml")).unwrap();

    lang.alphabet = alphabet;

    let mut stats = LanguageStatistics::from_toml_file(working_directory.join("statistics.toml")).unwrap();
    stats.rules = Some(lang);

    let stat_gen =
        StatisticalMorphemGenerator::new(stats.morphem(&MorphemType::Root), MorphemType::Root);
    for morphem in stat_gen.take(10) {
        println!("{}", morphem);
    }
}
