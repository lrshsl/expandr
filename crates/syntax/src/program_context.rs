use std::collections::HashMap;

use crate::{ast::mapping::Mapping, source_type::SourceType};

pub type ProgramContext<S> = HashMap<<S as SourceType>::Str, Vec<Mapping<S>>>;
