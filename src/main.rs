/*
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {}

fn main() {
    let args = Args::parse();
}
*/

use jevlor::morphem::MorphemType;
use jevlor::rules::language::{Alphabet, Language, LanguageStatistics, ULPair};
use jevlor::generator::morphem::StatisticalMorphemGenerator;
use jevlor::rules::morphem::{MorphemConcatRule, MorphemicRules};

fn main() {
    let letters = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя".chars().map(|ch| ULPair::new(ch.to_uppercase().next().unwrap(), ch)).collect();
    let vowels = "аеёиоуыэюя".chars().map(|ch| ULPair::new(ch.to_uppercase().next().unwrap(), ch)).collect();
    let consonants = "бвгджзйклмнпрстфхцчшщъь".chars().map(|ch| ULPair::new(ch.to_uppercase().next().unwrap(), ch)).collect();

    let mut alphabet = Alphabet::new(letters);
    alphabet.define_vowels(vowels);
    alphabet.define_consonants(consonants);

    let morphem_rules = MorphemicRules {
        concat_rules: [((MorphemType::Root, MorphemType::Root), MorphemConcatRule::Bridge("е".to_string()))].into_iter().collect(),
    };

    let lang = Language {
        alphabet,
        morphem_rules,
    };

    let stats = LanguageStatistics {
        rules: lang,
        vowels_per_morphem: [(MorphemType::Root, 3)].into_iter().collect(),
        consonants_per_morphem: [(MorphemType::Root, 5)].into_iter().collect(),
        lenghts_per_morphem: [(MorphemType::Root, 2u8..10u8)].into_iter().collect(),
    };

    let stat_gen = StatisticalMorphemGenerator::new(stats.morphem(&MorphemType::Root), MorphemType::Root);
    for morphem in stat_gen.take(10) {
        println!("{}", morphem);
    }
}
