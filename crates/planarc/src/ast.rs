use crate::spanned::{FileId, Spanned};

#[derive(Debug, Clone, Default)]
pub struct Module {
    pub file_id: FileId,
    pub imports: Vec<Spanned<Import>>,
    pub facts: Vec<Spanned<FactDefinition>>,
    pub externs: Vec<Spanned<ExternDefinition>>, 
    pub types: Vec<Spanned<TypeDeclaration>>,
    pub queries: Vec<Spanned<QueryDefinition>>,
}

#[derive(Debug, Clone)]
pub struct QueryDefinition {
    pub name: Spanned<String>,
    pub grammar: Spanned<String>,
    pub value: Spanned<String>,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct TypeDeclaration {
    pub attributes: Vec<Spanned<Attribute>>,
    pub name: Spanned<String>,
    pub definition: Spanned<TypeDefinition>,
}


#[derive(Debug, Clone)]
pub struct TypeDefinition {
    pub base_type: Option<TypeAnnotation>,
    pub fields: Vec<Spanned<TypeField>>,
}

#[derive(Debug, Clone)]
pub struct TypeField {
    pub name: Spanned<String>,
    pub definition: Spanned<TypeDefinition>,
}


#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    pub name: Spanned<String>,
    pub refinement: Option<Spanned<Expression>>,
    pub args: Vec<Spanned<TypeAnnotation>>,
}

#[derive(Debug, Clone)]
pub struct TypeArgument {
    pub ty: TypeAnnotation,
    pub refinement: Option<Spanned<Expression>>,
}


#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: Spanned<String>
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
        function: Box<Spanned<Expression>>,
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

#[derive(Debug, Clone)]
pub struct ExternDefinition {
    pub attributes: Vec<Spanned<Attribute>>,
    pub functions: Vec<Spanned<ExternFunction>>,
}

#[derive(Debug, Clone)]
pub struct ExternFunction {
    pub name: Spanned<String>,
    pub args: Vec<Spanned<ExternArgument>>,
    pub return_type: Option<Spanned<String>>,
}

#[derive(Debug, Clone)]
pub struct ExternArgument {
    pub name: Spanned<String>,
    pub ty: Spanned<String>,
}
