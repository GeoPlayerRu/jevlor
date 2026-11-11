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
use crate::{
    language::{Alphabet, Language, MorphemicRules, morphem::MorphemConcatRule},
    morphem::MorphemType,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(into = "Language")]
pub(crate) struct LanguagePrototype {
    pub alphabet: Option<Alphabet>,
    #[serde(rename = "concat")]
    pub concat_rules: MorphemicRulesPrototype,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(into = "MorphemicRules")]
pub(crate) struct MorphemicRulesPrototype(pub Vec<ConcatRulePrototype>);

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase", into = "MorphemConcatRule")]
pub(crate) enum MorphemConcatRulePrototype {
    Concat,
    Bridge { value: String },
    Eliminate,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct ConcatRulePrototype {
    pub left: MorphemType,
    pub right: MorphemType,
    #[serde(flatten)]
    pub rule: MorphemConcatRulePrototype,
}

impl Into<MorphemConcatRule> for MorphemConcatRulePrototype {
    fn into(self) -> MorphemConcatRule {
        match self {
            MorphemConcatRulePrototype::Concat => MorphemConcatRule::Concat,
            MorphemConcatRulePrototype::Bridge { value } => MorphemConcatRule::Bridge(value),
            MorphemConcatRulePrototype::Eliminate => MorphemConcatRule::Eliminate,
        }
    }
}
