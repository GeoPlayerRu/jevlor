use std::collections::HashMap;

use crate::morphem::MorphemType;
use morphem::MorphemConcatRule;

/// Contains structures that descibe additional information about morphems in context of language.
pub mod morphem;

/// Structure that contains rules, alphabet of some language.
pub struct Language {
    /// Rules of morhem concatenation. Right now only describes one relation between morphems of same
    /// type, so it is homomorphic.
    pub concat_rules: HashMap<MorphemType, MorphemConcatRule>,
}
