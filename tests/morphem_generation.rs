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

/// Test that syllable generator does not fail and generates syllables correctly.
use jevlor::{generator::morphem::StatisticalMorphemGenerator, serde::load_language};
use rand::{SeedableRng, rngs::StdRng};

const TEST_SEED: u64 = 214151;

#[test]
fn test_singular_generation() {
    let lang = load_language("lang/ru").unwrap();
    let mut morphgen = StatisticalMorphemGenerator::create(
        &lang,
        StdRng::seed_from_u64(TEST_SEED),
        jevlor::morphem::MorphemType::Root,
    );
    let res = morphgen.next();
    assert!(res.map(|v| v.to_string()) == Some("у".to_string()));
}

/// Test that syllable generator correctly generates 100 syllables.
#[test]
fn test_multiple_generation() {}
