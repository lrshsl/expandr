use std::collections::HashMap;

#[derive(Debug)]
pub struct Ast {
    pub mappings: HashMap<&'static str, Mapping>,
    pub exprs: Vec<Expr>,
}

#[derive(Debug)]
pub struct Mapping {
    pub args: Vec<MappingParam>,
    pub translation: Expr,
}

#[derive(Debug)]
pub enum MappingParam {
    Ident(&'static str),
    Expr(ParamExpr),
}

#[derive(Debug)]
pub struct ParamExpr {
    expr: Expr,
    number_repetitions: Repetition,
}

#[derive(Debug)]
pub enum Repetition {
    Exactly(usize),
    Optional,
    Any,
}

#[derive(Debug)]
pub enum Expr {
    String(&'static str),
    MappingApplication { name: &'static str, args: Vec<Expr> },
}
