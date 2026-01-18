use miette::Diagnostic;
use thiserror::Error;
use std::fmt;

#[derive(Error, Diagnostic)]
#[error("Compilation failed with {} errors", .0.len())]
pub struct CompilersError(
    #[related]
    pub Vec<Box<dyn Diagnostic + Send + Sync + 'static>>
);

impl CompilersError {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push<E>(&mut self, error: E)
    where
        E: Diagnostic + Send + Sync + 'static,
    {
        self.0.push(Box::new(error));
    }

    pub fn extend<E>(&mut self, errors: Vec<E>)
    where
        E: Diagnostic + Send + Sync + 'static,
    {
        for err in errors {
            self.0.push(Box::new(err));
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Debug for CompilersError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&miette::Report::new(unsafe { std::ptr::read(self) }), f)
    }
}