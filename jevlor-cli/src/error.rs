#[derive(Clone, Debug)]
pub enum JevlorCliError {
    Generate(JevlorGenerateError),
}

#[derive(Clone, Debug)]
pub enum JevlorGenerateError {
    UnresolvedDefaultPath,
    InvalidStructure,
}

impl From<JevlorGenerateError> for JevlorCliError {
    fn from(value: JevlorGenerateError) -> Self {
        Self::Generate(value)
    }
}

pub trait JevlorError {
    fn jevlor_error(self) -> JevlorCliError;
}

impl<T: Into<JevlorCliError>> JevlorError for T {
    fn jevlor_error(self) -> JevlorCliError {
        self.into()
    }
}
