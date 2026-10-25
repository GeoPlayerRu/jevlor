use std::collections::HashMap;

use crate::morphem::MorphemType;
use morphem::MorphemConcatRule;

pub mod morphem;

pub type ConcatRules = HashMap<MorphemType, MorphemConcatRule>;
