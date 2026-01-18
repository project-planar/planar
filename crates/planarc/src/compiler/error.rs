use miette::Diagnostic;
use thiserror::Error;


#[derive(Debug, Error, Diagnostic)]
#[error("Compilation failed with {} errors", .0.len())]
pub struct CompilersError(
    #[related]
    pub Vec<miette::Report>
);

impl CompilersError {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push<E: Diagnostic + Send + Sync + 'static>(&mut self, err: E) {
        self.0.push(miette::Report::new(err));
    }

    pub fn extend<I, E>(&mut self, errors: I)
    where
        I: IntoIterator<Item = E>,
        E: Diagnostic + Send + Sync + 'static,
    {
        for err in errors {
            self.0.push(miette::Report::new(err));
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

