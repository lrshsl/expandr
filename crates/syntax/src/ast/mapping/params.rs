use std::fmt;

use super::param::Param;

#[derive(Clone)]
pub struct Params {
    pub entries: Vec<Param>,
}

impl fmt::Debug for Params {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.entries)
    }
}
