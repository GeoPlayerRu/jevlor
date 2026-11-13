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
use jevlor::{generator::syllable::SyllableGenerator, serde::load_language};
use rand::{SeedableRng, rngs::StdRng};

const TEST_SEED: u64 = 214151;

/// Test that syllable generator does not fail and generates syllables correctly.
#[test]
fn test_singular_generation() {
    let lang = load_language("lang/ru").unwrap();
    let mut seeded_rng = StdRng::seed_from_u64(TEST_SEED);
    let morphem_slice = lang.stats.morphem(&jevlor::morphem::MorphemType::Root);
    let mut sylgen = SyllableGenerator::new(&morphem_slice, &mut seeded_rng, &lang);
    let res = sylgen.next();
    assert!(res == Some("у".to_string()));
}

/// Test that syllable generator correctly generates 100 syllables.
#[test]
fn test_multiple_generation() {
    let lang = load_language("lang/ru").unwrap();

    let mut seeded_rng = StdRng::seed_from_u64(TEST_SEED);
    let morphem_slice = lang.stats.morphem(&jevlor::morphem::MorphemType::Root);
    let sylgen = SyllableGenerator::new(&morphem_slice, &mut seeded_rng, &lang);

    let multiple_syllables = vec![
        "у".to_string(),
        "шесчч".to_string(),
        "сян".to_string(),
        "ё".to_string(),
        "рышж".to_string(),
        "сёлщ".to_string(),
        "фи".to_string(),
        "яжп".to_string(),
        "у".to_string(),
        "а".to_string(),
        "фтонщ".to_string(),
        "гё".to_string(),
        "оз".to_string(),
        "шэзфн".to_string(),
        "и".to_string(),
        "и".to_string(),
        "зы".to_string(),
        "ойк".to_string(),
        "яф".to_string(),
        "ю".to_string(),
        "а".to_string(),
        "е".to_string(),
        "цяр".to_string(),
        "ибн".to_string(),
        "йвящ".to_string(),
        "бю".to_string(),
        "нёщд".to_string(),
        "имбф".to_string(),
        "дю".to_string(),
        "ыфш".to_string(),
        "юкф".to_string(),
        "ючп".to_string(),
        "яз".to_string(),
        "фы".to_string(),
        "чуф".to_string(),
        "иш".to_string(),
        "со".to_string(),
        "эр".to_string(),
        "сэж".to_string(),
        "ыкз".to_string(),
        "цы".to_string(),
        "ё".to_string(),
        "и".to_string(),
        "пяс".to_string(),
        "ер".to_string(),
        "цы".to_string(),
        "фыш".to_string(),
        "гсоф".to_string(),
        "кжюз".to_string(),
        "йягк".to_string(),
        "бюн".to_string(),
        "у".to_string(),
        "шэ".to_string(),
        "бфэ".to_string(),
        "ё".to_string(),
        "нвя".to_string(),
        "ю".to_string(),
        "мах".to_string(),
        "жё".to_string(),
        "ё".to_string(),
        "ух".to_string(),
        "аф".to_string(),
        "о".to_string(),
        "я".to_string(),
        "щаб".to_string(),
        "кит".to_string(),
        "кя".to_string(),
        "э".to_string(),
        "ы".to_string(),
        "яцл".to_string(),
        "хях".to_string(),
        "жфюфф".to_string(),
        "тпи".to_string(),
        "ше".to_string(),
        "е".to_string(),
        "мё".to_string(),
        "эйт".to_string(),
        "ы".to_string(),
        "ы".to_string(),
        "лу".to_string(),
        "що".to_string(),
        "сацд".to_string(),
        "лёп".to_string(),
        "зя".to_string(),
        "нйа".to_string(),
        "ы".to_string(),
        "ай".to_string(),
        "игшп".to_string(),
        "чыф".to_string(),
        "бя".to_string(),
        "йы".to_string(),
        "мё".to_string(),
        "кю".to_string(),
        "ёл".to_string(),
        "ся".to_string(),
        "счижт".to_string(),
        "щпа".to_string(),
        "ма".to_string(),
        "ошб".to_string(),
        "у".to_string(),
    ];

    for syl in sylgen.take(100).zip(multiple_syllables.into_iter()) {
        assert_eq!(syl.0, syl.1);
    }
}
