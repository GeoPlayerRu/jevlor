use std::collections::HashMap;

use crate::morphem::MorphemType;

/// Describes relation between two morphems.
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
pub struct MorphemicRules {
    /// Rules of morhem concatenation. Describes relation between different morphem types, so its
    /// heteromorphic.
    pub concat_rules: HashMap<(MorphemType, MorphemType), MorphemConcatRule>,
}
