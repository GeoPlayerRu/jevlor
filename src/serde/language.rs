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
