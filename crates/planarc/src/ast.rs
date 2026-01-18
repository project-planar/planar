use crate::spanned::{FileId, Spanned};

#[derive(Debug, Clone, Default)]
pub struct Module {
    pub file_id: FileId,
    pub imports: Vec<Spanned<Import>>,
    pub facts: Vec<Spanned<FactDefinition>>,
    pub types: Vec<Spanned<TypeDeclaration>>,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct TypeDeclaration {
    pub name: Spanned<String>,
    pub ty: TypeAnnotation,
    pub refinement: Option<Spanned<Expression>>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: Spanned<String>,
    pub args: Vec<Spanned<Expression>>,
}

#[derive(Debug, Clone)]
pub struct FactDefinition {
    pub attributes: Vec<Spanned<Attribute>>,
    pub name: Spanned<String>,
    pub fields: Vec<Spanned<FactField>>,
}

#[derive(Debug, Clone)]
pub struct FactField {
    pub attributes: Vec<Spanned<Attribute>>,
    pub name: Spanned<String>,
    pub ty: TypeAnnotation,
    pub refinement: Option<Spanned<Expression>>,
}

#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    pub name: Spanned<String>,
    pub args: Vec<Spanned<TypeArgument>>,
    pub generic_var: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TypeArgument {
    pub ty: TypeAnnotation,
    pub refinement: Option<Spanned<Expression>>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Number(String),
    StringLit(String),

    Binary {
        left: Box<Spanned<Expression>>,
        op: String,
        right: Box<Spanned<Expression>>,
    },

    Call {
        function: String,
        args: Vec<Spanned<Expression>>,
    },

    InList(Vec<Spanned<Expression>>),
    InRange {
        start: Box<Spanned<Expression>>,
        end: Option<Box<Spanned<Expression>>>,
    },

    PartialComparison {
        op: String,
        right: Box<Spanned<Expression>>,
    },
}
