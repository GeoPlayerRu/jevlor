use std::collections::HashMap;

use serde::Serialize;

use crate::morphem::MorphemType;
use crate::serde::language::MorphemicRulesPrototype;

/// Describes relation between two morphems.
#[derive(Serialize)]
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
#[derive(Serialize)]
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
