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
