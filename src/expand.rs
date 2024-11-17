use std::collections::HashMap;

use crate::ast::{Expr, Mapping};

pub trait Expand {
    fn expand(&self, mappings: &HashMap<&'static str, Mapping>) -> String;
}

impl Expand for Expr {
    fn expand(&self, mappings: &HashMap<&'static str, Mapping>) -> String {
        match self {
            Expr::String(val) => val.to_string(),

            Expr::MappingApplication { name, args: _ } => {
                let mapping = mappings.get(name).expect("Mapping not found");
                if mapping.args.len() == 0 {
                    if let Expr::String(output) = mapping.translation {
                        output.replace(name, &mapping.translation.expand(mappings))
                    } else {
                        todo!()
                    }
                } else {
                    todo!();
                }
            }
        }
    }
}
