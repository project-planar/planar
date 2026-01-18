use std::fmt::Debug;
use derive_more::Display;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Archive, Serialize, Deserialize, Display)]
#[rkyv(derive(Debug))]
pub struct FileId(pub u32);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Archive, Serialize, Deserialize, Display)]
#[rkyv(derive(Debug))]
#[display("file id:{file_id} [{span}]")]
pub struct Location {
    pub file_id: FileId,
    pub span: Span,
}

impl Location {
    pub fn new(file_id: FileId, span: Span) -> Self {
        Self { file_id, span }
    }
}

#[derive(Clone, Archive, Serialize, Deserialize, PartialEq, Eq)]
pub struct Spanned<T> {
    pub value: T,
    pub loc: Location,
}

impl<T> Spanned<T> {
    pub fn new(value: T, loc: Location) -> Self {
        Self { value, loc }
    }
}

impl<T: Debug> Debug for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?} @ {:?}", self.value, self.loc.span)
        } else {
            write!(f, "{:?} @ {:?}", self.value, self.loc.span)
        }
    }
}

impl<T> std::fmt::Debug for ArchivedSpanned<T>
where
    T: rkyv::Archive,             
    T::Archived: std::fmt::Debug,    
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?} @ {:?}", self.value, self.loc.span)
        } else {
            write!(f, "{:?} @ {:?}", self.value, self.loc.span)
        }
    }
}



#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Archive, Serialize, Deserialize, Display)]
#[rkyv(derive(Debug))]
#[display("{start}..{end}")]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(range: std::ops::Range<usize>) -> Self {
        let start = range.start;
        let end = range.end;
        Self { start, end }
    }
}

impl From<Location> for miette::SourceSpan {
    fn from(loc: Location) -> Self {
        (loc.span.start..loc.span.end).into()
    }
}

impl From<std::ops::Range<usize>> for Span {
    fn from(value: std::ops::Range<usize>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}
