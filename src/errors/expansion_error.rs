use std::fmt;

use crate::{
    ast::{Args, PathIdent},
    errors::pretty_print::print_raise_ctx,
    expand::Expanded,
    source_type::Owned,
};

pub type ExpansionResult = Result<Expanded, ExpansionError>;

#[derive(Debug, thiserror::Error)]
pub enum ExpansionError {
    UnknownMappingReferenced {
        msg: String,
        name: PathIdent,
        args: Args<Owned>,
        file: &'static str,
        line: u32,
    },
}

impl<'s> fmt::Display for ExpansionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpansionError::UnknownMappingReferenced {
                msg,
                name,
                args,
                file,
                line,
            } => {
                print_raise_ctx(f, file, *line)?;
                color_print::cwrite!(
                    f,
                    "\
| Mapping could not be resolved: <italic>{name:?} {args:#?}</>
| <red>{msg}</>
"
                )
            }
        }
    }
}

#[macro_export]
macro_rules! undefined_mapping {
    (
        $msg:expr, $name:expr, $args:expr
    ) => {
        Err(
            $crate::errors::expansion_error::ExpansionError::UnknownMappingReferenced {
                msg: $msg.to_string(),
                name: $name.clone(),
                args: $args.clone(),
                file: file!(),
                line: line!(),
            },
        )
    };
}
