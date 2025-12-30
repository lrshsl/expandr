use std::fmt;

use expandr_syntax::{
    ast::{mapping::Args, PathIdent},
    errors::pretty_print::print_raise_ctx,
    source_type::Owned,
};

use crate::expand::Expanded;

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

impl fmt::Display for ExpansionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pretty_print(f, false)
    }
}

impl ExpansionError {
    pub fn pretty_print(&self, f: &mut impl fmt::Write, color_codes: bool) -> fmt::Result {
        match self {
            ExpansionError::UnknownMappingReferenced {
                msg,
                name,
                args,
                file,
                line,
            } => {
                print_raise_ctx(f, file, *line, color_codes)?;

                if color_codes {
                    color_print::cwrite!(
                        f,
                        "\
| Mapping could not be resolved: <italic>{name:?} {args:#?}</>
| <red>{msg}</>
"
                    )
                } else {
                    write!(
                        f,
                        "\
| Mapping could not be resolved: {name:?} {args:#?}
| {msg}
"
                    )
                }
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
            $crate::expansion_error::ExpansionError::UnknownMappingReferenced {
                msg: $msg.to_string(),
                name: $name.clone(),
                args: $args.clone(),
                file: file!(),
                line: line!(),
            },
        )
    };
}
