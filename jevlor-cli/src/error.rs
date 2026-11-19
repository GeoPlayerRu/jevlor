#[derive(Clone, Debug)]
pub enum JevlorCliError {
    Load(JevlorLoadError),
}

#[derive(Clone, Debug)]
pub enum JevlorLoadError {
    UnresolvedPath,
    InvalidStructure,
}

impl From<JevlorLoadError> for JevlorCliError {
    fn from(value: JevlorLoadError) -> Self {
        Self::Load(value)
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
