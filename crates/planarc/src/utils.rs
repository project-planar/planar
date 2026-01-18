use anyhow::anyhow;
use std::fmt::Display;

pub trait TypeSitterResultExt<T> {
    fn static_err(self) -> anyhow::Result<T>;

    fn static_context<C>(self, context: C) -> anyhow::Result<T>
    where
        C: Display + Send + Sync + 'static;
}

impl<T, E> TypeSitterResultExt<T> for Result<T, E>
where
    E: Display,
{
    fn static_err(self) -> anyhow::Result<T> {
        self.map_err(|e| anyhow!("{}", e))
    }

    fn static_context<C>(self, context: C) -> anyhow::Result<T>
    where
        C: Display + Send + Sync + 'static,
    {
        self.map_err(|e| anyhow!("{}: {}", context, e))
    }
}
