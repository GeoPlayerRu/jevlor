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
use std::collections::HashMap;

use serde::Serialize;

use crate::morphem::MorphemType;
use crate::serde::language::MorphemicRulesPrototype;

/// Describes relation between two morphems.
#[derive(Serialize, Clone, Debug)]
pub enum MorphemConcatRule {
    /// Concatenate them.
    Concat,
    /// Add substring 0 between them.
    Bridge(String),
    /// Remove the second morphem.
    Eliminate,
    // / Remove the first morphem.
    //ReverseEliminate,
}

/// Structure that contains rules to construct morphems
#[derive(Serialize, Clone, Debug)]
pub struct MorphemicRules {
    /// Rules of morhem concatenation. Describes relation between different morphem types, so its
    /// heteromorphic.
    pub concat_rules: HashMap<(MorphemType, MorphemType), Vec<MorphemConcatRule>>,
}

impl From<MorphemicRulesPrototype> for MorphemicRules {
    fn from(value: MorphemicRulesPrototype) -> Self {
        let mut result: Self = Self {
            concat_rules: HashMap::new(),
        };
        for rule in value.0 {
            result
                .concat_rules
                .entry((rule.left, rule.right))
                .or_default()
                .push(rule.rule.into());
        }

        result
    }
}
