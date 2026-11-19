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
/// Contains morphem struct and helper structures related to them.
pub mod morphem;

/// Contains word struct and helper structures related to them.
pub mod word;

/// Contains iterators that generate words with different information.
pub mod generator;

/// Contains structures that describe and abstract different languages for the proccess of
/// generation.
pub mod language;

/// Contains modules and structures that are used for deserializing [language] structures.
pub mod serde;
