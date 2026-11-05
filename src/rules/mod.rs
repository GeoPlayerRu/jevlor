use std::collections::HashMap;

use crate::morphem::MorphemType;
use morphem::MorphemConcatRule;

/// Contains structures that descibe additional information about morphems in context of language.
pub mod morphem;

/// Structure that contains rules, alphabet of some language.
pub struct Language {
    /// Rules of morhem concatenation. Describes relation between different morphem types, so its
    /// heteromorphic.
    pub concat_rules: HashMap<(MorphemType, MorphemType), MorphemConcatRule>,
}
