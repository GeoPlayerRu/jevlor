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

use jevlor::generator::morphem::StatisticalMorphemGenerator;
use jevlor::morphem::MorphemType;
use jevlor::serde::load_language;

use std::path::Path;

// Separate function for easier migration
fn print_notice() {
    println!(
        "
    jevlor  Copyright (C) 2026  Fedor Veselov
    This program comes with ABSOLUTELY NO WARRANTY.
    This is free software, and you are welcome to redistribute it
    under certain conditions.
    "
    );
}

fn main() {
    print_notice();
    let args = std::env::args().collect::<Vec<String>>();
    let lookup_dir = String::from("lang/ru/");

    let working_directory = Path::new(args.get(1).unwrap_or(&lookup_dir));

    let lang = load_language(working_directory).unwrap();

    let stat_gen = StatisticalMorphemGenerator::new(
        lang.stats.morphem(&MorphemType::Root),
        MorphemType::Root,
        &lang,
    );
    for morphem in stat_gen.take(10) {
        println!("{}", morphem);
    }
}
