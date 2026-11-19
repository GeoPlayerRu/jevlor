#[derive(Clone, Debug)]
pub enum JevlorCliError {
    Generate(JevlorGenerateError),
}

#[derive(Clone, Debug)]
pub enum JevlorGenerateError {
    UnresolvedDefaultPath,
}
