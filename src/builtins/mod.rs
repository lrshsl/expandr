use crate::{ast::Args, expand::Expanded};

type BuiltinFn = dyn Fn(&Args) -> Expanded;

pub fn get_builtin(name: &str) -> Option<Box<BuiltinFn>> {
    Some(Box::new(match name {
        "m" => builtins::evaluate_math,
        &_ => return None,
    }))
}

mod builtins {
    use crate::{
        ast::Args,
        expand::Expanded::{self, Int},
    };

    pub fn evaluate_math(args: &Args) -> Expanded {
        return Int(4);
    }
}
